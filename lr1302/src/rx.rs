//! SX1302 RX buffer fetch and packet parsing.
//!
//! Ports the C files `loragw_sx1302_rx.c` ([`Lr1302::rx_buffer_new`],
//! [`Lr1302::rx_buffer_del`], [`Lr1302::rx_buffer_fetch`],
//! [`Lr1302::rx_buffer_pop`]) and the RX parts of `loragw_sx1302.c`
//! ([`Lr1302::sx1302_fetch`], [`Lr1302::sx1302_parse`],
//! [`sx1302_lora_payload_crc`], [`sx1302_rssi_get_temperature_offset`]).
//!
//! Fine ("precision") timestamps are not ported: the
//! `precise_timestamp_calculate` branches of `sx1302_parse` are skipped and
//! [`RxPacket::ftime_received`] is always `false`, `RxPacket::ftime` always 0.

use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::spi::SpiDevice;

use crate::consts::*;
use crate::error::{Error, Result};
use crate::sx1302_defs::*;
use crate::types::{RssiTComp, RxPacket};
use crate::util::take_bits;
use crate::Lr1302;

// ---------------------------------------------------------------------------
// RX buffer packet framing (C: private macros/constants of loragw_sx1302_rx.c)
// ---------------------------------------------------------------------------

/// First byte of the packet sync word (C: `SX1302_PKT_SYNCWORD_BYTE_0`).
const SX1302_PKT_SYNCWORD_BYTE_0: u8 = 0xA5;
/// Second byte of the packet sync word (C: `SX1302_PKT_SYNCWORD_BYTE_1`).
const SX1302_PKT_SYNCWORD_BYTE_1: u8 = 0xC0;
/// Size of the metadata header of one packet (C: `SX1302_PKT_HEAD_METADATA`).
const SX1302_PKT_HEAD_METADATA: usize = 9;
/// Size of the metadata trailer of one packet (C: `SX1302_PKT_TAIL_METADATA`).
const SX1302_PKT_TAIL_METADATA: usize = 14;
/// Modem ID of the last multi-SF LoRa modem (C: `SX1302_LORA_MODEM_ID_MAX`,
/// used by the C header only).
#[allow(dead_code)]
const SX1302_LORA_MODEM_ID_MAX: u8 = 15;
/// Modem ID of the standard single-SF LoRa modem (C: `SX1302_LORA_STD_MODEM_ID`).
const SX1302_LORA_STD_MODEM_ID: u8 = 16;
/// Modem ID of the standard FSK modem (C: `SX1302_FSK_MODEM_ID`).
const SX1302_FSK_MODEM_ID: u8 = 17;
/// Capacity of the RX staging buffer (C: `buffer[4096]`).
const RX_BUFFER_CAPACITY: usize = 4096;

// ---------------------------------------------------------------------------
// Packet structure
// ---------------------------------------------------------------------------

/// Packet as stored in the SX1302 internal RX packet engine
/// (C: `rx_packet_t` in `loragw_sx1302_rx.h`).
///
/// Fields that the C code never writes when popping a packet
/// (`rssi_chan_max_neg_delta`, `rssi_chan_max_pos_delta`,
/// `rssi_sig_max_neg_delta`, `rssi_sig_max_pos_delta`, `packet_checksum`) are
/// zero-initialized here instead of carrying indeterminate stack values as in
/// C.
#[derive(Debug, Clone, Copy)]
pub struct RawRxPacket {
    /// Payload size in bytes (C: `rxbytenb_modem`).
    pub rxbytenb_modem: u8,
    /// IF chain the packet was received on (C: `rx_channel_in`).
    pub rx_channel_in: u8,
    /// CRC enabled for this packet (C: `crc_en`).
    pub crc_en: bool,
    /// Coding rate of the packet, LoRa only (C: `coding_rate`).
    pub coding_rate: u8,
    /// Spreading factor of the packet, LoRa only (C: `rx_rate_sf`).
    pub rx_rate_sf: u8,
    /// Modem that received the packet (C: `modem_id`).
    pub modem_id: u8,
    /// Frequency offset error of the packet in LSB, LoRa only
    /// (C: `frequency_offset_error`).
    pub frequency_offset_error: i32,
    /// Payload bytes (C: `payload[255]`).
    pub payload: [u8; 255],
    /// Payload CRC reported in error (C: `payload_crc_error`).
    pub payload_crc_error: bool,
    /// Sync word error (C: `sync_error`, LoRa only).
    pub sync_error: bool,
    /// Header error (C: `header_error`, LoRa only).
    pub header_error: bool,
    /// Fine timestamp requested for this packet (C: `timing_set`, LoRa only).
    pub timing_set: bool,
    /// Average SNR in 0.25 dB steps (C: `snr_average`, LoRa only).
    pub snr_average: i8,
    /// Average RSSI of the channel (C: `rssi_chan_avg`).
    pub rssi_chan_avg: u8,
    /// Average RSSI of the signal (C: `rssi_signal_avg`, LoRa only).
    pub rssi_signal_avg: u8,
    /// Maximum negative channel RSSI delta (C: `rssi_chan_max_neg_delta`).
    pub rssi_chan_max_neg_delta: u8,
    /// Maximum positive channel RSSI delta (C: `rssi_chan_max_pos_delta`).
    pub rssi_chan_max_pos_delta: u8,
    /// Maximum negative signal RSSI delta (C: `rssi_sig_max_neg_delta`).
    pub rssi_sig_max_neg_delta: u8,
    /// Maximum positive signal RSSI delta (C: `rssi_sig_max_pos_delta`).
    pub rssi_sig_max_pos_delta: u8,
    /// Packet timestamp at 32 MHz (C: `timestamp_cnt`).
    pub timestamp_cnt: u32,
    /// CRC16 carried in the packet payload (C: `rx_crc16_value`, LoRa only).
    pub rx_crc16_value: u16,
    /// Number of stored fine timestamp metrics (C: `num_ts_metrics_stored`).
    pub num_ts_metrics_stored: u8,
    /// Fine timestamp metrics (C: `timestamp_avg[255]`, LoRa only).
    pub timestamp_avg: [i8; 255],
    /// Fine timestamp standard deviations (C: `timestamp_stddev[255]`).
    pub timestamp_stddev: [i8; 255],
    /// Packet checksum byte as received (C: `packet_checksum`).
    pub packet_checksum: u8,
}

impl Default for RawRxPacket {
    fn default() -> Self {
        Self {
            rxbytenb_modem: 0,
            rx_channel_in: 0,
            crc_en: false,
            coding_rate: 0,
            rx_rate_sf: 0,
            modem_id: 0,
            frequency_offset_error: 0,
            payload: [0u8; 255],
            payload_crc_error: false,
            sync_error: false,
            header_error: false,
            timing_set: false,
            snr_average: 0,
            rssi_chan_avg: 0,
            rssi_signal_avg: 0,
            rssi_chan_max_neg_delta: 0,
            rssi_chan_max_pos_delta: 0,
            rssi_sig_max_neg_delta: 0,
            rssi_sig_max_pos_delta: 0,
            timestamp_cnt: 0,
            rx_crc16_value: 0,
            num_ts_metrics_stored: 0,
            timestamp_avg: [0i8; 255],
            timestamp_stddev: [0i8; 255],
            packet_checksum: 0,
        }
    }
}

// ---------------------------------------------------------------------------
// Private helpers
// ---------------------------------------------------------------------------

/// Read byte `idx` of a 4096-byte RX staging buffer (C: `buffer[idx]`).
///
/// `rx_buffer_fetch` zero-fills the whole staging buffer before every read
/// from the FIFO, exactly like the C `memset`, so every byte of the array that
/// was not filled reads as 0 here as well. Indices past the end of the array,
/// which would be out-of-bounds reads in the C code (possible only with
/// corrupted packet metadata), also read as 0 instead of panicking.
fn buf_get(buf: &[u8; RX_BUFFER_CAPACITY], idx: usize) -> u8 {
    buf.get(idx).copied().unwrap_or(0)
}

impl<SPI, D> Lr1302<SPI, D>
where
    SPI: SpiDevice<u8>,
    D: DelayNs,
{
    // -----------------------------------------------------------------------
    // RX buffer (C: loragw_sx1302_rx.c)
    // -----------------------------------------------------------------------

    /// Initialize the RX buffer instance (C: `rx_buffer_new`). No I/O.
    pub(crate) fn rx_buffer_new(&mut self) {
        // C: memset(self->buffer, 0, sizeof self->buffer)
        self.ctx.rx_buffer.buffer = [0u8; RX_BUFFER_CAPACITY];
        self.ctx.rx_buffer.buffer_size = 0;
        self.ctx.rx_buffer.buffer_index = 0;
        self.ctx.rx_buffer.buffer_pkt_nb = 0;
    }

    /// Reset the RX buffer instance (C: `rx_buffer_del`). No I/O.
    ///
    /// Only the indices are cleared; the buffer contents are left untouched,
    /// as in C.
    pub(crate) fn rx_buffer_del(&mut self) {
        self.ctx.rx_buffer.buffer_size = 0;
        self.ctx.rx_buffer.buffer_index = 0;
        self.ctx.rx_buffer.buffer_pkt_nb = 0;
    }

    /// Fetch packets from the SX1302 internal RX buffer and count the packets
    /// available (C: `rx_buffer_fetch`).
    pub(crate) async fn rx_buffer_fetch(&mut self) -> Result<()> {
        let mut buff = [0u8; 2];

        // Check if there is data in the FIFO (C reads 2 bytes: MSB then LSB)
        self.com
            .reg_rb(
                crate::reg::SX1302_REG_RX_TOP_RX_BUFFER_NB_BYTES_MSB_RX_BUFFER_NB_BYTES,
                &mut buff,
            )
            .await?;
        let nb_bytes_1: u16 = ((buff[0] as u16) << 8) | (buff[1] as u16);

        // Workaround for multi-byte read issue: read again and ensure the new
        // read is not lower than the previous one
        self.com
            .reg_rb(
                crate::reg::SX1302_REG_RX_TOP_RX_BUFFER_NB_BYTES_MSB_RX_BUFFER_NB_BYTES,
                &mut buff,
            )
            .await?;
        let nb_bytes_2: u16 = ((buff[0] as u16) << 8) | (buff[1] as u16);

        let nb_bytes = if nb_bytes_2 > nb_bytes_1 {
            nb_bytes_2
        } else {
            nb_bytes_1
        };
        // C stores the raw hardware count; clamped to the capacity of the
        // staging buffer so a corrupted count cannot overflow the array (the
        // C code would read past the end of `buffer[4096]` in that case).
        let mut size = (nb_bytes as usize).min(RX_BUFFER_CAPACITY);
        self.ctx.rx_buffer.buffer_size = size as u16;

        // Fetch bytes from fifo if any
        if size > 0 {
            // C: memset(self->buffer, 0, sizeof self->buffer)
            self.ctx.rx_buffer.buffer = [0u8; RX_BUFFER_CAPACITY];

            // C: lgw_mem_rb(0x4000, self->buffer, self->buffer_size, true) —
            // the RX FIFO base address is the literal 0x4000 in the C HAL
            self.com
                .mem_rb(0x4000, &mut self.ctx.rx_buffer.buffer[..size], true)
                .await?;

            // Sanity check: is there at least 1 complete packet in the buffer?
            // C: warning + return rx_buffer_del() (== LGW_REG_SUCCESS)
            if size < SX1302_PKT_HEAD_METADATA + SX1302_PKT_TAIL_METADATA {
                self.rx_buffer_del();
                return Ok(());
            }

            // Sanity check: is there a syncword at 0? If not, move to the
            // first syncword found
            let mut idx = 0usize;
            let mut found = false;
            {
                let buf = &self.ctx.rx_buffer.buffer;
                while idx <= size - 2 {
                    if buf_get(buf, idx) == SX1302_PKT_SYNCWORD_BYTE_0
                        && buf_get(buf, idx + 1) == SX1302_PKT_SYNCWORD_BYTE_1
                    {
                        found = true;
                        break;
                    }
                    idx += 1;
                }
            }
            if !found {
                // C: "no syncword found, discard rx_buffer"
                self.rx_buffer_del();
                return Ok(());
            }
            if idx != 0 {
                // C: memmove(buffer, buffer + idx, buffer_size - idx)
                self.ctx.rx_buffer.buffer.copy_within(idx..size, 0);
                size -= idx;
                self.ctx.rx_buffer.buffer_size = size as u16;
            }

            // Rewind and parse the buffer to get the number of packets fetched
            let mut pkt_nb: u8 = 0;
            let mut discard = false;
            {
                let buf = &self.ctx.rx_buffer.buffer;
                let mut i = 0usize;
                while i < size {
                    if buf_get(buf, i) != SX1302_PKT_SYNCWORD_BYTE_0
                        || buf_get(buf, i + 1) != SX1302_PKT_SYNCWORD_BYTE_1
                    {
                        // C: "syncword not found, discard the rx_buffer"
                        discard = true;
                        break;
                    }
                    // One packet found in the buffer
                    pkt_nb = pkt_nb.wrapping_add(1);

                    // Number of bytes for this packet
                    // C: SX1302_PKT_PAYLOAD_LENGTH(buffer, i) == buffer[i + 2]
                    let payload_len = buf_get(buf, i + 2) as usize;
                    // C: SX1302_PKT_NUM_TS_METRICS(buffer, i + payload_len)
                    let num_ts_metrics = buf_get(buf, i + payload_len + 21) as usize;
                    let next_pkt_idx = SX1302_PKT_HEAD_METADATA
                        + payload_len
                        + SX1302_PKT_TAIL_METADATA
                        + (2 * num_ts_metrics);

                    // Move to next packet
                    i += next_pkt_idx;
                }
            }
            if discard {
                self.rx_buffer_del();
                return Ok(());
            }
            self.ctx.rx_buffer.buffer_pkt_nb = self
                .ctx
                .rx_buffer
                .buffer_pkt_nb
                .wrapping_add(pkt_nb);
        }

        // Initialize the current buffer index to iterate on
        self.ctx.rx_buffer.buffer_index = 0;

        Ok(())
    }

    /// Parse the RX staging buffer and pop the first packet it contains
    /// (C: `rx_buffer_pop`).
    ///
    /// * `Ok(Some(_))` — C `LGW_REG_SUCCESS` (packet parsed),
    /// * `Ok(None)` — C `LGW_REG_WARNING` (truncated message or failed
    ///   checksum),
    /// * `Err(_)` — C `LGW_REG_ERROR` (fatal).
    pub(crate) fn rx_buffer_pop(&mut self) -> Result<Option<RawRxPacket>> {
        let mut pkt = RawRxPacket::default();

        // Is there any data to be parsed?
        if self.ctx.rx_buffer.buffer_index >= self.ctx.rx_buffer.buffer_size as i32 {
            // C: DEBUG_MSG("INFO: No more data to be parsed"); LGW_REG_ERROR
            return Err(Error::NothingToDo);
        }

        let base = self.ctx.rx_buffer.buffer_index as usize;
        let buf = &self.ctx.rx_buffer.buffer;

        // Get pkt sync words
        if buf_get(buf, base) != SX1302_PKT_SYNCWORD_BYTE_0
            || buf_get(buf, base + 1) != SX1302_PKT_SYNCWORD_BYTE_1
        {
            return Err(Error::InvalidConfig("syncword"));
        }

        // Get payload length (C: SX1302_PKT_PAYLOAD_LENGTH(buffer, base))
        pkt.rxbytenb_modem = buf_get(buf, base + 2);

        // Get fine timestamp metrics count (C: SX1302_PKT_NUM_TS_METRICS(
        // buffer, base + pkt->rxbytenb_modem))
        pkt.num_ts_metrics_stored = buf_get(buf, base + pkt.rxbytenb_modem as usize + 21);

        // Calculate the total number of bytes in the packet
        let pkt_num_bytes = SX1302_PKT_HEAD_METADATA
            + pkt.rxbytenb_modem as usize
            + SX1302_PKT_TAIL_METADATA
            + (2 * pkt.num_ts_metrics_stored as usize);

        // Check if we have a complete packet in the rx buffer fetched
        if base + pkt_num_bytes > self.ctx.rx_buffer.buffer_size as usize {
            // C: "aborting truncated message" + LGW_REG_WARNING
            return Ok(None);
        }

        // Get the checksum as received in the RX buffer
        let checksum_idx = pkt_num_bytes - 1;
        let checksum_rcv = buf_get(buf, base + pkt_num_bytes - 1);

        // Calculate the checksum from the actual payload bytes received
        let mut checksum_calc: u8 = 0;
        for i in 0..checksum_idx {
            checksum_calc = checksum_calc.wrapping_add(buf_get(buf, base + i));
        }

        // Check if the checksum is correct
        if checksum_rcv != checksum_calc {
            // C: "checksum failed" + LGW_REG_WARNING
            return Ok(None);
        }

        // Parse packet metadata. `tail` is the C expression
        // `self->buffer_index + pkt->rxbytenb_modem` used by the macros below.
        let tail = base + pkt.rxbytenb_modem as usize;
        pkt.modem_id = buf_get(buf, base + 5); // SX1302_PKT_MODEM_ID
        pkt.rx_channel_in = buf_get(buf, base + 3); // SX1302_PKT_CHANNEL
        // SX1302_PKT_CRC_EN: 1 bit at position 0 of buffer[base + 4]
        pkt.crc_en = take_bits(buf_get(buf, base + 4), 0, 1) != 0;
        // SX1302_PKT_CRC_ERROR / SYNC_ERROR / HEADER_ERROR / TIMING_SET:
        // single bits of buffer[tail + 9]
        pkt.payload_crc_error = take_bits(buf_get(buf, tail + 9), 0, 1) != 0;
        pkt.sync_error = take_bits(buf_get(buf, tail + 9), 2, 1) != 0;
        pkt.header_error = take_bits(buf_get(buf, tail + 9), 3, 1) != 0;
        pkt.timing_set = take_bits(buf_get(buf, tail + 9), 4, 1) != 0;
        // SX1302_PKT_CODING_RATE: 3 bits at position 1 of buffer[base + 4]
        pkt.coding_rate = take_bits(buf_get(buf, base + 4), 1, 3);
        // SX1302_PKT_DATARATE: 4 bits at position 4 of buffer[base + 4]
        pkt.rx_rate_sf = take_bits(buf_get(buf, base + 4), 4, 4);
        pkt.rssi_chan_avg = buf_get(buf, tail + 11); // SX1302_PKT_RSSI_CHAN
        pkt.rssi_signal_avg = buf_get(buf, tail + 12); // SX1302_PKT_RSSI_SIG
        pkt.rx_crc16_value = ((buf_get(buf, tail + 19) as u16) << 0) & 0x00FF
            | ((buf_get(buf, tail + 20) as u16) << 8) & 0xFF00;
        pkt.snr_average = buf_get(buf, tail + 10) as i8; // SX1302_PKT_SNR_AVG

        // Frequency offset, signed on 20 bits
        // (FREQ_OFFSET_19_16 / _15_8 / _7_0 of buffer[base + 8..=6])
        let mut frequency_offset_error: i32 = ((take_bits(buf_get(buf, base + 8), 0, 4) as i32)
            << 16)
            | ((buf_get(buf, base + 7) as i32) << 8)
            | (buf_get(buf, base + 6) as i32);
        if frequency_offset_error >= (1 << 19) {
            // Handle signed value on 20 bits
            frequency_offset_error -= 1 << 20;
        }
        pkt.frequency_offset_error = frequency_offset_error;

        // Packet timestamp (32 MHz), little-endian
        // (TIMESTAMP_7_0..31_24 of buffer[tail + 15..=18])
        pkt.timestamp_cnt = ((buf_get(buf, tail + 15) as u32) << 0) & 0x0000_00FF
            | ((buf_get(buf, tail + 16) as u32) << 8) & 0x0000_FF00
            | ((buf_get(buf, tail + 17) as u32) << 16) & 0x00FF_0000
            | ((buf_get(buf, tail + 18) as u32) << 24) & 0xFF00_0000;

        // TS metrics: it is expected the nb_symbols parameter is set to 0
        // here. The C code writes `num_ts_metrics_stored * 2` entries (up to
        // 510) into the 255-entry arrays; clamped here because the count
        // comes from corrupted packet metadata in that case.
        let nb_ts_entries = (pkt.num_ts_metrics_stored as usize) * 2;
        for i in 0..nb_ts_entries.min(pkt.timestamp_avg.len()) {
            // C: SX1302_PKT_NUM_TS_METRICS(buffer,
            //     buffer_index + rxbytenb_modem + 1 + i)
            pkt.timestamp_avg[i] = buf_get(
                buf,
                base + pkt.rxbytenb_modem as usize + 1 + i + 21,
            ) as i8;
            pkt.timestamp_stddev[i] = 0; // no stddev when nb_symbols == 0
        }

        // Sanity checks: check the range of a few metadata
        if pkt.modem_id > SX1302_FSK_MODEM_ID {
            return Err(Error::InvalidConfig("modem_id"));
        }
        if pkt.modem_id <= SX1302_LORA_STD_MODEM_ID {
            // LoRa modems
            if pkt.rx_channel_in > 9 {
                return Err(Error::InvalidConfig("channel"));
            }
            if pkt.rx_rate_sf < 5 || pkt.rx_rate_sf > 12 {
                return Err(Error::InvalidConfig("sf"));
            }
        }
        // else: FSK modem, not checked (C: "TODO: not checked")

        // Parse & copy payload in packet struct
        let payload_start = base + SX1302_PKT_HEAD_METADATA;
        let payload_len = pkt.rxbytenb_modem as usize;
        pkt.payload[..payload_len]
            .copy_from_slice(&buf[payload_start..payload_start + payload_len]);

        // Move buffer index toward next message
        self.ctx.rx_buffer.buffer_index += (SX1302_PKT_HEAD_METADATA
            + pkt.rxbytenb_modem as usize
            + SX1302_PKT_TAIL_METADATA
            + (2 * pkt.num_ts_metrics_stored as usize))
            as i32;

        // Update the number of packets currently stored in the rx_buffer
        // (C: uint8_t wrap-around semantics preserved with wrapping_sub)
        self.ctx.rx_buffer.buffer_pkt_nb = self.ctx.rx_buffer.buffer_pkt_nb.wrapping_sub(1);

        Ok(Some(pkt))
    }

    // -----------------------------------------------------------------------
    // Fetch / parse (C: loragw_sx1302.c)
    // -----------------------------------------------------------------------

    /// Fetch packets from the SX1302 if no more are left in the RX buffer
    /// (C: `sx1302_fetch`).
    ///
    /// Returns the number of packets available in the RX buffer (C writes it
    /// through the `nb_pkt` output parameter).
    pub(crate) async fn sx1302_fetch(&mut self) -> Result<u8> {
        // Fetch packets from sx1302 if no more left in RX buffer
        if self.ctx.rx_buffer.buffer_pkt_nb == 0 {
            // Initialize RX buffer (C: rx_buffer_new, cannot fail here)
            self.rx_buffer_new();

            // Fetch RX buffer if any data available
            self.rx_buffer_fetch().await?;
        }
        // else C: "remaining %u packets in RX buffer, do not fetch yet"

        // Return the number of packets fetched
        Ok(self.ctx.rx_buffer.buffer_pkt_nb)
    }

    /// Parse the next packet of the RX buffer (C: `sx1302_parse`).
    ///
    /// * `Ok(Some(_))` — C `LGW_REG_SUCCESS` (packet parsed),
    /// * `Ok(None)` — C `LGW_REG_WARNING` (buffer cleared, stop parsing),
    /// * `Err(_)` — C `LGW_REG_ERROR` (fatal).
    pub(crate) async fn sx1302_parse(&mut self) -> Result<Option<RxPacket>> {
        // Get packet from RX buffer
        let pkt = match self.rx_buffer_pop()? {
            Some(pkt) => pkt,
            None => {
                // C: LGW_REG_WARNING → rx_buffer_del(&rx_buffer); return
                self.rx_buffer_del();
                return Ok(None);
            }
        };

        let mut p = RxPacket::default();

        // Fine timestamps are not ported (C sets ftime_received = false and
        // ftime = 0 just before its precise_timestamp_calculate branch)
        p.ftime_received = false;
        p.ftime = 0;

        // Copy payload to result struct
        p.size = pkt.rxbytenb_modem as u16;
        p.payload[..pkt.rxbytenb_modem as usize]
            .copy_from_slice(&pkt.payload[..pkt.rxbytenb_modem as usize]);

        // Process metadata
        p.modem_id = pkt.modem_id;
        p.if_chain = pkt.rx_channel_in;
        if p.if_chain as usize >= LGW_IF_CHAIN_NB {
            // C: "NOT A VALID IF_CHAIN NUMBER, ABORTING" + LGW_REG_ERROR
            return Err(Error::InvalidConfig("if_chain"));
        }
        let ifmod = crate::sx1302::sx1302_get_ifmod_config(p.if_chain);

        p.rf_chain = self.ctx.if_chain[p.if_chain as usize].rf_chain;

        // Get the frequency for the channel configuration
        p.freq_hz = (self.ctx.rf_chain[p.rf_chain as usize].freq_hz as i32)
            .wrapping_add(self.ctx.if_chain[p.if_chain as usize].freq_hz) as u32;

        // Get signal strength: offset and temperature compensation will be
        // applied later
        p.rssic = pkt.rssi_chan_avg as f32;
        p.rssis = pkt.rssi_signal_avg as f32;

        // Get timestamp correction to be applied to count_us
        let timestamp_correction: i32;

        // Get modulation metadata
        if ifmod == IF_LORA_MULTI || ifmod == IF_LORA_STD {
            p.modulation = MOD_LORA;

            // Get CRC status
            if pkt.crc_en || (ifmod == IF_LORA_STD && self.ctx.lora_service.implicit_crc_en) {
                // CRC enabled
                if pkt.payload_crc_error {
                    p.status = STAT_CRC_BAD;
                } else {
                    p.status = STAT_CRC_OK;

                    // Sanity check of the payload CRC
                    if p.size > 0 {
                        let payload_crc16_calc =
                            sx1302_lora_payload_crc(&p.payload[..p.size as usize]);
                        if payload_crc16_calc != pkt.rx_crc16_value {
                            // C: "Payload CRC16 check failed" + LGW_REG_ERROR
                            return Err(Error::InvalidConfig("payload crc16"));
                        }
                    }
                }
            } else {
                // CRC disabled
                p.status = STAT_NO_CRC;
            }

            // Get SNR - converted from 0.25dB step to dB
            p.snr = pkt.snr_average as f32 / 4.0;

            // Get bandwidth
            p.bandwidth = if ifmod == IF_LORA_MULTI {
                BW_125KHZ // fixed in hardware
            } else {
                // get the parameter from the config variable
                self.ctx.lora_service.bandwidth
            };

            // Get datarate
            p.datarate = match pkt.rx_rate_sf {
                5 => DR_LORA_SF5,
                6 => DR_LORA_SF6,
                7 => DR_LORA_SF7,
                8 => DR_LORA_SF8,
                9 => DR_LORA_SF9,
                10 => DR_LORA_SF10,
                11 => DR_LORA_SF11,
                12 => DR_LORA_SF12,
                _ => DR_UNDEFINED,
            };

            // Get coding rate
            let cr = if ifmod == IF_LORA_MULTI || !self.ctx.lora_service.implicit_hdr {
                pkt.coding_rate
            } else {
                self.ctx.lora_service.implicit_coderate
            };
            p.coderate = match cr {
                1 => CR_LORA_4_5,
                2 => CR_LORA_4_6,
                3 => CR_LORA_4_7,
                4 => CR_LORA_4_8,
                _ => CR_UNDEFINED,
            };

            // Get frequency offset in Hz depending on bandwidth
            p.freq_offset = match p.bandwidth {
                BW_125KHZ => {
                    (pkt.frequency_offset_error as f32 * FREQ_OFFSET_LSB_125KHZ) as i32
                }
                BW_250KHZ => {
                    (pkt.frequency_offset_error as f32 * FREQ_OFFSET_LSB_250KHZ) as i32
                }
                BW_500KHZ => {
                    (pkt.frequency_offset_error as f32 * FREQ_OFFSET_LSB_500KHZ) as i32
                }
                _ => 0, // C: "Invalid frequency offset"
            };

            // Adjust the frequency offset with channel IF frequency error:
            // when the channel IF frequency has been configured, a precision
            // error may have been introduced due to the register precision.
            let if_freq_hz = self.ctx.if_chain[p.if_chain as usize].freq_hz;
            // C: if_freq_hz - (IF_HZ_TO_REG(if_freq_hz) * 15625 / 32)
            let if_freq_error = if_freq_hz - (if_hz_to_reg(if_freq_hz) * 15625 / 32);
            p.freq_offset += if_freq_error;

            // Get timestamp correction to be applied to count_us
            timestamp_correction = self.timestamp_counter_correction(
                p.bandwidth,
                p.datarate,
                p.coderate,
                pkt.crc_en,
                pkt.rxbytenb_modem,
                crate::timestamp::RX_DFT_PEAK_MODE_AUTO,
            );

            // NOTE: not ported — the C branch that computes the fine
            // timestamp when (num_ts_metrics_stored > 0 && timing_set &&
            // status == STAT_CRC_OK) via precise_timestamp_calculate().
            // ftime_received stays false, ftime stays 0 (set above).
        } else if ifmod == IF_FSK_STD {
            p.modulation = MOD_FSK;

            // Get CRC status
            if pkt.crc_en {
                // CRC enabled
                p.status = if pkt.payload_crc_error {
                    STAT_CRC_BAD
                } else {
                    STAT_CRC_OK
                };
            } else {
                // CRC disabled
                p.status = STAT_NO_CRC;
            }

            // Get modulation params
            p.bandwidth = self.ctx.fsk.bandwidth;
            p.datarate = self.ctx.fsk.datarate;

            // Compute timestamp correction to be applied
            // C: ((uint32_t)680000 / context->fsk_cfg.datarate) - 20
            timestamp_correction =
                (680_000u32 / self.ctx.fsk.datarate).wrapping_sub(20) as i32;

            // RSSI correction (C: double polynomial with `pow(rssic, 2/3)`,
            // expanded to explicit products because `core` has no `pow`)
            let rssic = p.rssic as f64;
            let rssic2 = rssic * rssic; // C: pow(p->rssic, 2)
            let rssic3 = rssic2 * rssic; // C: pow(p->rssic, 3)
            p.rssic = (RSSI_FSK_POLY_0
                + RSSI_FSK_POLY_1 * rssic
                + RSSI_FSK_POLY_2 * rssic2
                + RSSI_FSK_POLY_3 * rssic3) as f32;

            // Undefined for FSK
            p.coderate = CR_UNDEFINED;
            p.snr = -128.0;
            p.rssis = -128.0;
        } else {
            // C: "UNEXPECTED PACKET ORIGIN"
            p.status = STAT_UNDEFINED;
            p.modulation = MOD_UNDEFINED;
            p.rssic = -128.0;
            p.rssis = -128.0;
            p.snr = -128.0;
            p.snr_min = -128.0;
            p.snr_max = -128.0;
            p.bandwidth = BW_UNDEFINED;
            p.datarate = DR_UNDEFINED;
            p.coderate = CR_UNDEFINED;
            timestamp_correction = 0;
        }

        // Scale 32 MHz packet timestamp to 1 MHz (microseconds)
        p.count_us = pkt.timestamp_cnt / 32;

        // Expand 27-bits counter to 32-bits counter, based on current
        // wrapping status (updated after fetch)
        p.count_us = self.timestamp_pkt_expand(p.count_us);

        // Packet timestamp corrected
        p.count_us = p.count_us.wrapping_add(timestamp_correction as u32);

        // Packet CRC status
        p.crc = pkt.rx_crc16_value;

        Ok(Some(p))
    }
}

// ---------------------------------------------------------------------------
// Free functions (C: loragw_sx1302.c)
// ---------------------------------------------------------------------------

/// Compute the CRC16 of a LoRa payload (C: `sx1302_lora_payload_crc`).
///
/// The C function takes a pointer and a `uint8_t` size; the size is implied
/// by the slice length here.
pub fn sx1302_lora_payload_crc(data: &[u8]) -> u16 {
    let mut crc = 0;
    for &byte in data {
        lora_crc16(byte, &mut crc);
    }
    crc as u16
}

/// One-byte step of the LoRa payload CRC16 (C: `lora_crc16`).
fn lora_crc16(data: u8, crc: &mut i32) {
    let d = data as i32;
    let c = *crc;
    let mut next: i32;
    next = (d >> 0) & 1 ^ (c >> 12) & 1 ^ (c >> 8) & 1;
    next += ((d >> 1) & 1 ^ (c >> 13) & 1 ^ (c >> 9) & 1) << 1;
    next += ((d >> 2) & 1 ^ (c >> 14) & 1 ^ (c >> 10) & 1) << 2;
    next += ((d >> 3) & 1 ^ (c >> 15) & 1 ^ (c >> 11) & 1) << 3;
    next += ((d >> 4) & 1 ^ (c >> 12) & 1) << 4;
    next += ((d >> 5) & 1 ^ (c >> 13) & 1 ^ (c >> 12) & 1 ^ (c >> 8) & 1) << 5;
    next += ((d >> 6) & 1 ^ (c >> 14) & 1 ^ (c >> 13) & 1 ^ (c >> 9) & 1) << 6;
    next += ((d >> 7) & 1 ^ (c >> 15) & 1 ^ (c >> 14) & 1 ^ (c >> 10) & 1) << 7;
    next += ((c >> 0) & 1 ^ (c >> 15) & 1 ^ (c >> 11) & 1) << 8;
    next += ((c >> 1) & 1 ^ (c >> 12) & 1) << 9;
    next += ((c >> 2) & 1 ^ (c >> 13) & 1) << 10;
    next += ((c >> 3) & 1 ^ (c >> 14) & 1) << 11;
    next += ((c >> 4) & 1 ^ (c >> 15) & 1 ^ (c >> 12) & 1 ^ (c >> 8) & 1) << 12;
    next += ((c >> 5) & 1 ^ (c >> 13) & 1 ^ (c >> 9) & 1) << 13;
    next += ((c >> 6) & 1 ^ (c >> 14) & 1 ^ (c >> 10) & 1) << 14;
    next += ((c >> 7) & 1 ^ (c >> 15) & 1 ^ (c >> 11) & 1) << 15;
    *crc = next;
}

/// Compute the RSSI offset to apply for the given temperature
/// (C: `sx1302_rssi_get_temperature_offset`).
///
/// The C code performs the whole polynomial evaluation in `double` (`pow`)
/// and truncates the result to `float` on return. `core` has no `pow`, so
/// `pow(t, n)` with integral `n` is expanded into explicit products
/// (`t * t`, `t * t * t`, ...), which is what C's `pow` computes for integral
/// exponents anyway.
pub fn sx1302_rssi_get_temperature_offset(context: &RssiTComp, temperature: f32) -> f32 {
    let t = temperature as f64;
    let t2 = t * t; // C: pow(temperature, 2)
    let t3 = t2 * t; // C: pow(temperature, 3)
    let t4 = t3 * t; // C: pow(temperature, 4)
    let a = context.coeff_a as f64;
    let b = context.coeff_b as f64;
    let c = context.coeff_c as f64;
    let d = context.coeff_d as f64;
    let e = context.coeff_e as f64;
    let offset = ((a * t4) + (b * t3) + (c * t2) + (d * t) + e) / 65536.0; // C: / pow(2, 16)
    offset as f32
}
