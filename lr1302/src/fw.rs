//! MCU firmware upload and mailbox protocol
//! (C: `loragw_sx1302.c`, `sx1302_agc_*` / `sx1302_arb_*`).
//!
//! The SX1302 embeds two 8 KiB MCU firmware images: the AGC (automatic gain
//! control) firmware and the arbiter firmware. This module uploads them into
//! the MCU memories, verifies them by read-back, and then drives the
//! mailbox / status handshake that runs the firmware.
//!
//! Not ported (debug-only in the C HAL): `sx1302_arb_get_debug_stats_detect`,
//! `sx1302_arb_get_debug_stats_alloc`, `sx1302_arb_print_debug_stats`, and
//! `sx1302_arb_set_debug_stats` (whose single register write is inlined at its
//! one call site in `sx1302_arb_start`).

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::consts::{RadioType, DR_LORA_SF7};
use crate::error::{Error, Result};
use crate::reg::{
    SX1302_REG_AGC_MCU_CTRL_HOST_PROG, SX1302_REG_AGC_MCU_CTRL_MCU_CLEAR,
    SX1302_REG_AGC_MCU_CTRL_PARITY_ERROR, SX1302_REG_AGC_MCU_MCU_AGC_STATUS_MCU_AGC_STATUS,
    SX1302_REG_AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE0_MCU_MAIL_BOX_RD_DATA,
    SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE0_MCU_MAIL_BOX_WR_DATA,
    SX1302_REG_ARB_MCU_ARB_DEBUG_CFG_0_ARB_DEBUG_CFG_0,
    SX1302_REG_ARB_MCU_ARB_DEBUG_STS_0_ARB_DEBUG_STS_0, SX1302_REG_ARB_MCU_CTRL_HOST_PROG,
    SX1302_REG_ARB_MCU_CTRL_MCU_CLEAR, SX1302_REG_ARB_MCU_CTRL_PARITY_ERROR,
    SX1302_REG_ARB_MCU_MCU_ARB_STATUS_MCU_ARB_STATUS, SX1302_REG_COMMON_PAGE_PAGE,
};
use crate::sx1302_defs::{
    AGC_MEM_ADDR, AGC_PARAMS_SX1250, AGC_RADIO_A_INIT_DONE, AGC_RADIO_B_INIT_DONE, ARB_MEM_ADDR,
    MCU_FW_SIZE,
};
use crate::types::FtimeMode;
use crate::Lr1302;

/// Maximum number of polls in `sx1302_agc_wait_status` /
/// `sx1302_arb_wait_status` before giving up.
///
/// The C loops poll forever (`/* TODO: add timeout */`); this port bounds
/// them so a hung MCU reports [`Error::Timeout`] instead of blocking the
/// caller indefinitely. Every poll is a full SPI register read, so the bound
/// covers several seconds of polling.
const STATUS_POLL_MAX_RETRIES: u32 = 100_000;

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    // -----------------------------------------------------------------------
    // AGC MCU
    // -----------------------------------------------------------------------

    /// Upload the AGC firmware image and verify it by read-back
    /// (C: `sx1302_agc_load_firmware`).
    ///
    /// Only the first [`MCU_FW_SIZE`] bytes of `firmware` are transferred,
    /// as in the C function.
    pub(crate) async fn sx1302_agc_load_firmware(&mut self, firmware: &[u8]) -> Result<()> {
        if firmware.len() < MCU_FW_SIZE {
            return Err(Error::InvalidConfig("firmware size"));
        }
        let image = &firmware[..MCU_FW_SIZE];

        // Take control over AGC MCU
        self.com
            .reg_w(SX1302_REG_AGC_MCU_CTRL_MCU_CLEAR, 0x01)
            .await?;
        self.com
            .reg_w(SX1302_REG_AGC_MCU_CTRL_HOST_PROG, 0x01)
            .await?;
        self.com.reg_w(SX1302_REG_COMMON_PAGE_PAGE, 0x00).await?;

        // Write AGC fw in AGC MEM
        self.com.mem_wb(AGC_MEM_ADDR, image).await?;

        // Read back and check
        let mut fw_check = [0u8; MCU_FW_SIZE];
        self.com.mem_rb(AGC_MEM_ADDR, &mut fw_check, false).await?;
        if image != &fw_check[..] {
            return Err(Error::FirmwareVerify);
        }

        // Release control over AGC MCU
        self.com
            .reg_w(SX1302_REG_AGC_MCU_CTRL_HOST_PROG, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_AGC_MCU_CTRL_MCU_CLEAR, 0x00)
            .await?;

        let val: i32 = self.com.reg_r(SX1302_REG_AGC_MCU_CTRL_PARITY_ERROR).await?;
        if val != 0 {
            // C: "Failed to load AGC fw: parity error check failed"
            return Err(Error::FirmwareVerify);
        }

        Ok(())
    }

    /// Read the AGC MCU status register (C: `sx1302_agc_status`).
    pub(crate) async fn sx1302_agc_status(&mut self) -> Result<u8> {
        let val: i32 = self
            .com
            .reg_r(SX1302_REG_AGC_MCU_MCU_AGC_STATUS_MCU_AGC_STATUS)
            .await?;
        Ok(val as u8)
    }

    /// Poll until the AGC MCU reports `status` (C: `sx1302_agc_wait_status`).
    ///
    /// The C loop has no timeout (`/* TODO: add timeout */`); this port
    /// returns [`Error::Timeout`] after [`STATUS_POLL_MAX_RETRIES`] polls.
    pub(crate) async fn sx1302_agc_wait_status(&mut self, status: u8) -> Result<()> {
        let mut retries: u32 = 0;
        loop {
            let val = self.sx1302_agc_status().await?;
            if val == status {
                return Ok(());
            }
            retries += 1;
            if retries >= STATUS_POLL_MAX_RETRIES {
                return Err(Error::Timeout);
            }
        }
    }

    /// Read one AGC mailbox register (C: `sx1302_agc_mailbox_read`).
    pub(crate) async fn sx1302_agc_mailbox_read(&mut self, mailbox: u8) -> Result<u8> {
        // Check parameters
        if mailbox > 3 {
            return Err(Error::InvalidConfig("mailbox"));
        }

        let reg =
            SX1302_REG_AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE0_MCU_MAIL_BOX_RD_DATA - mailbox as u16;
        let val: i32 = self.com.reg_r(reg).await?;
        Ok(val as u8)
    }

    /// Write one AGC mailbox register (C: `sx1302_agc_mailbox_write`).
    pub(crate) async fn sx1302_agc_mailbox_write(&mut self, mailbox: u8, value: u8) -> Result<()> {
        // Check parameters
        if mailbox > 3 {
            return Err(Error::InvalidConfig("mailbox"));
        }

        let reg =
            SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE0_MCU_MAIL_BOX_WR_DATA - mailbox as u16;
        self.com.reg_w(reg, value as i32).await
    }

    /// Read AGC mailbox `mailbox` and check that it echoes `expected`
    /// (C: the `sx1302_agc_mailbox_read` + `if (val != ...)` checks of
    /// `sx1302_agc_start`).
    ///
    /// `field` is the subject of the C error message, reported as
    /// [`Error::InvalidConfig`] on mismatch.
    async fn agc_mailbox_expect(
        &mut self,
        mailbox: u8,
        expected: u8,
        field: &'static str,
    ) -> Result<()> {
        let val = self.sx1302_agc_mailbox_read(mailbox).await?;
        if val != expected {
            return Err(Error::InvalidConfig(field));
        }
        Ok(())
    }

    /// Run the AGC firmware handshake: version check, radio gain setup and
    /// AGC parameter upload (C: `sx1302_agc_start`).
    ///
    /// Only [`RadioType::Sx1250`] is supported: the C `agc_params_sx125x`
    /// table of the SX1255/1257 paths is not ported.
    pub(crate) async fn sx1302_agc_start(
        &mut self,
        version: u8,
        radio_type: RadioType,
        ana_gain: u8,
        dec_gain: u8,
        full_duplex: bool,
        lbt_enable: bool,
    ) -> Result<()> {
        // Check parameters
        if radio_type != RadioType::Sx1250 {
            // C also accepts LGW_RADIO_TYPE_SX1255 / LGW_RADIO_TYPE_SX1257
            return Err(Error::Unsupported);
        }
        let fdd_mode: u8 = if full_duplex { 1 } else { 0 };
        let agc_params = AGC_PARAMS_SX1250;

        // Wait for AGC fw to be started, and VERSION available in mailbox
        self.sx1302_agc_wait_status(0x01).await?; /* fw has started, VERSION is ready in mailbox */

        let val = self.sx1302_agc_mailbox_read(0).await?;
        if val != version {
            // C: "wrong AGC fw version"
            return Err(Error::FirmwareVerify);
        }

        // -------------------------------------------------------------------
        // Configure Radio A gains
        self.sx1302_agc_mailbox_write(0, ana_gain).await?; /* 0:auto agc */
        self.sx1302_agc_mailbox_write(1, dec_gain).await?;
        // C writes fdd_mode to mailbox 2 only for sx1255/sx1257 radios

        // notify AGC that gains has been set to mailbox for Radio A
        self.sx1302_agc_mailbox_write(3, AGC_RADIO_A_INIT_DONE)
            .await?;

        // Wait for AGC to acknoledge it has received gain settings for Radio A
        self.sx1302_agc_wait_status(0x02).await?;

        // Check ana_gain / dec_gain / FDD mode settings
        self.agc_mailbox_expect(0, ana_gain, "ana_gain").await?;
        self.agc_mailbox_expect(1, dec_gain, "dec_gain").await?;
        self.agc_mailbox_expect(2, fdd_mode, "fdd_mode").await?;

        // -------------------------------------------------------------------
        // Configure Radio B gains
        self.sx1302_agc_mailbox_write(0, ana_gain).await?; /* 0:auto agc */
        self.sx1302_agc_mailbox_write(1, dec_gain).await?;
        // C writes fdd_mode to mailbox 2 only for sx1255/sx1257 radios

        // notify AGC that gains has been set to mailbox for Radio B
        self.sx1302_agc_mailbox_write(3, AGC_RADIO_B_INIT_DONE)
            .await?;

        // Wait for AGC to acknoledge it has received gain settings for Radio B
        self.sx1302_agc_wait_status(0x03).await?;

        // Check ana_gain / dec_gain / FDD mode settings
        self.agc_mailbox_expect(0, ana_gain, "ana_gain").await?;
        self.agc_mailbox_expect(1, dec_gain, "dec_gain").await?;
        self.agc_mailbox_expect(2, fdd_mode, "fdd_mode").await?;

        // -------------------------------------------------------------------
        // Configure AGC gains (C: `agc_params_sx1250`, selected because
        // radio_type is Sx1250)

        // Configure analog gain min/max
        self.sx1302_agc_mailbox_write(0, agc_params.ana_min).await?;
        self.sx1302_agc_mailbox_write(1, agc_params.ana_max).await?;
        // notify AGC that params have been set to mailbox
        self.sx1302_agc_mailbox_write(3, 0x03).await?;
        // Wait for AGC to acknoledge it has received params
        self.sx1302_agc_wait_status(0x04).await?;
        self.agc_mailbox_expect(0, agc_params.ana_min, "ana_min")
            .await?;
        self.agc_mailbox_expect(1, agc_params.ana_max, "ana_max")
            .await?;

        // Configure analog thresholds
        self.sx1302_agc_mailbox_write(0, agc_params.ana_thresh_l)
            .await?;
        self.sx1302_agc_mailbox_write(1, agc_params.ana_thresh_h)
            .await?;
        self.sx1302_agc_mailbox_write(3, 0x04).await?;
        self.sx1302_agc_wait_status(0x05).await?;
        self.agc_mailbox_expect(0, agc_params.ana_thresh_l, "ana_thresh_l")
            .await?;
        self.agc_mailbox_expect(1, agc_params.ana_thresh_h, "ana_thresh_h")
            .await?;

        // Configure decimator attenuation min/max
        self.sx1302_agc_mailbox_write(0, agc_params.dec_attn_min)
            .await?;
        self.sx1302_agc_mailbox_write(1, agc_params.dec_attn_max)
            .await?;
        self.sx1302_agc_mailbox_write(3, 0x05).await?;
        self.sx1302_agc_wait_status(0x06).await?;
        self.agc_mailbox_expect(0, agc_params.dec_attn_min, "dec_attn_min")
            .await?;
        self.agc_mailbox_expect(1, agc_params.dec_attn_max, "dec_attn_max")
            .await?;

        // Configure decimator attenuation thresholds
        self.sx1302_agc_mailbox_write(0, agc_params.dec_thresh_l)
            .await?;
        self.sx1302_agc_mailbox_write(1, agc_params.dec_thresh_h1)
            .await?;
        self.sx1302_agc_mailbox_write(2, agc_params.dec_thresh_h2)
            .await?;
        self.sx1302_agc_mailbox_write(3, 0x06).await?;
        self.sx1302_agc_wait_status(0x07).await?;
        self.agc_mailbox_expect(0, agc_params.dec_thresh_l, "dec_thresh_l")
            .await?;
        self.agc_mailbox_expect(1, agc_params.dec_thresh_h1, "dec_thresh_h1")
            .await?;
        self.agc_mailbox_expect(2, agc_params.dec_thresh_h2, "dec_thresh_h2")
            .await?;

        // Configure channel attenuation min/max
        self.sx1302_agc_mailbox_write(0, agc_params.chan_attn_min)
            .await?;
        self.sx1302_agc_mailbox_write(1, agc_params.chan_attn_max)
            .await?;
        self.sx1302_agc_mailbox_write(3, 0x07).await?;
        self.sx1302_agc_wait_status(0x08).await?;
        self.agc_mailbox_expect(0, agc_params.chan_attn_min, "chan_attn_min")
            .await?;
        self.agc_mailbox_expect(1, agc_params.chan_attn_max, "chan_attn_max")
            .await?;

        // Configure channel attenuation threshold
        self.sx1302_agc_mailbox_write(0, agc_params.chan_thresh_l)
            .await?;
        self.sx1302_agc_mailbox_write(1, agc_params.chan_thresh_h)
            .await?;
        self.sx1302_agc_mailbox_write(3, 0x08).await?;
        self.sx1302_agc_wait_status(0x09).await?;
        self.agc_mailbox_expect(0, agc_params.chan_thresh_l, "chan_thresh_l")
            .await?;
        self.agc_mailbox_expect(1, agc_params.chan_thresh_h, "chan_thresh_h")
            .await?;

        // Configure sx1250 SetPAConfig
        // (C: guarded by `if (radio_type == LGW_RADIO_TYPE_SX1250)`, which is
        // always true here)
        self.sx1302_agc_mailbox_write(0, agc_params.device_sel)
            .await?;
        self.sx1302_agc_mailbox_write(1, agc_params.hp_max).await?;
        self.sx1302_agc_mailbox_write(2, agc_params.pa_duty_cycle)
            .await?;
        // notify AGC that params have been set to mailbox
        self.sx1302_agc_mailbox_write(3, 0x09).await?;
        // Wait for AGC to acknoledge it has received params
        self.sx1302_agc_wait_status(0x0A).await?;
        self.agc_mailbox_expect(0, agc_params.device_sel, "deviceSel")
            .await?;
        self.agc_mailbox_expect(1, agc_params.hp_max, "hpMax")
            .await?;
        self.agc_mailbox_expect(2, agc_params.pa_duty_cycle, "paDutyCycle")
            .await?;

        // Set PA start delay
        let pa_start_delay: u8 = 8;
        self.sx1302_agc_mailbox_write(0, pa_start_delay).await?; /* 1 LSB = 100 µs */
        self.sx1302_agc_mailbox_write(3, 0x0A).await?;
        self.sx1302_agc_wait_status(0x0B).await?;
        self.agc_mailbox_expect(0, pa_start_delay, "pa_start_delay")
            .await?;

        // Enable LBT if required (generic in this C section; the sx1261 LBT
        // specifics of the HAL are not ported)
        self.sx1302_agc_mailbox_write(0, if lbt_enable { 1 } else { 0 })
            .await?;
        self.sx1302_agc_mailbox_write(3, 0x0B).await?;
        self.sx1302_agc_wait_status(0x0F).await?;
        let val = self.sx1302_agc_mailbox_read(0).await?;
        if (val != 0) != lbt_enable {
            // C: "wrong LBT configuration"
            return Err(Error::InvalidConfig("lbt_enable"));
        }

        // notify AGC that configuration is finished
        self.sx1302_agc_mailbox_write(3, 0x0F).await?;

        Ok(())
    }

    // -----------------------------------------------------------------------
    // Arbiter MCU
    // -----------------------------------------------------------------------

    /// Upload the arbiter firmware image and verify it by read-back
    /// (C: `sx1302_arb_load_firmware`).
    ///
    /// Only the first [`MCU_FW_SIZE`] bytes of `firmware` are transferred,
    /// as in the C function.
    pub(crate) async fn sx1302_arb_load_firmware(&mut self, firmware: &[u8]) -> Result<()> {
        if firmware.len() < MCU_FW_SIZE {
            return Err(Error::InvalidConfig("firmware size"));
        }
        let image = &firmware[..MCU_FW_SIZE];

        // Take control over ARB MCU
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CTRL_MCU_CLEAR, 0x01)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CTRL_HOST_PROG, 0x01)
            .await?;
        self.com.reg_w(SX1302_REG_COMMON_PAGE_PAGE, 0x00).await?;

        // Write ARB fw in ARB MEM
        self.com.mem_wb(ARB_MEM_ADDR, image).await?;

        // Read back and check
        let mut fw_check = [0u8; MCU_FW_SIZE];
        self.com.mem_rb(ARB_MEM_ADDR, &mut fw_check, false).await?;
        if image != &fw_check[..] {
            return Err(Error::FirmwareVerify);
        }

        // Release control over ARB MCU
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CTRL_HOST_PROG, 0x00)
            .await?;
        self.com
            .reg_w(SX1302_REG_ARB_MCU_CTRL_MCU_CLEAR, 0x00)
            .await?;

        let val: i32 = self.com.reg_r(SX1302_REG_ARB_MCU_CTRL_PARITY_ERROR).await?;
        if val != 0 {
            // C: "Failed to load ARB fw: parity error check failed"
            return Err(Error::FirmwareVerify);
        }

        Ok(())
    }

    /// Read the arbiter MCU status register (C: `sx1302_arb_status`).
    pub(crate) async fn sx1302_arb_status(&mut self) -> Result<u8> {
        let val: i32 = self
            .com
            .reg_r(SX1302_REG_ARB_MCU_MCU_ARB_STATUS_MCU_ARB_STATUS)
            .await?;
        Ok(val as u8)
    }

    /// Poll until the arbiter MCU reports `status`
    /// (C: `sx1302_arb_wait_status`).
    ///
    /// The C loop has no timeout (`/* TODO: add timeout */`); this port
    /// returns [`Error::Timeout`] after [`STATUS_POLL_MAX_RETRIES`] polls.
    pub(crate) async fn sx1302_arb_wait_status(&mut self, status: u8) -> Result<()> {
        let mut retries: u32 = 0;
        loop {
            let val = self.sx1302_arb_status().await?;
            if val == status {
                return Ok(());
            }
            retries += 1;
            if retries >= STATUS_POLL_MAX_RETRIES {
                return Err(Error::Timeout);
            }
        }
    }

    /// Read an arbiter debug status register
    /// (C: `sx1302_arb_debug_read`).
    pub(crate) async fn sx1302_arb_debug_read(&mut self, reg_id: u8) -> Result<u8> {
        // Check parameters
        if reg_id > 15 {
            return Err(Error::InvalidConfig("reg_id"));
        }

        let reg = SX1302_REG_ARB_MCU_ARB_DEBUG_STS_0_ARB_DEBUG_STS_0 + reg_id as u16;
        let val: i32 = self.com.reg_r(reg).await?;
        Ok(val as u8)
    }

    /// Write an arbiter debug configuration register
    /// (C: `sx1302_arb_debug_write`).
    pub(crate) async fn sx1302_arb_debug_write(&mut self, reg_id: u8, value: u8) -> Result<()> {
        // Check parameters
        if reg_id > 3 {
            return Err(Error::InvalidConfig("reg_id"));
        }

        let reg = SX1302_REG_ARB_MCU_ARB_DEBUG_CFG_0_ARB_DEBUG_CFG_0 + reg_id as u16;
        self.com.reg_w(reg, value as i32).await
    }

    /// Run the arbiter firmware handshake: version check, double-demod
    /// (fine timestamp) configuration and resume (C: `sx1302_arb_start`).
    ///
    /// The C `ftime_context` pointer is replaced by `self.ctx.ftime`
    /// (see `crate::context::Context::ftime`).
    pub(crate) async fn sx1302_arb_start(&mut self, version: u8) -> Result<()> {
        // Wait for ARB fw to be started, and VERSION available in debug registers
        self.sx1302_arb_wait_status(0x01).await?;

        // Get firmware VERSION
        let val = self.sx1302_arb_debug_read(0).await?;
        if val != version {
            // C: "wrong ARB fw version"
            return Err(Error::FirmwareVerify);
        }

        // Enable ARB detect/modem alloc stats for the specified SF
        // (C: `sx1302_arb_set_debug_stats(true, DR_LORA_SF7)`, whose helper is
        // debug-only and not ported: it writes DR_LORA_SF7 to ARB_DEBUG_CFG_0)
        self.sx1302_arb_debug_write(0, DR_LORA_SF7 as u8).await?;

        // Enable/Disable double demod for different timing set (best timestamp
        // / best demodulation) - 1 bit per SF (LSB=SF5, MSB=SF12)
        // => 0:Disable 1:Enable
        let ftime = self.ctx.ftime;
        if !ftime.enable {
            // double demod disabled for all SF
            self.sx1302_arb_debug_write(3, 0x00).await?;
        } else if ftime.mode == FtimeMode::AllSf {
            // double demod enabled for all SF
            self.sx1302_arb_debug_write(3, 0xFF).await?;
        } else {
            // C: LGW_FTIME_MODE_HIGH_CAPACITY, double demod enabled for
            // SF10 <- SF5 (any other C mode is not representable in
            // `FtimeMode` and would be an error in C)
            self.sx1302_arb_debug_write(3, 0x3F).await?;
        }

        // Set double detect packet filtering threshold [0..3]
        self.sx1302_arb_debug_write(2, 3).await?;

        // Notify ARB that it can resume
        self.sx1302_arb_debug_write(1, 1).await?;

        // Wait for ARB to acknoledge
        self.sx1302_arb_wait_status(0x00).await?;

        Ok(())
    }
}
