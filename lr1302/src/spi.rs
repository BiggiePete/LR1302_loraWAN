//! SPI transport for the SX1302 concentrator and its SX1250 radios.
//!
//! This mirrors the C HAL layers `loragw_spi.c` (register/memory access) and
//! `sx1250_spi.c` (radio opcode frames) on top of an
//! [`embedded_hal_async::spi::SpiDevice`], so any embassy-compatible SPI
//! driver works.
//!
//! # Frame formats
//!
//! Every frame starts with a *mux target* byte selecting the device behind
//! the concentrator's SPI router ([`MUX_SX1302`], [`MUX_RADIO_A`],
//! [`MUX_RADIO_B`]).
//!
//! * Register write: `[mux, 0x80 | addr[14:8], addr[7:0], data ...]`
//! * Register read:  `[mux, 0x00 | addr[14:8], addr[7:0], 0x00]` followed by
//!   `n` clocks that clock the data in
//! * Radio opcode write: `[mux, opcode, data ...]`
//! * Radio opcode read:  `[mux, opcode, data ...]` (full duplex, the
//!   outgoing `data` bytes are the ones passed by the caller, as in the C
//!   HAL)
//!
//! The bus must be configured in SPI mode 0, MSB first, at most 2 MHz
//! (the C HAL uses `SPI_SPEED`, see `loragw_spi.h`).

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::{Operation, SpiDevice};

use crate::error::{Error, Result};
use crate::log::trace;
use crate::reg::{LOREGS, LGW_TOTALREGS};

/// Mux target: the SX1302 itself (C: `LGW_SPI_MUX_TARGET_SX1302`).
pub const MUX_SX1302: u8 = 0x00;
/// Mux target: radio A / RF chain 0 (C: `LGW_SPI_MUX_TARGET_RADIOA`).
pub const MUX_RADIO_A: u8 = 0x01;
/// Mux target: radio B / RF chain 1 (C: `LGW_SPI_MUX_TARGET_RADIOB`).
pub const MUX_RADIO_B: u8 = 0x02;

const READ_ACCESS: u8 = 0x00;
const WRITE_ACCESS: u8 = 0x80;

/// Maximum payload of a single SPI transfer (C: `LGW_BURST_CHUNK`).
pub const BURST_CHUNK: usize = 1024;

/// Maximum data length accepted by the radio opcode helpers.
pub const RADIO_DATA_MAX: usize = 32;

/// Mux target of a given RF chain (0 -> A, 1 -> B).
pub const fn mux_for_chain(rf_chain: u8) -> u8 {
    if rf_chain == 0 {
        MUX_RADIO_A
    } else {
        MUX_RADIO_B
    }
}

/// Raw SPI transport shared by all layers of the driver.
pub struct Com<SPI, D> {
    pub(crate) spi: SPI,
    pub(crate) delay: D,
}

impl<SPI, D> Com<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    pub fn new(spi: SPI, delay: D) -> Self {
        Self { spi, delay }
    }

    /// Millisecond wait (C: `wait_ms`).
    pub async fn wait_ms(&mut self, ms: u32) {
        self.delay.delay_ms(ms).await;
    }

    /// Microsecond wait (C: `wait_us`).
    pub async fn wait_us(&mut self, us: u32) {
        self.delay.delay_us(us).await;
    }

    fn map_err<E>(_: E) -> Error {
        Error::Spi
    }

    // -----------------------------------------------------------------------
    // raw frames (C: lgw_spi_w / lgw_spi_r / sx1250_spi_w / sx1250_spi_r)
    // -----------------------------------------------------------------------

    /// Write `data` at `addr` of the device selected by `mux`, chunked at
    /// [`BURST_CHUNK`] bytes like the C HAL.
    async fn frame_write(&mut self, mux: u8, addr: u16, data: &[u8]) -> Result<()> {
        trace!(
            "spi write mux={=u8} addr={=u16} len={=u32}",
            mux,
            addr,
            data.len() as u32
        );
        let header = [
            mux,
            WRITE_ACCESS | ((addr >> 8) as u8 & 0x7F),
            (addr & 0xFF) as u8,
        ];
        let mut offset = 0usize;
        while offset < data.len() {
            let chunk = core::cmp::min(BURST_CHUNK, data.len() - offset);
            self.spi
                .transaction(&mut [
                    Operation::Write(&header),
                    Operation::Write(&data[offset..offset + chunk]),
                ])
                .await
                .map_err(Self::map_err)?;
            // the address auto-increments inside the chip between chunks
            offset += chunk;
        }
        Ok(())
    }

    /// Read `out.len()` bytes at `addr` of the device selected by `mux`.
    async fn frame_read(&mut self, mux: u8, addr: u16, out: &mut [u8]) -> Result<()> {
        trace!(
            "spi read mux={=u8} addr={=u16} len={=u32}",
            mux,
            addr,
            out.len() as u32
        );
        let header = [
            mux,
            READ_ACCESS | ((addr >> 8) as u8 & 0x7F),
            (addr & 0xFF) as u8,
            0x00,
        ];
        let mut offset = 0usize;
        while offset < out.len() {
            let chunk = core::cmp::min(BURST_CHUNK, out.len() - offset);
            let target = &mut out[offset..offset + chunk];
            self.spi
                .transaction(&mut [Operation::Write(&header), Operation::Read(target)])
                .await
                .map_err(Self::map_err)?;
            offset += chunk;
        }
        Ok(())
    }

    /// Single byte register write (C: `lgw_spi_w`).
    pub async fn reg_byte_w(&mut self, mux: u8, addr: u16, data: u8) -> Result<()> {
        self.frame_write(mux, addr, core::slice::from_ref(&data)).await
    }

    /// Single byte register read (C: `lgw_spi_r`).
    pub async fn reg_byte_r(&mut self, mux: u8, addr: u16) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.frame_read(mux, addr, &mut buf).await?;
        Ok(buf[0])
    }

    /// Read-modify-write of a bit field (C: `lgw_spi_rmw`).
    pub async fn reg_rmw(
        &mut self,
        mux: u8,
        addr: u16,
        offs: u8,
        leng: u8,
        data: u8,
    ) -> Result<()> {
        let old = self.reg_byte_r(mux, addr).await?;
        let mask = (((1u16 << leng) - 1) << offs) as u8;
        let new = (!mask & old) | (mask & (data << offs));
        self.reg_byte_w(mux, addr, new).await
    }

    // -----------------------------------------------------------------------
    // register access by name (C: lgw_reg_w / lgw_reg_r / lgw_reg_wb / rb)
    // -----------------------------------------------------------------------

    fn desc(id: u16) -> Result<&'static crate::reg::RegDesc> {
        if id >= LGW_TOTALREGS {
            return Err(Error::InvalidConfig("register id out of range"));
        }
        Ok(&LOREGS[id as usize])
    }

    /// Write a named register (C: `lgw_reg_w`).
    pub async fn reg_w(&mut self, id: u16, value: i32) -> Result<()> {
        let r = Self::desc(id)?;
        if r.is_read_only() {
            return Err(Error::Unsupported);
        }
        self.reg_w_raw(MUX_SX1302, r, value).await
    }

    /// Read a named register (C: `lgw_reg_r`).
    pub async fn reg_r(&mut self, id: u16) -> Result<i32> {
        let r = Self::desc(id)?;
        self.reg_r_raw(MUX_SX1302, r).await
    }

    /// Burst write starting at the address of a named register
    /// (C: `lgw_reg_wb`).
    pub async fn reg_wb(&mut self, id: u16, data: &[u8]) -> Result<()> {
        let r = Self::desc(id)?;
        if r.is_read_only() {
            return Err(Error::Unsupported);
        }
        self.frame_write(MUX_SX1302, r.addr, data).await
    }

    /// Burst read starting at the address of a named register
    /// (C: `lgw_reg_rb`).
    pub async fn reg_rb(&mut self, id: u16, out: &mut [u8]) -> Result<()> {
        let r = Self::desc(id)?;
        self.frame_read(MUX_SX1302, r.addr, out).await
    }

    /// Descriptor of a named register.
    pub fn reg_desc(id: u16) -> Result<&'static crate::reg::RegDesc> {
        Self::desc(id)
    }

    /// Write using a register descriptor (C: private `reg_w()`).
    pub(crate) async fn reg_w_raw(
        &mut self,
        mux: u8,
        r: &crate::reg::RegDesc,
        value: i32,
    ) -> Result<()> {
        if r.leng == 8 && r.offs == 0 {
            self.reg_byte_w(mux, r.addr, value as u8).await
        } else if (r.offs as u32 + r.leng as u32) <= 8 {
            self.reg_rmw(mux, r.addr, r.offs, r.leng, value as u8).await
        } else {
            Err(Error::Unsupported)
        }
    }

    /// Read using a register descriptor (C: private `reg_r()`).
    pub(crate) async fn reg_r_raw(
        &mut self,
        mux: u8,
        r: &crate::reg::RegDesc,
    ) -> Result<i32> {
        if (r.offs as u32 + r.leng as u32) > 8 {
            return Err(Error::Unsupported);
        }
        let raw = self.reg_byte_r(mux, r.addr).await?;
        // left-align the field inside the byte, then right-align it again,
        // sign-extending if the field is signed.
        let left = raw << (8 - r.leng - r.offs);
        let value = if r.is_signed() {
            ((left as i8) >> (8 - r.leng)) as i32
        } else {
            (left >> (8 - r.leng)) as i32
        };
        Ok(value)
    }

    // -----------------------------------------------------------------------
    // memory access (C: lgw_mem_wb / lgw_mem_rb)
    // -----------------------------------------------------------------------

    /// Burst write at a raw memory address (C: `lgw_mem_wb`).
    pub async fn mem_wb(&mut self, addr: u16, data: &[u8]) -> Result<()> {
        if data.is_empty() {
            return Err(Error::InvalidConfig("zero-length burst"));
        }
        self.frame_write(MUX_SX1302, addr, data).await
    }

    /// Burst read at a raw memory address (C: `lgw_mem_rb`).
    ///
    /// `fifo_mode` selects a target whose read pointer auto-increments
    /// (like the RX buffer), in which case the address is not incremented
    /// between chunks.
    pub async fn mem_rb(&mut self, addr: u16, out: &mut [u8], fifo_mode: bool) -> Result<()> {
        if out.is_empty() {
            return Err(Error::InvalidConfig("zero-length burst"));
        }
        if fifo_mode {
            // single-shot read at a fixed address, still chunked to bound
            // the driver's internal buffers
            let mut offset = 0usize;
            while offset < out.len() {
                let chunk = core::cmp::min(BURST_CHUNK, out.len() - offset);
                let target = &mut out[offset..offset + chunk];
                self.spi
                    .transaction(&mut [
                        Operation::Write(&[
                            MUX_SX1302,
                            READ_ACCESS | ((addr >> 8) as u8 & 0x7F),
                            (addr & 0xFF) as u8,
                            0x00,
                        ]),
                        Operation::Read(target),
                    ])
                    .await
                    .map_err(Self::map_err)?;
                offset += chunk;
            }
            Ok(())
        } else {
            self.frame_read(MUX_SX1302, addr, out).await
        }
    }

    // -----------------------------------------------------------------------
    // radio opcode frames (C: sx1250_spi_w / sx1250_spi_r)
    // -----------------------------------------------------------------------

    /// Send an opcode frame to an SX1250 radio (C: `sx1250_spi_w`).
    pub async fn radio_w(&mut self, rf_chain: u8, opcode: u8, data: &[u8]) -> Result<()> {
        if rf_chain >= 2 {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        // C: wait 1 ms for the radio BUSY line before every frame
        self.wait_ms(1).await;
        trace!(
            "radio write chain={=u8} opcode={=u8} len={=u32}",
            rf_chain,
            opcode,
            data.len() as u32
        );
        if data.is_empty() {
            self.spi
                .write(&[mux_for_chain(rf_chain), opcode])
                .await
                .map_err(Self::map_err)
        } else {
            self.spi
                .transaction(&mut [
                    Operation::Write(&[mux_for_chain(rf_chain), opcode]),
                    Operation::Write(data),
                ])
                .await
                .map_err(Self::map_err)
        }
    }

    /// Receive `out.len()` bytes for an opcode frame from an SX1250 radio
    /// (C: `sx1250_spi_r`).
    ///
    /// As in the C HAL, the current contents of `out` are clocked out first
    /// (used by `READ_REGISTER` to send the address), and are then replaced
    /// by the bytes read from the radio.
    pub async fn radio_r(&mut self, rf_chain: u8, opcode: u8, out: &mut [u8]) -> Result<()> {
        if rf_chain >= 2 {
            return Err(Error::InvalidConfig("rf_chain"));
        }
        if out.len() > RADIO_DATA_MAX {
            return Err(Error::InvalidConfig("radio frame too long"));
        }
        self.wait_ms(1).await;
        trace!(
            "radio read chain={=u8} opcode={=u8} len={=u32}",
            rf_chain,
            opcode,
            out.len() as u32
        );
        self.spi
            .transaction(&mut [
                Operation::Write(&[mux_for_chain(rf_chain), opcode]),
                Operation::TransferInPlace(out),
            ])
            .await
            .map_err(Self::map_err)
    }
}
