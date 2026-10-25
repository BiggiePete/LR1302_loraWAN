//! Register map for the Semtech SX1302 concentrator.
//!
//! GENERATED FILE - do not edit by hand.
//! Produced by `tools/gen_reg.py` from the Semtech SX1302 HAL v2.1.0
//! sources (`loragw_reg.h` / `loragw_reg.c`).
//!
//! Names and indices are identical to the C HAL `#define` values, so a
//! register referenced as `SX1302_REG_...` in the C sources maps 1:1 to
//! the Rust constant of the same name.

/// Number of registers in [`LOREGS`] (C: `LGW_TOTALREGS`).
pub const LGW_TOTALREGS: u16 = 1044;

/// Descriptor of one register, mirroring `struct lgw_reg_s`.
///
/// The legacy C `page` field is dropped: it is never read by the C
/// register access routines.
#[derive(Clone, Copy, Debug)]
pub struct RegDesc {
    /// 15-bit register address
    pub addr: u16,
    /// bit position of the LSB inside the addressed byte (0..7)
    pub offs: u8,
    /// field width in bits
    pub leng: u8,
    /// reset/default value
    pub dflt: i16,
    /// bit flags, see `FLAG_*`
    pub flags: u8,
}

/// Field is signed (2's complement)
pub const FLAG_SIGN: u8 = 0x01;
/// Field is read-only
pub const FLAG_RDON: u8 = 0x02;
/// Field can be checked (pulse, w0clr, w1clr)
pub const FLAG_CHCK: u8 = 0x04;

impl RegDesc {
    pub const fn is_signed(&self) -> bool { self.flags & FLAG_SIGN != 0 }
    pub const fn is_read_only(&self) -> bool { self.flags & FLAG_RDON != 0 }
    pub const fn is_checkable(&self) -> bool { self.flags & FLAG_CHCK != 0 }
}

/// Register descriptors, indexed by the register name constants below
/// (C: `loregs[]`, 1044 entries).
pub static LOREGS: [RegDesc; LGW_TOTALREGS as usize] = [
    RegDesc { addr: 0x5600, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 0: COMMON_PAGE_PAGE
    RegDesc { addr: 0x5601, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 1: COMMON_CTRL0_CLK32_RIF_CTRL
    RegDesc { addr: 0x5601, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 2: COMMON_CTRL0_HOST_RADIO_CTRL
    RegDesc { addr: 0x5601, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 3: COMMON_CTRL0_RADIO_MISC_EN
    RegDesc { addr: 0x5601, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 4: COMMON_CTRL0_SX1261_MODE_RADIO_B
    RegDesc { addr: 0x5601, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 5: COMMON_CTRL0_SX1261_MODE_RADIO_A
    RegDesc { addr: 0x5602, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 6: COMMON_CTRL1_SWAP_IQ_RADIO_B
    RegDesc { addr: 0x5602, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 7: COMMON_CTRL1_SAMPLING_EDGE_RADIO_B
    RegDesc { addr: 0x5602, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 8: COMMON_CTRL1_SWAP_IQ_RADIO_A
    RegDesc { addr: 0x5602, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 9: COMMON_CTRL1_SAMPLING_EDGE_RADIO_A
    RegDesc { addr: 0x5603, offs: 0, leng: 8, dflt: 2, flags: 0x04 }, // 10: COMMON_SPI_DIV_RATIO_SPI_HALF_PERIOD
    RegDesc { addr: 0x5604, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 11: COMMON_RADIO_SELECT_RADIO_SELECT
    RegDesc { addr: 0x5605, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 12: COMMON_GEN_GLOBAL_EN
    RegDesc { addr: 0x5605, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 13: COMMON_GEN_FSK_MODEM_ENABLE
    RegDesc { addr: 0x5605, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 14: COMMON_GEN_CONCENTRATOR_MODEM_ENABLE
    RegDesc { addr: 0x5605, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 15: COMMON_GEN_MBWSSF_MODEM_ENABLE
    RegDesc { addr: 0x5606, offs: 0, leng: 8, dflt: 16, flags: 0x06 }, // 16: COMMON_VERSION_VERSION
    RegDesc { addr: 0x5607, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 17: COMMON_DUMMY_DUMMY
    RegDesc { addr: 0x5780, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 18: AGC_MCU_CTRL_CLK_EN
    RegDesc { addr: 0x5780, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 19: AGC_MCU_CTRL_FORCE_HOST_FE_CTRL
    RegDesc { addr: 0x5780, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 20: AGC_MCU_CTRL_MCU_CLEAR
    RegDesc { addr: 0x5780, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 21: AGC_MCU_CTRL_HOST_PROG
    RegDesc { addr: 0x5780, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 22: AGC_MCU_CTRL_PARITY_ERROR
    RegDesc { addr: 0x5781, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 23: AGC_MCU_MCU_AGC_STATUS_MCU_AGC_STATUS
    RegDesc { addr: 0x5782, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 24: AGC_MCU_PA_GAIN_PA_B_GAIN
    RegDesc { addr: 0x5782, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 25: AGC_MCU_PA_GAIN_PA_A_GAIN
    RegDesc { addr: 0x5783, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 26: AGC_MCU_RF_EN_A_RADIO_RST
    RegDesc { addr: 0x5783, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 27: AGC_MCU_RF_EN_A_RADIO_EN
    RegDesc { addr: 0x5783, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 28: AGC_MCU_RF_EN_A_PA_EN
    RegDesc { addr: 0x5783, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 29: AGC_MCU_RF_EN_A_LNA_EN
    RegDesc { addr: 0x5784, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 30: AGC_MCU_RF_EN_B_RADIO_RST
    RegDesc { addr: 0x5784, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 31: AGC_MCU_RF_EN_B_RADIO_EN
    RegDesc { addr: 0x5784, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 32: AGC_MCU_RF_EN_B_PA_EN
    RegDesc { addr: 0x5784, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 33: AGC_MCU_RF_EN_B_LNA_EN
    RegDesc { addr: 0x5785, offs: 4, leng: 4, dflt: 0, flags: 0x04 }, // 34: AGC_MCU_LUT_TABLE_A_PA_LUT
    RegDesc { addr: 0x5785, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 35: AGC_MCU_LUT_TABLE_A_LNA_LUT
    RegDesc { addr: 0x5786, offs: 4, leng: 4, dflt: 0, flags: 0x04 }, // 36: AGC_MCU_LUT_TABLE_B_PA_LUT
    RegDesc { addr: 0x5786, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 37: AGC_MCU_LUT_TABLE_B_LNA_LUT
    RegDesc { addr: 0x5787, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 38: AGC_MCU_UART_CFG_MSBF
    RegDesc { addr: 0x5787, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 39: AGC_MCU_UART_CFG_PAR_EN
    RegDesc { addr: 0x5787, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 40: AGC_MCU_UART_CFG_PAR_MODE
    RegDesc { addr: 0x5787, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 41: AGC_MCU_UART_CFG_START_LEN
    RegDesc { addr: 0x5787, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 42: AGC_MCU_UART_CFG_STOP_LEN
    RegDesc { addr: 0x5787, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 43: AGC_MCU_UART_CFG_WORD_LEN
    RegDesc { addr: 0x5788, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 44: AGC_MCU_UART_CFG2_BIT_RATE
    RegDesc { addr: 0x5789, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 45: AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE3_MCU_MAIL_BOX_WR_DATA
    RegDesc { addr: 0x578A, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 46: AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE2_MCU_MAIL_BOX_WR_DATA
    RegDesc { addr: 0x578B, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 47: AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE1_MCU_MAIL_BOX_WR_DATA
    RegDesc { addr: 0x578C, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 48: AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE0_MCU_MAIL_BOX_WR_DATA
    RegDesc { addr: 0x578D, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 49: AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE3_MCU_MAIL_BOX_RD_DATA
    RegDesc { addr: 0x578E, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 50: AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE2_MCU_MAIL_BOX_RD_DATA
    RegDesc { addr: 0x578F, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 51: AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE1_MCU_MAIL_BOX_RD_DATA
    RegDesc { addr: 0x5790, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 52: AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE0_MCU_MAIL_BOX_RD_DATA
    RegDesc { addr: 0x5791, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 53: AGC_MCU_DUMMY_DUMMY3
    RegDesc { addr: 0x57C0, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 54: CLK_CTRL_CLK_SEL_CLKDIV_EN
    RegDesc { addr: 0x57C0, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 55: CLK_CTRL_CLK_SEL_CLK_RADIO_B_SEL
    RegDesc { addr: 0x57C0, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 56: CLK_CTRL_CLK_SEL_CLK_RADIO_A_SEL
    RegDesc { addr: 0x57C1, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 57: CLK_CTRL_DUMMY_DUMMY
    RegDesc { addr: 0x5200, offs: 3, leng: 1, dflt: 0, flags: 0x00 }, // 58: TX_TOP_A_TX_TRIG_TX_FSM_CLR
    RegDesc { addr: 0x5200, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 59: TX_TOP_A_TX_TRIG_TX_TRIG_GPS
    RegDesc { addr: 0x5200, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 60: TX_TOP_A_TX_TRIG_TX_TRIG_DELAYED
    RegDesc { addr: 0x5200, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 61: TX_TOP_A_TX_TRIG_TX_TRIG_IMMEDIATE
    RegDesc { addr: 0x5201, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 62: TX_TOP_A_TIMER_TRIG_BYTE3_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5202, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 63: TX_TOP_A_TIMER_TRIG_BYTE2_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5203, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 64: TX_TOP_A_TIMER_TRIG_BYTE1_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5204, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 65: TX_TOP_A_TIMER_TRIG_BYTE0_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5205, offs: 0, leng: 8, dflt: 187, flags: 0x04 }, // 66: TX_TOP_A_TX_START_DELAY_MSB_TX_START_DELAY
    RegDesc { addr: 0x5206, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 67: TX_TOP_A_TX_START_DELAY_LSB_TX_START_DELAY
    RegDesc { addr: 0x5207, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 68: TX_TOP_A_TX_CTRL_WRITE_BUFFER
    RegDesc { addr: 0x5208, offs: 0, leng: 3, dflt: 1, flags: 0x04 }, // 69: TX_TOP_A_TX_RAMP_DURATION_TX_RAMP_DURATION
    RegDesc { addr: 0x5209, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 70: TX_TOP_A_GEN_CFG_0_MODULATION_TYPE
    RegDesc { addr: 0x520A, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 71: TX_TOP_A_TEST_0_TX_ACTIVE_CTRL
    RegDesc { addr: 0x520A, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 72: TX_TOP_A_TEST_0_TX_ACTIVE_SEL
    RegDesc { addr: 0x520B, offs: 1, leng: 1, dflt: 0, flags: 0x00 }, // 73: TX_TOP_A_TX_FLAG_TX_TIMEOUT
    RegDesc { addr: 0x520B, offs: 0, leng: 1, dflt: 0, flags: 0x00 }, // 74: TX_TOP_A_TX_FLAG_PKT_DONE
    RegDesc { addr: 0x520C, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 75: TX_TOP_A_AGC_TX_BW_AGC_TX_BW
    RegDesc { addr: 0x520D, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 76: TX_TOP_A_AGC_TX_PWR_AGC_TX_PWR
    RegDesc { addr: 0x520E, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 77: TX_TOP_A_TIMEOUT_CNT_BYTE_2_TIMEOUT_CNT
    RegDesc { addr: 0x520F, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 78: TX_TOP_A_TIMEOUT_CNT_BYTE_1_TIMEOUT_CNT
    RegDesc { addr: 0x5210, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 79: TX_TOP_A_TIMEOUT_CNT_BYTE_0_TIMEOUT_CNT
    RegDesc { addr: 0x5211, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 80: TX_TOP_A_TX_FSM_STATUS_TX_STATUS
    RegDesc { addr: 0x5212, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 81: TX_TOP_A_DUMMY_CONTROL_DUMMY
    RegDesc { addr: 0x5220, offs: 5, leng: 3, dflt: 0, flags: 0x04 }, // 82: TX_TOP_A_TX_RFFE_IF_CTRL_PLL_DIV_CTRL
    RegDesc { addr: 0x5220, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 83: TX_TOP_A_TX_RFFE_IF_CTRL_TX_CLK_EDGE
    RegDesc { addr: 0x5220, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 84: TX_TOP_A_TX_RFFE_IF_CTRL_TX_MODE
    RegDesc { addr: 0x5220, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 85: TX_TOP_A_TX_RFFE_IF_CTRL_TX_IF_DST
    RegDesc { addr: 0x5220, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 86: TX_TOP_A_TX_RFFE_IF_CTRL_TX_IF_SRC
    RegDesc { addr: 0x5221, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 87: TX_TOP_A_TX_RFFE_IF_CTRL2_SX125X_IQ_INVERT
    RegDesc { addr: 0x5221, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 88: TX_TOP_A_TX_RFFE_IF_CTRL2_PLL_DIV_CTRL_AGC
    RegDesc { addr: 0x5222, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 89: TX_TOP_A_TX_RFFE_IF_IQ_GAIN_IQ_GAIN
    RegDesc { addr: 0x5223, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 90: TX_TOP_A_TX_RFFE_IF_I_OFFSET_I_OFFSET
    RegDesc { addr: 0x5224, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 91: TX_TOP_A_TX_RFFE_IF_Q_OFFSET_Q_OFFSET
    RegDesc { addr: 0x5225, offs: 0, leng: 8, dflt: 108, flags: 0x04 }, // 92: TX_TOP_A_TX_RFFE_IF_FREQ_RF_H_FREQ_RF
    RegDesc { addr: 0x5226, offs: 0, leng: 8, dflt: 144, flags: 0x04 }, // 93: TX_TOP_A_TX_RFFE_IF_FREQ_RF_M_FREQ_RF
    RegDesc { addr: 0x5227, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 94: TX_TOP_A_TX_RFFE_IF_FREQ_RF_L_FREQ_RF
    RegDesc { addr: 0x5228, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 95: TX_TOP_A_TX_RFFE_IF_FREQ_DEV_H_FREQ_DEV
    RegDesc { addr: 0x5229, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 96: TX_TOP_A_TX_RFFE_IF_FREQ_DEV_L_FREQ_DEV
    RegDesc { addr: 0x522A, offs: 0, leng: 8, dflt: 64, flags: 0x04 }, // 97: TX_TOP_A_TX_RFFE_IF_TEST_MOD_FREQ
    RegDesc { addr: 0x522B, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 98: TX_TOP_A_DUMMY_MODULATOR_DUMMY
    RegDesc { addr: 0x5240, offs: 0, leng: 8, dflt: 15, flags: 0x04 }, // 99: TX_TOP_A_FSK_PKT_LEN_PKT_LENGTH
    RegDesc { addr: 0x5241, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 100: TX_TOP_A_FSK_CFG_0_TX_CONT
    RegDesc { addr: 0x5241, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 101: TX_TOP_A_FSK_CFG_0_CRC_IBM
    RegDesc { addr: 0x5241, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 102: TX_TOP_A_FSK_CFG_0_DCFREE_ENC
    RegDesc { addr: 0x5241, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 103: TX_TOP_A_FSK_CFG_0_CRC_EN
    RegDesc { addr: 0x5241, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 104: TX_TOP_A_FSK_CFG_0_PKT_MODE
    RegDesc { addr: 0x5242, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 105: TX_TOP_A_FSK_PREAMBLE_SIZE_MSB_PREAMBLE_SIZE
    RegDesc { addr: 0x5243, offs: 0, leng: 8, dflt: 20, flags: 0x04 }, // 106: TX_TOP_A_FSK_PREAMBLE_SIZE_LSB_PREAMBLE_SIZE
    RegDesc { addr: 0x5244, offs: 0, leng: 8, dflt: 26, flags: 0x04 }, // 107: TX_TOP_A_FSK_BIT_RATE_MSB_BIT_RATE
    RegDesc { addr: 0x5245, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 108: TX_TOP_A_FSK_BIT_RATE_LSB_BIT_RATE
    RegDesc { addr: 0x5246, offs: 5, leng: 3, dflt: 3, flags: 0x04 }, // 109: TX_TOP_A_FSK_MOD_FSK_REF_PATTERN_SIZE
    RegDesc { addr: 0x5246, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 110: TX_TOP_A_FSK_MOD_FSK_PREAMBLE_SEQ
    RegDesc { addr: 0x5246, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 111: TX_TOP_A_FSK_MOD_FSK_REF_PATTERN_EN
    RegDesc { addr: 0x5246, offs: 1, leng: 2, dflt: 0, flags: 0x04 }, // 112: TX_TOP_A_FSK_MOD_FSK_GAUSSIAN_SELECT_BT
    RegDesc { addr: 0x5246, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 113: TX_TOP_A_FSK_MOD_FSK_GAUSSIAN_EN
    RegDesc { addr: 0x5247, offs: 0, leng: 8, dflt: 151, flags: 0x04 }, // 114: TX_TOP_A_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN
    RegDesc { addr: 0x5248, offs: 0, leng: 8, dflt: 35, flags: 0x04 }, // 115: TX_TOP_A_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN
    RegDesc { addr: 0x5249, offs: 0, leng: 8, dflt: 82, flags: 0x04 }, // 116: TX_TOP_A_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN
    RegDesc { addr: 0x524A, offs: 0, leng: 8, dflt: 37, flags: 0x04 }, // 117: TX_TOP_A_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN
    RegDesc { addr: 0x524B, offs: 0, leng: 8, dflt: 86, flags: 0x04 }, // 118: TX_TOP_A_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN
    RegDesc { addr: 0x524C, offs: 0, leng: 8, dflt: 83, flags: 0x04 }, // 119: TX_TOP_A_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN
    RegDesc { addr: 0x524D, offs: 0, leng: 8, dflt: 101, flags: 0x04 }, // 120: TX_TOP_A_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN
    RegDesc { addr: 0x524E, offs: 0, leng: 8, dflt: 100, flags: 0x04 }, // 121: TX_TOP_A_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN
    RegDesc { addr: 0x524F, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 122: TX_TOP_A_DUMMY_GSFK_DUMMY
    RegDesc { addr: 0x5260, offs: 4, leng: 4, dflt: 5, flags: 0x04 }, // 123: TX_TOP_A_TXRX_CFG0_0_MODEM_BW
    RegDesc { addr: 0x5260, offs: 0, leng: 4, dflt: 7, flags: 0x04 }, // 124: TX_TOP_A_TXRX_CFG0_0_MODEM_SF
    RegDesc { addr: 0x5261, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 125: TX_TOP_A_TXRX_CFG0_1_PPM_OFFSET_HDR_CTRL
    RegDesc { addr: 0x5261, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 126: TX_TOP_A_TXRX_CFG0_1_PPM_OFFSET
    RegDesc { addr: 0x5261, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 127: TX_TOP_A_TXRX_CFG0_1_POST_PREAMBLE_GAP_LONG
    RegDesc { addr: 0x5261, offs: 0, leng: 3, dflt: 2, flags: 0x04 }, // 128: TX_TOP_A_TXRX_CFG0_1_CODING_RATE
    RegDesc { addr: 0x5262, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 129: TX_TOP_A_TXRX_CFG0_2_FINE_SYNCH_EN
    RegDesc { addr: 0x5262, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 130: TX_TOP_A_TXRX_CFG0_2_MODEM_EN
    RegDesc { addr: 0x5262, offs: 4, leng: 2, dflt: 2, flags: 0x04 }, // 131: TX_TOP_A_TXRX_CFG0_2_CADRXTX
    RegDesc { addr: 0x5262, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 132: TX_TOP_A_TXRX_CFG0_2_IMPLICIT_HEADER
    RegDesc { addr: 0x5262, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 133: TX_TOP_A_TXRX_CFG0_2_CRC_EN
    RegDesc { addr: 0x5263, offs: 0, leng: 8, dflt: 12, flags: 0x04 }, // 134: TX_TOP_A_TXRX_CFG0_3_PAYLOAD_LENGTH
    RegDesc { addr: 0x5264, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 135: TX_TOP_A_TXRX_CFG1_0_INT_STEP_ORIDE_EN
    RegDesc { addr: 0x5264, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 136: TX_TOP_A_TXRX_CFG1_0_INT_STEP_ORIDE
    RegDesc { addr: 0x5265, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 137: TX_TOP_A_TXRX_CFG1_1_MODEM_START
    RegDesc { addr: 0x5265, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 138: TX_TOP_A_TXRX_CFG1_1_HEADER_DIFF_MODE
    RegDesc { addr: 0x5265, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 139: TX_TOP_A_TXRX_CFG1_1_ZERO_PAD
    RegDesc { addr: 0x5266, offs: 0, leng: 8, dflt: 8, flags: 0x04 }, // 140: TX_TOP_A_TXRX_CFG1_2_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5267, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 141: TX_TOP_A_TXRX_CFG1_3_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5268, offs: 6, leng: 1, dflt: 1, flags: 0x04 }, // 142: TX_TOP_A_TXRX_CFG1_4_AUTO_ACK_INT_DELAY
    RegDesc { addr: 0x5268, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 143: TX_TOP_A_TXRX_CFG1_4_AUTO_ACK_RX
    RegDesc { addr: 0x5268, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 144: TX_TOP_A_TXRX_CFG1_4_AUTO_ACK_TX
    RegDesc { addr: 0x5269, offs: 4, leng: 3, dflt: 0, flags: 0x04 }, // 145: TX_TOP_A_TX_CFG0_0_CHIRP_LOWPASS
    RegDesc { addr: 0x5269, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 146: TX_TOP_A_TX_CFG0_0_PPM_OFFSET_SIG
    RegDesc { addr: 0x5269, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 147: TX_TOP_A_TX_CFG0_0_CONTCHIRP
    RegDesc { addr: 0x5269, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 148: TX_TOP_A_TX_CFG0_0_CHIRP_INVERT
    RegDesc { addr: 0x5269, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 149: TX_TOP_A_TX_CFG0_0_CONTINUOUS
    RegDesc { addr: 0x526A, offs: 0, leng: 6, dflt: 20, flags: 0x04 }, // 150: TX_TOP_A_TX_CFG0_1_POWER_RANGING
    RegDesc { addr: 0x526B, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 151: TX_TOP_A_TX_CFG1_0_FRAME_NB
    RegDesc { addr: 0x526C, offs: 5, leng: 2, dflt: 0, flags: 0x04 }, // 152: TX_TOP_A_TX_CFG1_1_HOP_CTRL
    RegDesc { addr: 0x526C, offs: 0, leng: 5, dflt: 10, flags: 0x04 }, // 153: TX_TOP_A_TX_CFG1_1_IFS
    RegDesc { addr: 0x526D, offs: 7, leng: 1, dflt: 1, flags: 0x04 }, // 154: TX_TOP_A_FRAME_SYNCH_0_AUTO_SCALE
    RegDesc { addr: 0x526D, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 155: TX_TOP_A_FRAME_SYNCH_0_DROP_ON_SYNCH
    RegDesc { addr: 0x526D, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 156: TX_TOP_A_FRAME_SYNCH_0_GAIN
    RegDesc { addr: 0x526D, offs: 0, leng: 5, dflt: 2, flags: 0x05 }, // 157: TX_TOP_A_FRAME_SYNCH_0_PEAK1_POS
    RegDesc { addr: 0x526E, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 158: TX_TOP_A_FRAME_SYNCH_1_FINETIME_ON_LAST
    RegDesc { addr: 0x526E, offs: 5, leng: 2, dflt: 3, flags: 0x04 }, // 159: TX_TOP_A_FRAME_SYNCH_1_TIMEOUT_OPT
    RegDesc { addr: 0x526E, offs: 0, leng: 5, dflt: 4, flags: 0x05 }, // 160: TX_TOP_A_FRAME_SYNCH_1_PEAK2_POS
    RegDesc { addr: 0x526F, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 161: TX_TOP_A_LORA_TX_STATE_STATUS
    RegDesc { addr: 0x5270, offs: 2, leng: 1, dflt: 0, flags: 0x00 }, // 162: TX_TOP_A_LORA_TX_FLAG_FRAME_DONE
    RegDesc { addr: 0x5270, offs: 1, leng: 1, dflt: 0, flags: 0x00 }, // 163: TX_TOP_A_LORA_TX_FLAG_CONT_DONE
    RegDesc { addr: 0x5270, offs: 0, leng: 1, dflt: 0, flags: 0x00 }, // 164: TX_TOP_A_LORA_TX_FLAG_PLD_DONE
    RegDesc { addr: 0x5271, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 165: TX_TOP_A_DUMMY_LORA_DUMMY
    RegDesc { addr: 0x5400, offs: 3, leng: 1, dflt: 0, flags: 0x00 }, // 166: TX_TOP_B_TX_TRIG_TX_FSM_CLR
    RegDesc { addr: 0x5400, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 167: TX_TOP_B_TX_TRIG_TX_TRIG_GPS
    RegDesc { addr: 0x5400, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 168: TX_TOP_B_TX_TRIG_TX_TRIG_DELAYED
    RegDesc { addr: 0x5400, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 169: TX_TOP_B_TX_TRIG_TX_TRIG_IMMEDIATE
    RegDesc { addr: 0x5401, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 170: TX_TOP_B_TIMER_TRIG_BYTE3_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5402, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 171: TX_TOP_B_TIMER_TRIG_BYTE2_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5403, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 172: TX_TOP_B_TIMER_TRIG_BYTE1_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5404, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 173: TX_TOP_B_TIMER_TRIG_BYTE0_TIMER_DELAYED_TRIG
    RegDesc { addr: 0x5405, offs: 0, leng: 8, dflt: 187, flags: 0x04 }, // 174: TX_TOP_B_TX_START_DELAY_MSB_TX_START_DELAY
    RegDesc { addr: 0x5406, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 175: TX_TOP_B_TX_START_DELAY_LSB_TX_START_DELAY
    RegDesc { addr: 0x5407, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 176: TX_TOP_B_TX_CTRL_WRITE_BUFFER
    RegDesc { addr: 0x5408, offs: 0, leng: 3, dflt: 1, flags: 0x04 }, // 177: TX_TOP_B_TX_RAMP_DURATION_TX_RAMP_DURATION
    RegDesc { addr: 0x5409, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 178: TX_TOP_B_GEN_CFG_0_MODULATION_TYPE
    RegDesc { addr: 0x540A, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 179: TX_TOP_B_TEST_0_TX_ACTIVE_CTRL
    RegDesc { addr: 0x540A, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 180: TX_TOP_B_TEST_0_TX_ACTIVE_SEL
    RegDesc { addr: 0x540B, offs: 1, leng: 1, dflt: 0, flags: 0x00 }, // 181: TX_TOP_B_TX_FLAG_TX_TIMEOUT
    RegDesc { addr: 0x540B, offs: 0, leng: 1, dflt: 0, flags: 0x00 }, // 182: TX_TOP_B_TX_FLAG_PKT_DONE
    RegDesc { addr: 0x540C, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 183: TX_TOP_B_AGC_TX_BW_AGC_TX_BW
    RegDesc { addr: 0x540D, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 184: TX_TOP_B_AGC_TX_PWR_AGC_TX_PWR
    RegDesc { addr: 0x540E, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 185: TX_TOP_B_TIMEOUT_CNT_BYTE_2_TIMEOUT_CNT
    RegDesc { addr: 0x540F, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 186: TX_TOP_B_TIMEOUT_CNT_BYTE_1_TIMEOUT_CNT
    RegDesc { addr: 0x5410, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 187: TX_TOP_B_TIMEOUT_CNT_BYTE_0_TIMEOUT_CNT
    RegDesc { addr: 0x5411, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 188: TX_TOP_B_TX_FSM_STATUS_TX_STATUS
    RegDesc { addr: 0x5412, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 189: TX_TOP_B_DUMMY_CONTROL_DUMMY
    RegDesc { addr: 0x5420, offs: 5, leng: 3, dflt: 0, flags: 0x04 }, // 190: TX_TOP_B_TX_RFFE_IF_CTRL_PLL_DIV_CTRL
    RegDesc { addr: 0x5420, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 191: TX_TOP_B_TX_RFFE_IF_CTRL_TX_CLK_EDGE
    RegDesc { addr: 0x5420, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 192: TX_TOP_B_TX_RFFE_IF_CTRL_TX_MODE
    RegDesc { addr: 0x5420, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 193: TX_TOP_B_TX_RFFE_IF_CTRL_TX_IF_DST
    RegDesc { addr: 0x5420, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 194: TX_TOP_B_TX_RFFE_IF_CTRL_TX_IF_SRC
    RegDesc { addr: 0x5421, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 195: TX_TOP_B_TX_RFFE_IF_CTRL2_SX125X_IQ_INVERT
    RegDesc { addr: 0x5421, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 196: TX_TOP_B_TX_RFFE_IF_CTRL2_PLL_DIV_CTRL_AGC
    RegDesc { addr: 0x5422, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 197: TX_TOP_B_TX_RFFE_IF_IQ_GAIN_IQ_GAIN
    RegDesc { addr: 0x5423, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 198: TX_TOP_B_TX_RFFE_IF_I_OFFSET_I_OFFSET
    RegDesc { addr: 0x5424, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 199: TX_TOP_B_TX_RFFE_IF_Q_OFFSET_Q_OFFSET
    RegDesc { addr: 0x5425, offs: 0, leng: 8, dflt: 108, flags: 0x04 }, // 200: TX_TOP_B_TX_RFFE_IF_FREQ_RF_H_FREQ_RF
    RegDesc { addr: 0x5426, offs: 0, leng: 8, dflt: 144, flags: 0x04 }, // 201: TX_TOP_B_TX_RFFE_IF_FREQ_RF_M_FREQ_RF
    RegDesc { addr: 0x5427, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 202: TX_TOP_B_TX_RFFE_IF_FREQ_RF_L_FREQ_RF
    RegDesc { addr: 0x5428, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 203: TX_TOP_B_TX_RFFE_IF_FREQ_DEV_H_FREQ_DEV
    RegDesc { addr: 0x5429, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 204: TX_TOP_B_TX_RFFE_IF_FREQ_DEV_L_FREQ_DEV
    RegDesc { addr: 0x542A, offs: 0, leng: 8, dflt: 64, flags: 0x04 }, // 205: TX_TOP_B_TX_RFFE_IF_TEST_MOD_FREQ
    RegDesc { addr: 0x542B, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 206: TX_TOP_B_DUMMY_MODULATOR_DUMMY
    RegDesc { addr: 0x5440, offs: 0, leng: 8, dflt: 15, flags: 0x04 }, // 207: TX_TOP_B_FSK_PKT_LEN_PKT_LENGTH
    RegDesc { addr: 0x5441, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 208: TX_TOP_B_FSK_CFG_0_TX_CONT
    RegDesc { addr: 0x5441, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 209: TX_TOP_B_FSK_CFG_0_CRC_IBM
    RegDesc { addr: 0x5441, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 210: TX_TOP_B_FSK_CFG_0_DCFREE_ENC
    RegDesc { addr: 0x5441, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 211: TX_TOP_B_FSK_CFG_0_CRC_EN
    RegDesc { addr: 0x5441, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 212: TX_TOP_B_FSK_CFG_0_PKT_MODE
    RegDesc { addr: 0x5442, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 213: TX_TOP_B_FSK_PREAMBLE_SIZE_MSB_PREAMBLE_SIZE
    RegDesc { addr: 0x5443, offs: 0, leng: 8, dflt: 20, flags: 0x04 }, // 214: TX_TOP_B_FSK_PREAMBLE_SIZE_LSB_PREAMBLE_SIZE
    RegDesc { addr: 0x5444, offs: 0, leng: 8, dflt: 26, flags: 0x04 }, // 215: TX_TOP_B_FSK_BIT_RATE_MSB_BIT_RATE
    RegDesc { addr: 0x5445, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 216: TX_TOP_B_FSK_BIT_RATE_LSB_BIT_RATE
    RegDesc { addr: 0x5446, offs: 5, leng: 3, dflt: 3, flags: 0x04 }, // 217: TX_TOP_B_FSK_MOD_FSK_REF_PATTERN_SIZE
    RegDesc { addr: 0x5446, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 218: TX_TOP_B_FSK_MOD_FSK_PREAMBLE_SEQ
    RegDesc { addr: 0x5446, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 219: TX_TOP_B_FSK_MOD_FSK_REF_PATTERN_EN
    RegDesc { addr: 0x5446, offs: 1, leng: 2, dflt: 0, flags: 0x04 }, // 220: TX_TOP_B_FSK_MOD_FSK_GAUSSIAN_SELECT_BT
    RegDesc { addr: 0x5446, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 221: TX_TOP_B_FSK_MOD_FSK_GAUSSIAN_EN
    RegDesc { addr: 0x5447, offs: 0, leng: 8, dflt: 151, flags: 0x04 }, // 222: TX_TOP_B_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN
    RegDesc { addr: 0x5448, offs: 0, leng: 8, dflt: 35, flags: 0x04 }, // 223: TX_TOP_B_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN
    RegDesc { addr: 0x5449, offs: 0, leng: 8, dflt: 82, flags: 0x04 }, // 224: TX_TOP_B_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN
    RegDesc { addr: 0x544A, offs: 0, leng: 8, dflt: 37, flags: 0x04 }, // 225: TX_TOP_B_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN
    RegDesc { addr: 0x544B, offs: 0, leng: 8, dflt: 86, flags: 0x04 }, // 226: TX_TOP_B_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN
    RegDesc { addr: 0x544C, offs: 0, leng: 8, dflt: 83, flags: 0x04 }, // 227: TX_TOP_B_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN
    RegDesc { addr: 0x544D, offs: 0, leng: 8, dflt: 101, flags: 0x04 }, // 228: TX_TOP_B_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN
    RegDesc { addr: 0x544E, offs: 0, leng: 8, dflt: 100, flags: 0x04 }, // 229: TX_TOP_B_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN
    RegDesc { addr: 0x544F, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 230: TX_TOP_B_DUMMY_GSFK_DUMMY
    RegDesc { addr: 0x5460, offs: 4, leng: 4, dflt: 5, flags: 0x04 }, // 231: TX_TOP_B_TXRX_CFG0_0_MODEM_BW
    RegDesc { addr: 0x5460, offs: 0, leng: 4, dflt: 7, flags: 0x04 }, // 232: TX_TOP_B_TXRX_CFG0_0_MODEM_SF
    RegDesc { addr: 0x5461, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 233: TX_TOP_B_TXRX_CFG0_1_PPM_OFFSET_HDR_CTRL
    RegDesc { addr: 0x5461, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 234: TX_TOP_B_TXRX_CFG0_1_PPM_OFFSET
    RegDesc { addr: 0x5461, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 235: TX_TOP_B_TXRX_CFG0_1_POST_PREAMBLE_GAP_LONG
    RegDesc { addr: 0x5461, offs: 0, leng: 3, dflt: 2, flags: 0x04 }, // 236: TX_TOP_B_TXRX_CFG0_1_CODING_RATE
    RegDesc { addr: 0x5462, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 237: TX_TOP_B_TXRX_CFG0_2_FINE_SYNCH_EN
    RegDesc { addr: 0x5462, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 238: TX_TOP_B_TXRX_CFG0_2_MODEM_EN
    RegDesc { addr: 0x5462, offs: 4, leng: 2, dflt: 2, flags: 0x04 }, // 239: TX_TOP_B_TXRX_CFG0_2_CADRXTX
    RegDesc { addr: 0x5462, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 240: TX_TOP_B_TXRX_CFG0_2_IMPLICIT_HEADER
    RegDesc { addr: 0x5462, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 241: TX_TOP_B_TXRX_CFG0_2_CRC_EN
    RegDesc { addr: 0x5463, offs: 0, leng: 8, dflt: 12, flags: 0x04 }, // 242: TX_TOP_B_TXRX_CFG0_3_PAYLOAD_LENGTH
    RegDesc { addr: 0x5464, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 243: TX_TOP_B_TXRX_CFG1_0_INT_STEP_ORIDE_EN
    RegDesc { addr: 0x5464, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 244: TX_TOP_B_TXRX_CFG1_0_INT_STEP_ORIDE
    RegDesc { addr: 0x5465, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 245: TX_TOP_B_TXRX_CFG1_1_MODEM_START
    RegDesc { addr: 0x5465, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 246: TX_TOP_B_TXRX_CFG1_1_HEADER_DIFF_MODE
    RegDesc { addr: 0x5465, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 247: TX_TOP_B_TXRX_CFG1_1_ZERO_PAD
    RegDesc { addr: 0x5466, offs: 0, leng: 8, dflt: 8, flags: 0x04 }, // 248: TX_TOP_B_TXRX_CFG1_2_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5467, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 249: TX_TOP_B_TXRX_CFG1_3_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5468, offs: 6, leng: 1, dflt: 1, flags: 0x04 }, // 250: TX_TOP_B_TXRX_CFG1_4_AUTO_ACK_INT_DELAY
    RegDesc { addr: 0x5468, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 251: TX_TOP_B_TXRX_CFG1_4_AUTO_ACK_RX
    RegDesc { addr: 0x5468, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 252: TX_TOP_B_TXRX_CFG1_4_AUTO_ACK_TX
    RegDesc { addr: 0x5469, offs: 4, leng: 3, dflt: 0, flags: 0x04 }, // 253: TX_TOP_B_TX_CFG0_0_CHIRP_LOWPASS
    RegDesc { addr: 0x5469, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 254: TX_TOP_B_TX_CFG0_0_PPM_OFFSET_SIG
    RegDesc { addr: 0x5469, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 255: TX_TOP_B_TX_CFG0_0_CONTCHIRP
    RegDesc { addr: 0x5469, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 256: TX_TOP_B_TX_CFG0_0_CHIRP_INVERT
    RegDesc { addr: 0x5469, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 257: TX_TOP_B_TX_CFG0_0_CONTINUOUS
    RegDesc { addr: 0x546A, offs: 0, leng: 6, dflt: 20, flags: 0x04 }, // 258: TX_TOP_B_TX_CFG0_1_POWER_RANGING
    RegDesc { addr: 0x546B, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 259: TX_TOP_B_TX_CFG1_0_FRAME_NB
    RegDesc { addr: 0x546C, offs: 5, leng: 2, dflt: 0, flags: 0x04 }, // 260: TX_TOP_B_TX_CFG1_1_HOP_CTRL
    RegDesc { addr: 0x546C, offs: 0, leng: 5, dflt: 10, flags: 0x04 }, // 261: TX_TOP_B_TX_CFG1_1_IFS
    RegDesc { addr: 0x546D, offs: 7, leng: 1, dflt: 1, flags: 0x04 }, // 262: TX_TOP_B_FRAME_SYNCH_0_AUTO_SCALE
    RegDesc { addr: 0x546D, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 263: TX_TOP_B_FRAME_SYNCH_0_DROP_ON_SYNCH
    RegDesc { addr: 0x546D, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 264: TX_TOP_B_FRAME_SYNCH_0_GAIN
    RegDesc { addr: 0x546D, offs: 0, leng: 5, dflt: 2, flags: 0x05 }, // 265: TX_TOP_B_FRAME_SYNCH_0_PEAK1_POS
    RegDesc { addr: 0x546E, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 266: TX_TOP_B_FRAME_SYNCH_1_FINETIME_ON_LAST
    RegDesc { addr: 0x546E, offs: 5, leng: 2, dflt: 3, flags: 0x04 }, // 267: TX_TOP_B_FRAME_SYNCH_1_TIMEOUT_OPT
    RegDesc { addr: 0x546E, offs: 0, leng: 5, dflt: 4, flags: 0x05 }, // 268: TX_TOP_B_FRAME_SYNCH_1_PEAK2_POS
    RegDesc { addr: 0x546F, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 269: TX_TOP_B_LORA_TX_STATE_STATUS
    RegDesc { addr: 0x5470, offs: 2, leng: 1, dflt: 0, flags: 0x00 }, // 270: TX_TOP_B_LORA_TX_FLAG_FRAME_DONE
    RegDesc { addr: 0x5470, offs: 1, leng: 1, dflt: 0, flags: 0x00 }, // 271: TX_TOP_B_LORA_TX_FLAG_CONT_DONE
    RegDesc { addr: 0x5470, offs: 0, leng: 1, dflt: 0, flags: 0x00 }, // 272: TX_TOP_B_LORA_TX_FLAG_PLD_DONE
    RegDesc { addr: 0x5471, offs: 3, leng: 1, dflt: 0, flags: 0x06 }, // 273: TX_TOP_B_DUMMY_LORA_DUMMY
    RegDesc { addr: 0x5640, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 274: GPIO_GPIO_DIR_H_DIRECTION
    RegDesc { addr: 0x5641, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 275: GPIO_GPIO_DIR_L_DIRECTION
    RegDesc { addr: 0x5642, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 276: GPIO_GPIO_OUT_H_OUT_VALUE
    RegDesc { addr: 0x5643, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 277: GPIO_GPIO_OUT_L_OUT_VALUE
    RegDesc { addr: 0x5644, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 278: GPIO_GPIO_IN_H_IN_VALUE
    RegDesc { addr: 0x5645, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 279: GPIO_GPIO_IN_L_IN_VALUE
    RegDesc { addr: 0x5646, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 280: GPIO_GPIO_PD_H_PD_VALUE
    RegDesc { addr: 0x5647, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 281: GPIO_GPIO_PD_L_PD_VALUE
    RegDesc { addr: 0x5648, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 282: GPIO_GPIO_SEL_0_SELECTION
    RegDesc { addr: 0x5649, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 283: GPIO_GPIO_SEL_1_SELECTION
    RegDesc { addr: 0x564A, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 284: GPIO_GPIO_SEL_2_SELECTION
    RegDesc { addr: 0x564B, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 285: GPIO_GPIO_SEL_3_SELECTION
    RegDesc { addr: 0x564C, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 286: GPIO_GPIO_SEL_4_SELECTION
    RegDesc { addr: 0x564D, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 287: GPIO_GPIO_SEL_5_SELECTION
    RegDesc { addr: 0x564E, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 288: GPIO_GPIO_SEL_6_SELECTION
    RegDesc { addr: 0x564F, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 289: GPIO_GPIO_SEL_7_SELECTION
    RegDesc { addr: 0x5650, offs: 1, leng: 4, dflt: 0, flags: 0x04 }, // 290: GPIO_GPIO_SEL_8_11_GPIO_11_9_SEL
    RegDesc { addr: 0x5650, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 291: GPIO_GPIO_SEL_8_11_GPIO_8_SEL
    RegDesc { addr: 0x5651, offs: 5, leng: 1, dflt: 0, flags: 0x00 }, // 292: GPIO_HOST_IRQ_TX_TIMEOUT_B
    RegDesc { addr: 0x5651, offs: 4, leng: 1, dflt: 0, flags: 0x00 }, // 293: GPIO_HOST_IRQ_TX_TIMEOUT_A
    RegDesc { addr: 0x5651, offs: 3, leng: 1, dflt: 0, flags: 0x00 }, // 294: GPIO_HOST_IRQ_TX_DONE_B
    RegDesc { addr: 0x5651, offs: 2, leng: 1, dflt: 0, flags: 0x00 }, // 295: GPIO_HOST_IRQ_TX_DONE_A
    RegDesc { addr: 0x5651, offs: 1, leng: 1, dflt: 0, flags: 0x00 }, // 296: GPIO_HOST_IRQ_TIMESTAMP
    RegDesc { addr: 0x5651, offs: 0, leng: 1, dflt: 0, flags: 0x00 }, // 297: GPIO_HOST_IRQ_RX_BUFFER_WATERMARK
    RegDesc { addr: 0x5652, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 298: GPIO_HOST_IRQ_EN_TX_TIMEOUT_B
    RegDesc { addr: 0x5652, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 299: GPIO_HOST_IRQ_EN_TX_TIMEOUT_A
    RegDesc { addr: 0x5652, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 300: GPIO_HOST_IRQ_EN_TX_DONE_B
    RegDesc { addr: 0x5652, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 301: GPIO_HOST_IRQ_EN_TX_DONE_A
    RegDesc { addr: 0x5652, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 302: GPIO_HOST_IRQ_EN_TIMESTAMP
    RegDesc { addr: 0x5652, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 303: GPIO_HOST_IRQ_EN_RX_BUFFER_WATERMARK
    RegDesc { addr: 0x5653, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 304: GPIO_DUMMY_DUMMY
    RegDesc { addr: 0x6100, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 305: TIMESTAMP_GPS_CTRL_GPS_POL
    RegDesc { addr: 0x6100, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 306: TIMESTAMP_GPS_CTRL_GPS_EN
    RegDesc { addr: 0x6101, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 307: TIMESTAMP_TIMESTAMP_PPS_MSB2_TIMESTAMP_PPS
    RegDesc { addr: 0x6102, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 308: TIMESTAMP_TIMESTAMP_PPS_MSB1_TIMESTAMP_PPS
    RegDesc { addr: 0x6103, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 309: TIMESTAMP_TIMESTAMP_PPS_LSB2_TIMESTAMP_PPS
    RegDesc { addr: 0x6104, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 310: TIMESTAMP_TIMESTAMP_PPS_LSB1_TIMESTAMP_PPS
    RegDesc { addr: 0x6105, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 311: TIMESTAMP_TIMESTAMP_MSB2_TIMESTAMP
    RegDesc { addr: 0x6106, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 312: TIMESTAMP_TIMESTAMP_MSB1_TIMESTAMP
    RegDesc { addr: 0x6107, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 313: TIMESTAMP_TIMESTAMP_LSB2_TIMESTAMP
    RegDesc { addr: 0x6108, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 314: TIMESTAMP_TIMESTAMP_LSB1_TIMESTAMP
    RegDesc { addr: 0x6109, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 315: TIMESTAMP_TIMESTAMP_SET3_TIMESTAMP
    RegDesc { addr: 0x610A, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 316: TIMESTAMP_TIMESTAMP_SET2_TIMESTAMP
    RegDesc { addr: 0x610B, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 317: TIMESTAMP_TIMESTAMP_SET1_TIMESTAMP
    RegDesc { addr: 0x610C, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 318: TIMESTAMP_TIMESTAMP_SET0_TIMESTAMP
    RegDesc { addr: 0x610D, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 319: TIMESTAMP_TIMESTAMP_IRQ_3_TIMESTAMP
    RegDesc { addr: 0x610E, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 320: TIMESTAMP_TIMESTAMP_IRQ_2_TIMESTAMP
    RegDesc { addr: 0x610F, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 321: TIMESTAMP_TIMESTAMP_IRQ_1_TIMESTAMP
    RegDesc { addr: 0x6110, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 322: TIMESTAMP_TIMESTAMP_IRQ_0_TIMESTAMP
    RegDesc { addr: 0x6111, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 323: TIMESTAMP_DUMMY_DUMMY
    RegDesc { addr: 0x5800, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 324: RX_TOP_FREQ_0_MSB_IF_FREQ_0
    RegDesc { addr: 0x5801, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 325: RX_TOP_FREQ_0_LSB_IF_FREQ_0
    RegDesc { addr: 0x5802, offs: 0, leng: 5, dflt: 1, flags: 0x04 }, // 326: RX_TOP_FREQ_1_MSB_IF_FREQ_1
    RegDesc { addr: 0x5803, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 327: RX_TOP_FREQ_1_LSB_IF_FREQ_1
    RegDesc { addr: 0x5804, offs: 0, leng: 5, dflt: 30, flags: 0x04 }, // 328: RX_TOP_FREQ_2_MSB_IF_FREQ_2
    RegDesc { addr: 0x5805, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 329: RX_TOP_FREQ_2_LSB_IF_FREQ_2
    RegDesc { addr: 0x5806, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 330: RX_TOP_FREQ_3_MSB_IF_FREQ_3
    RegDesc { addr: 0x5807, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 331: RX_TOP_FREQ_3_LSB_IF_FREQ_3
    RegDesc { addr: 0x5808, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 332: RX_TOP_FREQ_4_MSB_IF_FREQ_4
    RegDesc { addr: 0x5809, offs: 0, leng: 8, dflt: 50, flags: 0x04 }, // 333: RX_TOP_FREQ_4_LSB_IF_FREQ_4
    RegDesc { addr: 0x580A, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 334: RX_TOP_FREQ_5_MSB_IF_FREQ_5
    RegDesc { addr: 0x580B, offs: 0, leng: 8, dflt: 60, flags: 0x04 }, // 335: RX_TOP_FREQ_5_LSB_IF_FREQ_5
    RegDesc { addr: 0x580C, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 336: RX_TOP_FREQ_6_MSB_IF_FREQ_6
    RegDesc { addr: 0x580D, offs: 0, leng: 8, dflt: 70, flags: 0x04 }, // 337: RX_TOP_FREQ_6_LSB_IF_FREQ_6
    RegDesc { addr: 0x580E, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 338: RX_TOP_FREQ_7_MSB_IF_FREQ_7
    RegDesc { addr: 0x580F, offs: 0, leng: 8, dflt: 80, flags: 0x04 }, // 339: RX_TOP_FREQ_7_LSB_IF_FREQ_7
    RegDesc { addr: 0x5810, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 340: RX_TOP_RADIO_SELECT_RADIO_SELECT
    RegDesc { addr: 0x5811, offs: 3, leng: 5, dflt: 7, flags: 0x04 }, // 341: RX_TOP_RSSI_CONTROL_RSSI_FILTER_ALPHA
    RegDesc { addr: 0x5811, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 342: RX_TOP_RSSI_CONTROL_SELECT_RSSI
    RegDesc { addr: 0x5812, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 343: RX_TOP_RSSI_DEF_VALUE_CHAN_RSSI_DEF_VALUE
    RegDesc { addr: 0x5813, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 344: RX_TOP_CHANN_DAGC_CFG1_CHAN_DAGC_THRESHOLD_HIGH
    RegDesc { addr: 0x5814, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 345: RX_TOP_CHANN_DAGC_CFG2_CHAN_DAGC_THRESHOLD_LOW
    RegDesc { addr: 0x5815, offs: 4, leng: 4, dflt: 15, flags: 0x04 }, // 346: RX_TOP_CHANN_DAGC_CFG3_CHAN_DAGC_MAX_ATTEN
    RegDesc { addr: 0x5815, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 347: RX_TOP_CHANN_DAGC_CFG3_CHAN_DAGC_MIN_ATTEN
    RegDesc { addr: 0x5816, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 348: RX_TOP_CHANN_DAGC_CFG4_CHAN_DAGC_STEP
    RegDesc { addr: 0x5817, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 349: RX_TOP_CHANN_DAGC_CFG5_CHAN_DAGC_MODE
    RegDesc { addr: 0x5818, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 350: RX_TOP_RSSI_VALUE_CHAN_RSSI
    RegDesc { addr: 0x5819, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 351: RX_TOP_GAIN_CONTROL_CHAN_GAIN_VALID
    RegDesc { addr: 0x5819, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 352: RX_TOP_GAIN_CONTROL_CHAN_GAIN
    RegDesc { addr: 0x581A, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 353: RX_TOP_CLK_CONTROL_CHAN_CLK_EN
    RegDesc { addr: 0x581B, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 354: RX_TOP_DUMMY0_DUMMY0
    RegDesc { addr: 0x5820, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 355: RX_TOP_CORR_CLOCK_ENABLE_CLK_EN
    RegDesc { addr: 0x5821, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 356: RX_TOP_CORRELATOR_EN_CORR_EN
    RegDesc { addr: 0x5822, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 357: RX_TOP_CORRELATOR_SF_EN_CORR_SF_EN
    RegDesc { addr: 0x5823, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 358: RX_TOP_CORRELATOR_ENABLE_ONLY_FIRST_DET_EDGE_ENABLE_ONLY_FIRST_DET_EDGE
    RegDesc { addr: 0x5824, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 359: RX_TOP_CORRELATOR_ENABLE_ACC_CLEAR_ENABLE_CORR_ACC_CLEAR
    RegDesc { addr: 0x5825, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 360: RX_TOP_SF5_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x5825, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 361: RX_TOP_SF5_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x5825, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 362: RX_TOP_SF5_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x5825, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 363: RX_TOP_SF5_CFG1_ACC_COEFF
    RegDesc { addr: 0x5825, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 364: RX_TOP_SF5_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x5825, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 365: RX_TOP_SF5_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x5826, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 366: RX_TOP_SF5_CFG2_ACC_MIN2
    RegDesc { addr: 0x5826, offs: 0, leng: 7, dflt: 55, flags: 0x04 }, // 367: RX_TOP_SF5_CFG2_ACC_PNR
    RegDesc { addr: 0x5827, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 368: RX_TOP_SF5_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x5828, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 369: RX_TOP_SF5_CFG4_MSP_PNR
    RegDesc { addr: 0x5829, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 370: RX_TOP_SF5_CFG5_MSP2_PNR
    RegDesc { addr: 0x582A, offs: 3, leng: 3, dflt: 5, flags: 0x04 }, // 371: RX_TOP_SF5_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x582A, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 372: RX_TOP_SF5_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x582A, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 373: RX_TOP_SF5_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x582B, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 374: RX_TOP_SF5_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x582B, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 375: RX_TOP_SF5_CFG7_NOISE_COEFF
    RegDesc { addr: 0x582C, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 376: RX_TOP_SF6_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x582C, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 377: RX_TOP_SF6_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x582C, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 378: RX_TOP_SF6_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x582C, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 379: RX_TOP_SF6_CFG1_ACC_COEFF
    RegDesc { addr: 0x582C, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 380: RX_TOP_SF6_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x582C, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 381: RX_TOP_SF6_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x582D, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 382: RX_TOP_SF6_CFG2_ACC_MIN2
    RegDesc { addr: 0x582D, offs: 0, leng: 7, dflt: 55, flags: 0x04 }, // 383: RX_TOP_SF6_CFG2_ACC_PNR
    RegDesc { addr: 0x582E, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 384: RX_TOP_SF6_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x582F, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 385: RX_TOP_SF6_CFG4_MSP_PNR
    RegDesc { addr: 0x5830, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 386: RX_TOP_SF6_CFG5_MSP2_PNR
    RegDesc { addr: 0x5831, offs: 3, leng: 3, dflt: 4, flags: 0x04 }, // 387: RX_TOP_SF6_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x5831, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 388: RX_TOP_SF6_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x5831, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 389: RX_TOP_SF6_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x5832, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 390: RX_TOP_SF6_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x5832, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 391: RX_TOP_SF6_CFG7_NOISE_COEFF
    RegDesc { addr: 0x5833, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 392: RX_TOP_SF7_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x5833, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 393: RX_TOP_SF7_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x5833, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 394: RX_TOP_SF7_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x5833, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 395: RX_TOP_SF7_CFG1_ACC_COEFF
    RegDesc { addr: 0x5833, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 396: RX_TOP_SF7_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x5833, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 397: RX_TOP_SF7_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x5834, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 398: RX_TOP_SF7_CFG2_ACC_MIN2
    RegDesc { addr: 0x5834, offs: 0, leng: 7, dflt: 55, flags: 0x04 }, // 399: RX_TOP_SF7_CFG2_ACC_PNR
    RegDesc { addr: 0x5835, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 400: RX_TOP_SF7_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x5836, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 401: RX_TOP_SF7_CFG4_MSP_PNR
    RegDesc { addr: 0x5837, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 402: RX_TOP_SF7_CFG5_MSP2_PNR
    RegDesc { addr: 0x5838, offs: 3, leng: 3, dflt: 3, flags: 0x04 }, // 403: RX_TOP_SF7_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x5838, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 404: RX_TOP_SF7_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x5838, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 405: RX_TOP_SF7_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x5839, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 406: RX_TOP_SF7_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x5839, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 407: RX_TOP_SF7_CFG7_NOISE_COEFF
    RegDesc { addr: 0x583A, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 408: RX_TOP_SF8_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x583A, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 409: RX_TOP_SF8_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x583A, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 410: RX_TOP_SF8_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x583A, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 411: RX_TOP_SF8_CFG1_ACC_COEFF
    RegDesc { addr: 0x583A, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 412: RX_TOP_SF8_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x583A, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 413: RX_TOP_SF8_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x583B, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 414: RX_TOP_SF8_CFG2_ACC_MIN2
    RegDesc { addr: 0x583B, offs: 0, leng: 7, dflt: 56, flags: 0x04 }, // 415: RX_TOP_SF8_CFG2_ACC_PNR
    RegDesc { addr: 0x583C, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 416: RX_TOP_SF8_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x583D, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 417: RX_TOP_SF8_CFG4_MSP_PNR
    RegDesc { addr: 0x583E, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 418: RX_TOP_SF8_CFG5_MSP2_PNR
    RegDesc { addr: 0x583F, offs: 3, leng: 3, dflt: 3, flags: 0x04 }, // 419: RX_TOP_SF8_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x583F, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 420: RX_TOP_SF8_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x583F, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 421: RX_TOP_SF8_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x5840, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 422: RX_TOP_SF8_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x5840, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 423: RX_TOP_SF8_CFG7_NOISE_COEFF
    RegDesc { addr: 0x5841, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 424: RX_TOP_SF9_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x5841, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 425: RX_TOP_SF9_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x5841, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 426: RX_TOP_SF9_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x5841, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 427: RX_TOP_SF9_CFG1_ACC_COEFF
    RegDesc { addr: 0x5841, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 428: RX_TOP_SF9_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x5841, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 429: RX_TOP_SF9_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x5842, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 430: RX_TOP_SF9_CFG2_ACC_MIN2
    RegDesc { addr: 0x5842, offs: 0, leng: 7, dflt: 58, flags: 0x04 }, // 431: RX_TOP_SF9_CFG2_ACC_PNR
    RegDesc { addr: 0x5843, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 432: RX_TOP_SF9_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x5844, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 433: RX_TOP_SF9_CFG4_MSP_PNR
    RegDesc { addr: 0x5845, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 434: RX_TOP_SF9_CFG5_MSP2_PNR
    RegDesc { addr: 0x5846, offs: 3, leng: 3, dflt: 3, flags: 0x04 }, // 435: RX_TOP_SF9_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x5846, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 436: RX_TOP_SF9_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x5846, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 437: RX_TOP_SF9_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x5847, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 438: RX_TOP_SF9_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x5847, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 439: RX_TOP_SF9_CFG7_NOISE_COEFF
    RegDesc { addr: 0x5848, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 440: RX_TOP_SF10_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x5848, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 441: RX_TOP_SF10_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x5848, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 442: RX_TOP_SF10_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x5848, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 443: RX_TOP_SF10_CFG1_ACC_COEFF
    RegDesc { addr: 0x5848, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 444: RX_TOP_SF10_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x5848, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 445: RX_TOP_SF10_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x5849, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 446: RX_TOP_SF10_CFG2_ACC_MIN2
    RegDesc { addr: 0x5849, offs: 0, leng: 7, dflt: 60, flags: 0x04 }, // 447: RX_TOP_SF10_CFG2_ACC_PNR
    RegDesc { addr: 0x584A, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 448: RX_TOP_SF10_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x584B, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 449: RX_TOP_SF10_CFG4_MSP_PNR
    RegDesc { addr: 0x584C, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 450: RX_TOP_SF10_CFG5_MSP2_PNR
    RegDesc { addr: 0x584D, offs: 3, leng: 3, dflt: 3, flags: 0x04 }, // 451: RX_TOP_SF10_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x584D, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 452: RX_TOP_SF10_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x584D, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 453: RX_TOP_SF10_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x584E, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 454: RX_TOP_SF10_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x584E, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 455: RX_TOP_SF10_CFG7_NOISE_COEFF
    RegDesc { addr: 0x584F, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 456: RX_TOP_SF11_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x584F, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 457: RX_TOP_SF11_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x584F, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 458: RX_TOP_SF11_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x584F, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 459: RX_TOP_SF11_CFG1_ACC_COEFF
    RegDesc { addr: 0x584F, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 460: RX_TOP_SF11_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x584F, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 461: RX_TOP_SF11_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x5850, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 462: RX_TOP_SF11_CFG2_ACC_MIN2
    RegDesc { addr: 0x5850, offs: 0, leng: 7, dflt: 60, flags: 0x04 }, // 463: RX_TOP_SF11_CFG2_ACC_PNR
    RegDesc { addr: 0x5851, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 464: RX_TOP_SF11_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x5852, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 465: RX_TOP_SF11_CFG4_MSP_PNR
    RegDesc { addr: 0x5853, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 466: RX_TOP_SF11_CFG5_MSP2_PNR
    RegDesc { addr: 0x5854, offs: 3, leng: 3, dflt: 3, flags: 0x04 }, // 467: RX_TOP_SF11_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x5854, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 468: RX_TOP_SF11_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x5854, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 469: RX_TOP_SF11_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x5855, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 470: RX_TOP_SF11_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x5855, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 471: RX_TOP_SF11_CFG7_NOISE_COEFF
    RegDesc { addr: 0x5856, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 472: RX_TOP_SF12_CFG1_ACC_WIN_LEN
    RegDesc { addr: 0x5856, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 473: RX_TOP_SF12_CFG1_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x5856, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 474: RX_TOP_SF12_CFG1_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x5856, offs: 2, leng: 2, dflt: 2, flags: 0x04 }, // 475: RX_TOP_SF12_CFG1_ACC_COEFF
    RegDesc { addr: 0x5856, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 476: RX_TOP_SF12_CFG1_ACC_AUTO_RESCALE
    RegDesc { addr: 0x5856, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 477: RX_TOP_SF12_CFG1_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x5857, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 478: RX_TOP_SF12_CFG2_ACC_MIN2
    RegDesc { addr: 0x5857, offs: 0, leng: 7, dflt: 60, flags: 0x04 }, // 479: RX_TOP_SF12_CFG2_ACC_PNR
    RegDesc { addr: 0x5858, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 480: RX_TOP_SF12_CFG3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x5859, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 481: RX_TOP_SF12_CFG4_MSP_PNR
    RegDesc { addr: 0x585A, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 482: RX_TOP_SF12_CFG5_MSP2_PNR
    RegDesc { addr: 0x585B, offs: 3, leng: 3, dflt: 3, flags: 0x04 }, // 483: RX_TOP_SF12_CFG6_MSP_PEAK_NB
    RegDesc { addr: 0x585B, offs: 1, leng: 2, dflt: 1, flags: 0x04 }, // 484: RX_TOP_SF12_CFG6_MSP_CNT_MODE
    RegDesc { addr: 0x585B, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 485: RX_TOP_SF12_CFG6_MSP_POS_SEL
    RegDesc { addr: 0x585C, offs: 2, leng: 3, dflt: 5, flags: 0x04 }, // 486: RX_TOP_SF12_CFG7_MSP2_PEAK_NB
    RegDesc { addr: 0x585C, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 487: RX_TOP_SF12_CFG7_NOISE_COEFF
    RegDesc { addr: 0x585D, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 488: RX_TOP_DUMMY1_DUMMY1
    RegDesc { addr: 0x5860, offs: 4, leng: 3, dflt: 0, flags: 0x04 }, // 489: RX_TOP_DC_NOTCH_CFG1_BW_START
    RegDesc { addr: 0x5860, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 490: RX_TOP_DC_NOTCH_CFG1_AUTO_BW_RED
    RegDesc { addr: 0x5860, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 491: RX_TOP_DC_NOTCH_CFG1_NO_FAST_START
    RegDesc { addr: 0x5860, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 492: RX_TOP_DC_NOTCH_CFG1_BYPASS
    RegDesc { addr: 0x5860, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 493: RX_TOP_DC_NOTCH_CFG1_ENABLE
    RegDesc { addr: 0x5861, offs: 3, leng: 3, dflt: 1, flags: 0x04 }, // 494: RX_TOP_DC_NOTCH_CFG2_BW_LOCKED
    RegDesc { addr: 0x5861, offs: 0, leng: 3, dflt: 5, flags: 0x04 }, // 495: RX_TOP_DC_NOTCH_CFG2_BW
    RegDesc { addr: 0x5862, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 496: RX_TOP_DC_NOTCH_CFG3_BW_RED
    RegDesc { addr: 0x5863, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 497: RX_TOP_DC_NOTCH_CFG4_IIR_DCC_TIME
    RegDesc { addr: 0x5864, offs: 0, leng: 8, dflt: 2, flags: 0x05 }, // 498: RX_TOP_RX_DFE_FIR1_0_FIR1_COEFF_0
    RegDesc { addr: 0x5865, offs: 0, leng: 8, dflt: 3, flags: 0x05 }, // 499: RX_TOP_RX_DFE_FIR1_1_FIR1_COEFF_1
    RegDesc { addr: 0x5866, offs: 0, leng: 8, dflt: 2, flags: 0x05 }, // 500: RX_TOP_RX_DFE_FIR1_2_FIR1_COEFF_2
    RegDesc { addr: 0x5867, offs: 0, leng: 8, dflt: 3, flags: 0x05 }, // 501: RX_TOP_RX_DFE_FIR1_3_FIR1_COEFF_3
    RegDesc { addr: 0x5868, offs: 0, leng: 8, dflt: 5, flags: 0x05 }, // 502: RX_TOP_RX_DFE_FIR1_4_FIR1_COEFF_4
    RegDesc { addr: 0x5869, offs: 0, leng: 8, dflt: 8, flags: 0x05 }, // 503: RX_TOP_RX_DFE_FIR1_5_FIR1_COEFF_5
    RegDesc { addr: 0x586A, offs: 0, leng: 8, dflt: 6, flags: 0x05 }, // 504: RX_TOP_RX_DFE_FIR1_6_FIR1_COEFF_6
    RegDesc { addr: 0x586B, offs: 0, leng: 8, dflt: 4, flags: 0x05 }, // 505: RX_TOP_RX_DFE_FIR1_7_FIR1_COEFF_7
    RegDesc { addr: 0x586C, offs: 0, leng: 8, dflt: 2, flags: 0x05 }, // 506: RX_TOP_RX_DFE_FIR2_0_FIR2_COEFF_0
    RegDesc { addr: 0x586D, offs: 0, leng: 8, dflt: -2, flags: 0x05 }, // 507: RX_TOP_RX_DFE_FIR2_1_FIR2_COEFF_1
    RegDesc { addr: 0x586E, offs: 0, leng: 8, dflt: -4, flags: 0x05 }, // 508: RX_TOP_RX_DFE_FIR2_2_FIR2_COEFF_2
    RegDesc { addr: 0x586F, offs: 0, leng: 8, dflt: -3, flags: 0x05 }, // 509: RX_TOP_RX_DFE_FIR2_3_FIR2_COEFF_3
    RegDesc { addr: 0x5870, offs: 0, leng: 8, dflt: 3, flags: 0x05 }, // 510: RX_TOP_RX_DFE_FIR2_4_FIR2_COEFF_4
    RegDesc { addr: 0x5871, offs: 0, leng: 8, dflt: 11, flags: 0x05 }, // 511: RX_TOP_RX_DFE_FIR2_5_FIR2_COEFF_5
    RegDesc { addr: 0x5872, offs: 0, leng: 8, dflt: 19, flags: 0x05 }, // 512: RX_TOP_RX_DFE_FIR2_6_FIR2_COEFF_6
    RegDesc { addr: 0x5873, offs: 0, leng: 8, dflt: 10, flags: 0x05 }, // 513: RX_TOP_RX_DFE_FIR2_7_FIR2_COEFF_7
    RegDesc { addr: 0x5874, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 514: RX_TOP_RX_DFE_AGC0_RADIO_GAIN_RED_SEL
    RegDesc { addr: 0x5874, offs: 0, leng: 7, dflt: 0, flags: 0x04 }, // 515: RX_TOP_RX_DFE_AGC0_RADIO_GAIN_RED_DB
    RegDesc { addr: 0x5875, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 516: RX_TOP_RX_DFE_AGC1_DC_COMP_EN
    RegDesc { addr: 0x5875, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 517: RX_TOP_RX_DFE_AGC1_FORCE_DEFAULT_FIR
    RegDesc { addr: 0x5875, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 518: RX_TOP_RX_DFE_AGC1_RSSI_EARLY_LATCH
    RegDesc { addr: 0x5875, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 519: RX_TOP_RX_DFE_AGC1_FREEZE_ON_SYNC
    RegDesc { addr: 0x5876, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 520: RX_TOP_RX_DFE_AGC2_DAGC_IN_COMP
    RegDesc { addr: 0x5876, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 521: RX_TOP_RX_DFE_AGC2_DAGC_FIR_HYST
    RegDesc { addr: 0x5876, offs: 3, leng: 2, dflt: 1, flags: 0x04 }, // 522: RX_TOP_RX_DFE_AGC2_RSSI_MAX_SAMPLE
    RegDesc { addr: 0x5876, offs: 0, leng: 3, dflt: 2, flags: 0x04 }, // 523: RX_TOP_RX_DFE_AGC2_RSSI_MIN_SAMPLE
    RegDesc { addr: 0x5877, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 524: RX_TOP_RX_DFE_GAIN0_DAGC_FIR_FAST
    RegDesc { addr: 0x5877, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 525: RX_TOP_RX_DFE_GAIN0_FORCE_GAIN_FIR
    RegDesc { addr: 0x5877, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 526: RX_TOP_RX_DFE_GAIN0_GAIN_FIR1
    RegDesc { addr: 0x5877, offs: 0, leng: 3, dflt: 1, flags: 0x04 }, // 527: RX_TOP_RX_DFE_GAIN0_GAIN_FIR2
    RegDesc { addr: 0x5878, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 528: RX_TOP_DAGC_CFG_TARGET_LVL
    RegDesc { addr: 0x5878, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 529: RX_TOP_DAGC_CFG_GAIN_INCR_STEP
    RegDesc { addr: 0x5878, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 530: RX_TOP_DAGC_CFG_GAIN_DROP_COMP
    RegDesc { addr: 0x5878, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 531: RX_TOP_DAGC_CFG_COMB_FILTER_EN
    RegDesc { addr: 0x5878, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 532: RX_TOP_DAGC_CFG_NO_FREEZE_START
    RegDesc { addr: 0x5878, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 533: RX_TOP_DAGC_CFG_FREEZE_ON_SYNC
    RegDesc { addr: 0x5879, offs: 0, leng: 8, dflt: 60, flags: 0x04 }, // 534: RX_TOP_DAGC_CNT0_SAMPLE
    RegDesc { addr: 0x587A, offs: 0, leng: 8, dflt: 6, flags: 0x04 }, // 535: RX_TOP_DAGC_CNT1_THR_M6
    RegDesc { addr: 0x587B, offs: 0, leng: 8, dflt: 25, flags: 0x04 }, // 536: RX_TOP_DAGC_CNT2_THR_M12
    RegDesc { addr: 0x587C, offs: 0, leng: 8, dflt: 42, flags: 0x04 }, // 537: RX_TOP_DAGC_CNT3_THR_M18
    RegDesc { addr: 0x587D, offs: 4, leng: 4, dflt: 8, flags: 0x04 }, // 538: RX_TOP_DAGC_CNT4_GAIN
    RegDesc { addr: 0x587D, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 539: RX_TOP_DAGC_CNT4_FORCE_GAIN
    RegDesc { addr: 0x587E, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 540: RX_TOP_TXRX_CFG1_PPM_OFFSET_HDR_CTRL
    RegDesc { addr: 0x587E, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 541: RX_TOP_TXRX_CFG1_PPM_OFFSET
    RegDesc { addr: 0x587E, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 542: RX_TOP_TXRX_CFG1_MODEM_EN
    RegDesc { addr: 0x587E, offs: 0, leng: 3, dflt: 2, flags: 0x04 }, // 543: RX_TOP_TXRX_CFG1_CODING_RATE
    RegDesc { addr: 0x587F, offs: 4, leng: 1, dflt: 0, flags: 0x00 }, // 544: RX_TOP_TXRX_CFG2_MODEM_START
    RegDesc { addr: 0x587F, offs: 2, leng: 2, dflt: 1, flags: 0x04 }, // 545: RX_TOP_TXRX_CFG2_CADRXTX
    RegDesc { addr: 0x587F, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 546: RX_TOP_TXRX_CFG2_IMPLICIT_HEADER
    RegDesc { addr: 0x587F, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 547: RX_TOP_TXRX_CFG2_CRC_EN
    RegDesc { addr: 0x5880, offs: 0, leng: 8, dflt: 12, flags: 0x04 }, // 548: RX_TOP_TXRX_CFG3_PAYLOAD_LENGTH
    RegDesc { addr: 0x5881, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 549: RX_TOP_TXRX_CFG4_INT_STEP_ORIDE_EN
    RegDesc { addr: 0x5881, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 550: RX_TOP_TXRX_CFG4_INT_STEP_ORIDE
    RegDesc { addr: 0x5882, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 551: RX_TOP_TXRX_CFG5_HEADER_DIFF_MODE
    RegDesc { addr: 0x5882, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 552: RX_TOP_TXRX_CFG5_ZERO_PAD
    RegDesc { addr: 0x5883, offs: 0, leng: 8, dflt: 8, flags: 0x04 }, // 553: RX_TOP_TXRX_CFG6_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5884, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 554: RX_TOP_TXRX_CFG7_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5885, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 555: RX_TOP_TXRX_CFG8_AUTO_ACK_INT_DELAY
    RegDesc { addr: 0x5885, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 556: RX_TOP_TXRX_CFG8_AUTO_ACK_RX
    RegDesc { addr: 0x5885, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 557: RX_TOP_TXRX_CFG8_AUTO_ACK_TX
    RegDesc { addr: 0x5885, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 558: RX_TOP_TXRX_CFG8_POST_PREAMBLE_GAP_LONG
    RegDesc { addr: 0x5886, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 559: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF12
    RegDesc { addr: 0x5886, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 560: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF11
    RegDesc { addr: 0x5886, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 561: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF10
    RegDesc { addr: 0x5886, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 562: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF9
    RegDesc { addr: 0x5886, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 563: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF8
    RegDesc { addr: 0x5886, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 564: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF7
    RegDesc { addr: 0x5886, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 565: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF6
    RegDesc { addr: 0x5886, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 566: RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF5
    RegDesc { addr: 0x5887, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 567: RX_TOP_RX_CFG0_DFT_PEAK_EN
    RegDesc { addr: 0x5887, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 568: RX_TOP_RX_CFG0_CHIRP_INVERT
    RegDesc { addr: 0x5887, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 569: RX_TOP_RX_CFG0_SWAP_IQ
    RegDesc { addr: 0x5887, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 570: RX_TOP_RX_CFG0_CONTINUOUS
    RegDesc { addr: 0x5888, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 571: RX_TOP_RX_CFG1_DETECT_TIMEOUT
    RegDesc { addr: 0x5889, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 572: RX_TOP_RX_CFG2_CLK_EN_RESYNC_DIN
    RegDesc { addr: 0x5889, offs: 0, leng: 4, dflt: 11, flags: 0x04 }, // 573: RX_TOP_RX_CFG2_LLR_SCALE
    RegDesc { addr: 0x588A, offs: 0, leng: 5, dflt: 2, flags: 0x05 }, // 574: RX_TOP_FRAME_SYNCH0_SF5_PEAK1_POS_SF5
    RegDesc { addr: 0x588B, offs: 0, leng: 5, dflt: 4, flags: 0x05 }, // 575: RX_TOP_FRAME_SYNCH1_SF5_PEAK2_POS_SF5
    RegDesc { addr: 0x588C, offs: 0, leng: 5, dflt: 2, flags: 0x05 }, // 576: RX_TOP_FRAME_SYNCH0_SF6_PEAK1_POS_SF6
    RegDesc { addr: 0x588D, offs: 0, leng: 5, dflt: 4, flags: 0x05 }, // 577: RX_TOP_FRAME_SYNCH1_SF6_PEAK2_POS_SF6
    RegDesc { addr: 0x588E, offs: 0, leng: 5, dflt: 2, flags: 0x05 }, // 578: RX_TOP_FRAME_SYNCH0_SF7TO12_PEAK1_POS_SF7TO12
    RegDesc { addr: 0x588F, offs: 0, leng: 5, dflt: 4, flags: 0x05 }, // 579: RX_TOP_FRAME_SYNCH1_SF7TO12_PEAK2_POS_SF7TO12
    RegDesc { addr: 0x5890, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 580: RX_TOP_FRAME_SYNCH2_FINETIME_ON_LAST
    RegDesc { addr: 0x5890, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 581: RX_TOP_FRAME_SYNCH2_AUTO_SCALE
    RegDesc { addr: 0x5890, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 582: RX_TOP_FRAME_SYNCH2_DROP_ON_SYNCH
    RegDesc { addr: 0x5890, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 583: RX_TOP_FRAME_SYNCH2_GAIN
    RegDesc { addr: 0x5890, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 584: RX_TOP_FRAME_SYNCH2_TIMEOUT_OPT
    RegDesc { addr: 0x5891, offs: 7, leng: 1, dflt: 1, flags: 0x04 }, // 585: RX_TOP_FINE_TIMING_A_0_GAIN_P_HDR_RED
    RegDesc { addr: 0x5891, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 586: RX_TOP_FINE_TIMING_A_0_ROUNDING
    RegDesc { addr: 0x5891, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 587: RX_TOP_FINE_TIMING_A_0_POS_LIMIT
    RegDesc { addr: 0x5891, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 588: RX_TOP_FINE_TIMING_A_0_SUM_SIZE
    RegDesc { addr: 0x5891, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 589: RX_TOP_FINE_TIMING_A_0_MODE
    RegDesc { addr: 0x5892, offs: 6, leng: 2, dflt: 1, flags: 0x04 }, // 590: RX_TOP_FINE_TIMING_A_1_GAIN_P_AUTO
    RegDesc { addr: 0x5892, offs: 3, leng: 3, dflt: 2, flags: 0x04 }, // 591: RX_TOP_FINE_TIMING_A_1_GAIN_P_PAYLOAD
    RegDesc { addr: 0x5892, offs: 0, leng: 3, dflt: 6, flags: 0x04 }, // 592: RX_TOP_FINE_TIMING_A_1_GAIN_P_PREAMB
    RegDesc { addr: 0x5893, offs: 6, leng: 2, dflt: 3, flags: 0x04 }, // 593: RX_TOP_FINE_TIMING_A_2_GAIN_I_AUTO
    RegDesc { addr: 0x5893, offs: 3, leng: 3, dflt: 1, flags: 0x04 }, // 594: RX_TOP_FINE_TIMING_A_2_GAIN_I_PAYLOAD
    RegDesc { addr: 0x5893, offs: 0, leng: 3, dflt: 4, flags: 0x04 }, // 595: RX_TOP_FINE_TIMING_A_2_GAIN_I_PREAMB
    RegDesc { addr: 0x5894, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 596: RX_TOP_FINE_TIMING_A_3_FINESYNCH_SUM
    RegDesc { addr: 0x5894, offs: 4, leng: 3, dflt: 5, flags: 0x04 }, // 597: RX_TOP_FINE_TIMING_A_3_FINESYNCH_GAIN
    RegDesc { addr: 0x5895, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 598: RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF8
    RegDesc { addr: 0x5895, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 599: RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF7
    RegDesc { addr: 0x5895, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 600: RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF6
    RegDesc { addr: 0x5895, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 601: RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF5
    RegDesc { addr: 0x5896, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 602: RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF12
    RegDesc { addr: 0x5896, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 603: RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF11
    RegDesc { addr: 0x5896, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 604: RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF10
    RegDesc { addr: 0x5896, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 605: RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF9
    RegDesc { addr: 0x5897, offs: 4, leng: 3, dflt: 7, flags: 0x04 }, // 606: RX_TOP_FINE_TIMING_A_6_GAIN_P_PREAMB_SF12
    RegDesc { addr: 0x5897, offs: 0, leng: 3, dflt: 4, flags: 0x04 }, // 607: RX_TOP_FINE_TIMING_A_6_GAIN_P_PREAMB_SF5_6
    RegDesc { addr: 0x5898, offs: 4, leng: 3, dflt: 0, flags: 0x04 }, // 608: RX_TOP_FINE_TIMING_7_GAIN_I_AUTO_MAX
    RegDesc { addr: 0x5898, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 609: RX_TOP_FINE_TIMING_7_GAIN_P_AUTO_MAX
    RegDesc { addr: 0x5899, offs: 7, leng: 1, dflt: 1, flags: 0x04 }, // 610: RX_TOP_FINE_TIMING_B_0_GAIN_P_HDR_RED
    RegDesc { addr: 0x5899, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 611: RX_TOP_FINE_TIMING_B_0_ROUNDING
    RegDesc { addr: 0x5899, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 612: RX_TOP_FINE_TIMING_B_0_POS_LIMIT
    RegDesc { addr: 0x5899, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 613: RX_TOP_FINE_TIMING_B_0_SUM_SIZE
    RegDesc { addr: 0x5899, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 614: RX_TOP_FINE_TIMING_B_0_MODE
    RegDesc { addr: 0x589A, offs: 6, leng: 2, dflt: 1, flags: 0x04 }, // 615: RX_TOP_FINE_TIMING_B_1_GAIN_P_AUTO
    RegDesc { addr: 0x589A, offs: 3, leng: 3, dflt: 2, flags: 0x04 }, // 616: RX_TOP_FINE_TIMING_B_1_GAIN_P_PAYLOAD
    RegDesc { addr: 0x589A, offs: 0, leng: 3, dflt: 6, flags: 0x04 }, // 617: RX_TOP_FINE_TIMING_B_1_GAIN_P_PREAMB
    RegDesc { addr: 0x589B, offs: 6, leng: 2, dflt: 3, flags: 0x04 }, // 618: RX_TOP_FINE_TIMING_B_2_GAIN_I_AUTO
    RegDesc { addr: 0x589B, offs: 3, leng: 3, dflt: 1, flags: 0x04 }, // 619: RX_TOP_FINE_TIMING_B_2_GAIN_I_PAYLOAD
    RegDesc { addr: 0x589B, offs: 0, leng: 3, dflt: 4, flags: 0x04 }, // 620: RX_TOP_FINE_TIMING_B_2_GAIN_I_PREAMB
    RegDesc { addr: 0x589C, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 621: RX_TOP_FINE_TIMING_B_3_FINESYNCH_SUM
    RegDesc { addr: 0x589C, offs: 4, leng: 3, dflt: 5, flags: 0x04 }, // 622: RX_TOP_FINE_TIMING_B_3_FINESYNCH_GAIN
    RegDesc { addr: 0x589D, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 623: RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF8
    RegDesc { addr: 0x589D, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 624: RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF7
    RegDesc { addr: 0x589D, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 625: RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF6
    RegDesc { addr: 0x589D, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 626: RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF5
    RegDesc { addr: 0x589E, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 627: RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF12
    RegDesc { addr: 0x589E, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 628: RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF11
    RegDesc { addr: 0x589E, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 629: RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF10
    RegDesc { addr: 0x589E, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 630: RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF9
    RegDesc { addr: 0x589F, offs: 4, leng: 3, dflt: 7, flags: 0x04 }, // 631: RX_TOP_FINE_TIMING_B_6_GAIN_P_PREAMB_SF12
    RegDesc { addr: 0x589F, offs: 0, leng: 3, dflt: 4, flags: 0x04 }, // 632: RX_TOP_FINE_TIMING_B_6_GAIN_P_PREAMB_SF5_6
    RegDesc { addr: 0x58A0, offs: 0, leng: 4, dflt: 9, flags: 0x04 }, // 633: RX_TOP_FREQ_TO_TIME0_FREQ_TO_TIME_DRIFT_MANT
    RegDesc { addr: 0x58A1, offs: 0, leng: 8, dflt: 112, flags: 0x04 }, // 634: RX_TOP_FREQ_TO_TIME1_FREQ_TO_TIME_DRIFT_MANT
    RegDesc { addr: 0x58A2, offs: 0, leng: 3, dflt: 3, flags: 0x04 }, // 635: RX_TOP_FREQ_TO_TIME2_FREQ_TO_TIME_DRIFT_EXP
    RegDesc { addr: 0x58A3, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 636: RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_DELTA
    RegDesc { addr: 0x58A3, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 637: RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FINE_DELTA
    RegDesc { addr: 0x58A3, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 638: RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_ERROR
    RegDesc { addr: 0x58A3, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 639: RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_SYMB
    RegDesc { addr: 0x58A3, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 640: RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_OFFSET
    RegDesc { addr: 0x58A3, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 641: RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_DETECT
    RegDesc { addr: 0x58A4, offs: 0, leng: 8, dflt: 33, flags: 0x04 }, // 642: RX_TOP_FREQ_TO_TIME4_FREQ_TO_TIME_INVERT_RNG
    RegDesc { addr: 0x58A5, offs: 6, leng: 2, dflt: 3, flags: 0x04 }, // 643: RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF8
    RegDesc { addr: 0x58A5, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 644: RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF7
    RegDesc { addr: 0x58A5, offs: 2, leng: 2, dflt: 3, flags: 0x04 }, // 645: RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF6
    RegDesc { addr: 0x58A5, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 646: RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF5
    RegDesc { addr: 0x58A6, offs: 6, leng: 2, dflt: 3, flags: 0x04 }, // 647: RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF12
    RegDesc { addr: 0x58A6, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 648: RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF11
    RegDesc { addr: 0x58A6, offs: 2, leng: 2, dflt: 3, flags: 0x04 }, // 649: RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF10
    RegDesc { addr: 0x58A6, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 650: RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF9
    RegDesc { addr: 0x58A7, offs: 6, leng: 2, dflt: 3, flags: 0x04 }, // 651: RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF8
    RegDesc { addr: 0x58A7, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 652: RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF7
    RegDesc { addr: 0x58A7, offs: 2, leng: 2, dflt: 3, flags: 0x04 }, // 653: RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF6
    RegDesc { addr: 0x58A7, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 654: RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF5
    RegDesc { addr: 0x58A8, offs: 6, leng: 2, dflt: 3, flags: 0x04 }, // 655: RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF12
    RegDesc { addr: 0x58A8, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 656: RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF11
    RegDesc { addr: 0x58A8, offs: 2, leng: 2, dflt: 3, flags: 0x04 }, // 657: RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF10
    RegDesc { addr: 0x58A8, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 658: RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF9
    RegDesc { addr: 0x58A9, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 659: RX_TOP_FREQ_TRACK2_FREQ_TRACK_FINE
    RegDesc { addr: 0x58A9, offs: 4, leng: 3, dflt: 4, flags: 0x04 }, // 660: RX_TOP_FREQ_TRACK2_FREQ_TRACK_HDR_SKIP
    RegDesc { addr: 0x58AA, offs: 4, leng: 3, dflt: 5, flags: 0x04 }, // 661: RX_TOP_FREQ_TRACK3_FREQ_SYNCH_GAIN
    RegDesc { addr: 0x58AA, offs: 0, leng: 4, dflt: 3, flags: 0x04 }, // 662: RX_TOP_FREQ_TRACK3_FREQ_TRACK_AUTO_THR
    RegDesc { addr: 0x58AB, offs: 5, leng: 2, dflt: 1, flags: 0x04 }, // 663: RX_TOP_FREQ_TRACK4_SNR_MIN_WINDOW
    RegDesc { addr: 0x58AB, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 664: RX_TOP_FREQ_TRACK4_GAIN_AUTO_SNR_MIN
    RegDesc { addr: 0x58AB, offs: 0, leng: 4, dflt: 8, flags: 0x04 }, // 665: RX_TOP_FREQ_TRACK4_FREQ_SYNCH_THR
    RegDesc { addr: 0x58AC, offs: 0, leng: 7, dflt: 32, flags: 0x04 }, // 666: RX_TOP_DETECT_MSP0_MSP_PNR
    RegDesc { addr: 0x58AD, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 667: RX_TOP_DETECT_MSP1_MSP2_PNR
    RegDesc { addr: 0x58AE, offs: 4, leng: 3, dflt: 5, flags: 0x04 }, // 668: RX_TOP_DETECT_MSP2_MSP2_PEAK_NB
    RegDesc { addr: 0x58AE, offs: 0, leng: 3, dflt: 3, flags: 0x04 }, // 669: RX_TOP_DETECT_MSP2_MSP_PEAK_NB
    RegDesc { addr: 0x58AF, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 670: RX_TOP_DETECT_MSP3_ACC_MIN2
    RegDesc { addr: 0x58AF, offs: 4, leng: 2, dflt: 2, flags: 0x04 }, // 671: RX_TOP_DETECT_MSP3_ACC_WIN_LEN
    RegDesc { addr: 0x58AF, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 672: RX_TOP_DETECT_MSP3_MSP_POS_SEL
    RegDesc { addr: 0x58AF, offs: 0, leng: 2, dflt: 1, flags: 0x04 }, // 673: RX_TOP_DETECT_MSP3_MSP_CNT_MODE
    RegDesc { addr: 0x58B0, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 674: RX_TOP_DETECT_ACC1_USE_GAIN_SYMB
    RegDesc { addr: 0x58B0, offs: 0, leng: 7, dflt: 64, flags: 0x04 }, // 675: RX_TOP_DETECT_ACC1_ACC_PNR
    RegDesc { addr: 0x58B1, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 676: RX_TOP_DETECT_ACC2_NOISE_COEFF
    RegDesc { addr: 0x58B1, offs: 4, leng: 2, dflt: 2, flags: 0x04 }, // 677: RX_TOP_DETECT_ACC2_ACC_COEFF
    RegDesc { addr: 0x58B1, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 678: RX_TOP_DETECT_ACC2_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x58B1, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 679: RX_TOP_DETECT_ACC2_ACC_AUTO_RESCALE
    RegDesc { addr: 0x58B1, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 680: RX_TOP_DETECT_ACC2_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x58B1, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 681: RX_TOP_DETECT_ACC2_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x58B2, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 682: RX_TOP_DETECT_ACC3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x58B3, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 683: RX_TOP_TIMESTAMP_SEL_SNR_MIN
    RegDesc { addr: 0x58B3, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 684: RX_TOP_TIMESTAMP_ENABLE
    RegDesc { addr: 0x58B3, offs: 0, leng: 3, dflt: 7, flags: 0x04 }, // 685: RX_TOP_TIMESTAMP_NB_SYMB
    RegDesc { addr: 0x58B4, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 686: RX_TOP_MODEM_BUSY_MSB_RX_MODEM_BUSY
    RegDesc { addr: 0x58B5, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 687: RX_TOP_MODEM_BUSY_LSB_RX_MODEM_BUSY
    RegDesc { addr: 0x58B6, offs: 4, leng: 4, dflt: 0, flags: 0x06 }, // 688: RX_TOP_MODEM_STATE_RX_MODEM_STS_SPARE
    RegDesc { addr: 0x58B6, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 689: RX_TOP_MODEM_STATE_RX_MODEM_STATE
    RegDesc { addr: 0x58B7, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 690: RX_TOP_MODEM_SYNC_DELTA_MSB_PEAK_POS_FINE_GAIN_H
    RegDesc { addr: 0x58B7, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 691: RX_TOP_MODEM_SYNC_DELTA_MSB_PEAK_POS_FINE_GAIN_L
    RegDesc { addr: 0x58B7, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 692: RX_TOP_MODEM_SYNC_DELTA_MSB_PEAK_POS_FINE_SIGN
    RegDesc { addr: 0x58B7, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 693: RX_TOP_MODEM_SYNC_DELTA_MSB_MODEM_SYNC_DELTA
    RegDesc { addr: 0x58B8, offs: 0, leng: 8, dflt: 127, flags: 0x04 }, // 694: RX_TOP_MODEM_SYNC_DELTA_LSB_MODEM_SYNC_DELTA
    RegDesc { addr: 0x58B9, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 695: RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF8
    RegDesc { addr: 0x58B9, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 696: RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF7
    RegDesc { addr: 0x58B9, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 697: RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF6
    RegDesc { addr: 0x58B9, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 698: RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF5
    RegDesc { addr: 0x58BA, offs: 6, leng: 2, dflt: 1, flags: 0x04 }, // 699: RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF12
    RegDesc { addr: 0x58BA, offs: 4, leng: 2, dflt: 1, flags: 0x04 }, // 700: RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF11
    RegDesc { addr: 0x58BA, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 701: RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF10
    RegDesc { addr: 0x58BA, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 702: RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF9
    RegDesc { addr: 0x58BB, offs: 0, leng: 8, dflt: 85, flags: 0x04 }, // 703: RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_3_CLK_OVERRIDE
    RegDesc { addr: 0x58BC, offs: 0, leng: 8, dflt: 85, flags: 0x04 }, // 704: RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_2_CLK_OVERRIDE
    RegDesc { addr: 0x58BD, offs: 0, leng: 8, dflt: 85, flags: 0x04 }, // 705: RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_1_CLK_OVERRIDE
    RegDesc { addr: 0x58BE, offs: 0, leng: 8, dflt: 85, flags: 0x04 }, // 706: RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_0_CLK_OVERRIDE
    RegDesc { addr: 0x58BF, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 707: RX_TOP_DUMMY2_DUMMY2
    RegDesc { addr: 0x58C0, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 708: RX_TOP_RX_BUFFER_DEBUG_MODE
    RegDesc { addr: 0x58C0, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 709: RX_TOP_RX_BUFFER_DIRECT_RAM_IF
    RegDesc { addr: 0x58C0, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 710: RX_TOP_RX_BUFFER_LEGACY_TIMESTAMP
    RegDesc { addr: 0x58C0, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 711: RX_TOP_RX_BUFFER_STORE_HEADER_ERR_META
    RegDesc { addr: 0x58C0, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 712: RX_TOP_RX_BUFFER_STORE_SYNC_FAIL_META
    RegDesc { addr: 0x58C1, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 713: RX_TOP_RX_BUFFER_TIMESTAMP_CFG_MAX_TS_METRICS
    RegDesc { addr: 0x58C2, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 714: RX_TOP_RX_BUFFER_IRQ_CTRL_MSB_RX_BUFFER_IRQ_THRESHOLD
    RegDesc { addr: 0x58C3, offs: 0, leng: 8, dflt: 21, flags: 0x04 }, // 715: RX_TOP_RX_BUFFER_IRQ_CTRL_LSB_RX_BUFFER_IRQ_THRESHOLD
    RegDesc { addr: 0x58C4, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 716: RX_TOP_RX_BUFFER_LAST_ADDR_READ_MSB_LAST_ADDR_READ
    RegDesc { addr: 0x58C5, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 717: RX_TOP_RX_BUFFER_LAST_ADDR_READ_LSB_LAST_ADDR_READ
    RegDesc { addr: 0x58C6, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 718: RX_TOP_RX_BUFFER_LAST_ADDR_WRITE_MSB_LAST_ADDR_WRITE
    RegDesc { addr: 0x58C7, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 719: RX_TOP_RX_BUFFER_LAST_ADDR_WRITE_LSB_LAST_ADDR_WRITE
    RegDesc { addr: 0x58C8, offs: 0, leng: 5, dflt: 0, flags: 0x06 }, // 720: RX_TOP_RX_BUFFER_NB_BYTES_MSB_RX_BUFFER_NB_BYTES
    RegDesc { addr: 0x58C9, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 721: RX_TOP_RX_BUFFER_NB_BYTES_LSB_RX_BUFFER_NB_BYTES
    RegDesc { addr: 0x58CA, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 722: RX_TOP_MULTI_SF_SYNC_ERR_PKT_CNT_MULTI_SF_SYNC_ERR_PKTS
    RegDesc { addr: 0x58CB, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 723: RX_TOP_MULTI_SF_PLD_ERR_PKT_CNT_MULTI_SF_PLD_ERR_PKTS
    RegDesc { addr: 0x58CC, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 724: RX_TOP_MULTI_SF_GOOD_PKT_CNT_MULTI_SF_GOOD_PKTS
    RegDesc { addr: 0x58CD, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 725: RX_TOP_SERV_MODEM_SYNC_ERR_PKT_CNT_SERV_MODEM_SYNC_ERR_PKTS
    RegDesc { addr: 0x58CE, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 726: RX_TOP_SERV_MODEM_PLD_ERR_PKT_CNT_SERV_MODEM_PLD_ERR_PKTS
    RegDesc { addr: 0x58CF, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 727: RX_TOP_SERV_MODEM_GOOD_PKT_CNT_SERV_MODEM_GOOD_PKTS
    RegDesc { addr: 0x58D0, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 728: RX_TOP_GFSK_MODEM_SYNC_ERR_PKT_CNT_GFSK_MODEM_SYNC_ERR_PKTS
    RegDesc { addr: 0x58D1, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 729: RX_TOP_GFSK_MODEM_PLD_ERR_PKT_CNT_GFSK_MODEM_PLD_ERR_PKTS
    RegDesc { addr: 0x58D2, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 730: RX_TOP_GFSK_MODEM_GOOD_PKT_CNT_GFSK_MODEM_GOOD_PKTS
    RegDesc { addr: 0x58D3, offs: 0, leng: 2, dflt: 0, flags: 0x06 }, // 731: RX_TOP_BAD_MODEM_ID_WRITE_0_BAD_MODEM_ID_WRITE
    RegDesc { addr: 0x58D4, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 732: RX_TOP_BAD_MODEM_ID_WRITE_1_BAD_MODEM_ID_WRITE
    RegDesc { addr: 0x58D5, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 733: RX_TOP_BAD_MODEM_ID_WRITE_2_BAD_MODEM_ID_WRITE
    RegDesc { addr: 0x58D6, offs: 0, leng: 2, dflt: 0, flags: 0x06 }, // 734: RX_TOP_BAD_MODEM_ID_READ_0_BAD_MODEM_ID_READ
    RegDesc { addr: 0x58D7, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 735: RX_TOP_BAD_MODEM_ID_READ_1_BAD_MODEM_ID_READ
    RegDesc { addr: 0x58D8, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 736: RX_TOP_BAD_MODEM_ID_READ_2_BAD_MODEM_ID_READ
    RegDesc { addr: 0x58D9, offs: 0, leng: 2, dflt: 1, flags: 0x04 }, // 737: RX_TOP_CLOCK_GATE_OVERRIDE_0_CLK_OVERRIDE
    RegDesc { addr: 0x58DA, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 738: RX_TOP_SAMPLE_4_MSPS_LATCHED_125K_SAMPLE_4_MSPS_LATCHED_125K
    RegDesc { addr: 0x58DB, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 739: RX_TOP_DUMMY3_DUMMY3
    RegDesc { addr: 0x6080, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 740: ARB_MCU_CTRL_CLK_EN
    RegDesc { addr: 0x6080, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 741: ARB_MCU_CTRL_RADIO_RST
    RegDesc { addr: 0x6080, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 742: ARB_MCU_CTRL_FORCE_HOST_FE_CTRL
    RegDesc { addr: 0x6080, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 743: ARB_MCU_CTRL_MCU_CLEAR
    RegDesc { addr: 0x6080, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 744: ARB_MCU_CTRL_HOST_PROG
    RegDesc { addr: 0x6080, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 745: ARB_MCU_CTRL_PARITY_ERROR
    RegDesc { addr: 0x6081, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 746: ARB_MCU_MCU_ARB_STATUS_MCU_ARB_STATUS
    RegDesc { addr: 0x6087, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 747: ARB_MCU_UART_CFG_MSBF
    RegDesc { addr: 0x6087, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 748: ARB_MCU_UART_CFG_PAR_EN
    RegDesc { addr: 0x6087, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 749: ARB_MCU_UART_CFG_PAR_MODE
    RegDesc { addr: 0x6087, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 750: ARB_MCU_UART_CFG_START_LEN
    RegDesc { addr: 0x6087, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 751: ARB_MCU_UART_CFG_STOP_LEN
    RegDesc { addr: 0x6087, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 752: ARB_MCU_UART_CFG_WORD_LEN
    RegDesc { addr: 0x6088, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 753: ARB_MCU_UART_CFG2_BIT_RATE
    RegDesc { addr: 0x6089, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 754: ARB_MCU_ARB_DEBUG_CFG_0_ARB_DEBUG_CFG_0
    RegDesc { addr: 0x608A, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 755: ARB_MCU_ARB_DEBUG_CFG_1_ARB_DEBUG_CFG_1
    RegDesc { addr: 0x608B, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 756: ARB_MCU_ARB_DEBUG_CFG_2_ARB_DEBUG_CFG_2
    RegDesc { addr: 0x608C, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 757: ARB_MCU_ARB_DEBUG_CFG_3_ARB_DEBUG_CFG_3
    RegDesc { addr: 0x608D, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 758: ARB_MCU_ARB_DEBUG_STS_0_ARB_DEBUG_STS_0
    RegDesc { addr: 0x608E, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 759: ARB_MCU_ARB_DEBUG_STS_1_ARB_DEBUG_STS_1
    RegDesc { addr: 0x608F, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 760: ARB_MCU_ARB_DEBUG_STS_2_ARB_DEBUG_STS_2
    RegDesc { addr: 0x6090, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 761: ARB_MCU_ARB_DEBUG_STS_3_ARB_DEBUG_STS_3
    RegDesc { addr: 0x6091, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 762: ARB_MCU_ARB_DEBUG_STS_4_ARB_DEBUG_STS_4
    RegDesc { addr: 0x6092, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 763: ARB_MCU_ARB_DEBUG_STS_5_ARB_DEBUG_STS_5
    RegDesc { addr: 0x6093, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 764: ARB_MCU_ARB_DEBUG_STS_6_ARB_DEBUG_STS_6
    RegDesc { addr: 0x6094, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 765: ARB_MCU_ARB_DEBUG_STS_7_ARB_DEBUG_STS_7
    RegDesc { addr: 0x6095, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 766: ARB_MCU_ARB_DEBUG_STS_8_ARB_DEBUG_STS_8
    RegDesc { addr: 0x6096, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 767: ARB_MCU_ARB_DEBUG_STS_9_ARB_DEBUG_STS_9
    RegDesc { addr: 0x6097, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 768: ARB_MCU_ARB_DEBUG_STS_10_ARB_DEBUG_STS_10
    RegDesc { addr: 0x6098, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 769: ARB_MCU_ARB_DEBUG_STS_11_ARB_DEBUG_STS_11
    RegDesc { addr: 0x6099, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 770: ARB_MCU_ARB_DEBUG_STS_12_ARB_DEBUG_STS_12
    RegDesc { addr: 0x609A, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 771: ARB_MCU_ARB_DEBUG_STS_13_ARB_DEBUG_STS_13
    RegDesc { addr: 0x609B, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 772: ARB_MCU_ARB_DEBUG_STS_14_ARB_DEBUG_STS_14
    RegDesc { addr: 0x609C, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 773: ARB_MCU_ARB_DEBUG_STS_15_ARB_DEBUG_STS_15
    RegDesc { addr: 0x609D, offs: 4, leng: 4, dflt: 0, flags: 0x04 }, // 774: ARB_MCU_CHANNEL_SYNC_OFFSET_01_CHANNEL_1_OFFSET
    RegDesc { addr: 0x609D, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 775: ARB_MCU_CHANNEL_SYNC_OFFSET_01_CHANNEL_0_OFFSET
    RegDesc { addr: 0x609E, offs: 4, leng: 4, dflt: 0, flags: 0x04 }, // 776: ARB_MCU_CHANNEL_SYNC_OFFSET_23_CHANNEL_3_OFFSET
    RegDesc { addr: 0x609E, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 777: ARB_MCU_CHANNEL_SYNC_OFFSET_23_CHANNEL_2_OFFSET
    RegDesc { addr: 0x609F, offs: 4, leng: 4, dflt: 0, flags: 0x04 }, // 778: ARB_MCU_CHANNEL_SYNC_OFFSET_45_CHANNEL_5_OFFSET
    RegDesc { addr: 0x609F, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 779: ARB_MCU_CHANNEL_SYNC_OFFSET_45_CHANNEL_4_OFFSET
    RegDesc { addr: 0x60A0, offs: 4, leng: 4, dflt: 0, flags: 0x04 }, // 780: ARB_MCU_CHANNEL_SYNC_OFFSET_67_CHANNEL_7_OFFSET
    RegDesc { addr: 0x60A0, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 781: ARB_MCU_CHANNEL_SYNC_OFFSET_67_CHANNEL_6_OFFSET
    RegDesc { addr: 0x60A1, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 782: ARB_MCU_DUMMY_DUMMY3
    RegDesc { addr: 0x5700, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 783: RADIO_FE_GLBL_CTRL_DECIM_B_CLR
    RegDesc { addr: 0x5700, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 784: RADIO_FE_GLBL_CTRL_DECIM_A_CLR
    RegDesc { addr: 0x5701, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 785: RADIO_FE_CTRL0_RADIO_A_DC_NOTCH_EN
    RegDesc { addr: 0x5701, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 786: RADIO_FE_CTRL0_RADIO_A_FORCE_HOST_FILTER_GAIN
    RegDesc { addr: 0x5701, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 787: RADIO_FE_CTRL0_RADIO_A_HOST_FILTER_GAIN
    RegDesc { addr: 0x5702, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 788: RADIO_FE_RSSI_DB_DEF_RADIO_A_RSSI_DB_DEFAULT_VALUE
    RegDesc { addr: 0x5703, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 789: RADIO_FE_RSSI_DEC_DEF_RADIO_A_RSSI_DEC_DEFAULT_VALUE
    RegDesc { addr: 0x5704, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 790: RADIO_FE_RSSI_DEC_RD_RADIO_A_RSSI_DEC_OUT
    RegDesc { addr: 0x5705, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 791: RADIO_FE_RSSI_BB_RD_RADIO_A_RSSI_BB_OUT
    RegDesc { addr: 0x5706, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 792: RADIO_FE_DEC_FILTER_RD_RADIO_A_DEC_FILTER_GAIN
    RegDesc { addr: 0x5707, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 793: RADIO_FE_RSSI_BB_FILTER_ALPHA_RADIO_A_RSSI_BB_FILTER_ALPHA
    RegDesc { addr: 0x5708, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 794: RADIO_FE_RSSI_DEC_FILTER_ALPHA_RADIO_A_RSSI_DEC_FILTER_ALPHA
    RegDesc { addr: 0x5709, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 795: RADIO_FE_IQ_COMP_AMP_COEFF_RADIO_A_AMP_COEFF
    RegDesc { addr: 0x570A, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 796: RADIO_FE_IQ_COMP_PHI_COEFF_RADIO_A_PHI_COEFF
    RegDesc { addr: 0x570B, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 797: RADIO_FE_RADIO_DIO_TEST_MODE_RADIO_A_DIO_TEST_MODE
    RegDesc { addr: 0x570C, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 798: RADIO_FE_RADIO_DIO_TEST_DIR_RADIO_A_DIO_TEST_DIR
    RegDesc { addr: 0x570D, offs: 0, leng: 6, dflt: 0, flags: 0x06 }, // 799: RADIO_FE_RADIO_DIO_DIR_RADIO_A_DIO_DIR
    RegDesc { addr: 0x570E, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 800: RADIO_FE_CTRL0_RADIO_B_DC_NOTCH_EN
    RegDesc { addr: 0x570E, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 801: RADIO_FE_CTRL0_RADIO_B_FORCE_HOST_FILTER_GAIN
    RegDesc { addr: 0x570E, offs: 0, leng: 4, dflt: 0, flags: 0x04 }, // 802: RADIO_FE_CTRL0_RADIO_B_HOST_FILTER_GAIN
    RegDesc { addr: 0x570F, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 803: RADIO_FE_RSSI_DB_DEF_RADIO_B_RSSI_DB_DEFAULT_VALUE
    RegDesc { addr: 0x5710, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 804: RADIO_FE_RSSI_DEC_DEF_RADIO_B_RSSI_DEC_DEFAULT_VALUE
    RegDesc { addr: 0x5711, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 805: RADIO_FE_RSSI_DEC_RD_RADIO_B_RSSI_DEC_OUT
    RegDesc { addr: 0x5712, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 806: RADIO_FE_RSSI_BB_RD_RADIO_B_RSSI_BB_OUT
    RegDesc { addr: 0x5713, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 807: RADIO_FE_DEC_FILTER_RD_RADIO_B_DEC_FILTER_GAIN
    RegDesc { addr: 0x5714, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 808: RADIO_FE_RSSI_BB_FILTER_ALPHA_RADIO_B_RSSI_BB_FILTER_ALPHA
    RegDesc { addr: 0x5715, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 809: RADIO_FE_RSSI_DEC_FILTER_ALPHA_RADIO_B_RSSI_DEC_FILTER_ALPHA
    RegDesc { addr: 0x5716, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 810: RADIO_FE_IQ_COMP_AMP_COEFF_RADIO_B_AMP_COEFF
    RegDesc { addr: 0x5717, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 811: RADIO_FE_IQ_COMP_PHI_COEFF_RADIO_B_PHI_COEFF
    RegDesc { addr: 0x5718, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 812: RADIO_FE_RADIO_DIO_TEST_MODE_RADIO_B_DIO_TEST_MODE
    RegDesc { addr: 0x5719, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 813: RADIO_FE_RADIO_DIO_TEST_DIR_RADIO_B_DIO_TEST_DIR
    RegDesc { addr: 0x571A, offs: 0, leng: 6, dflt: 0, flags: 0x06 }, // 814: RADIO_FE_RADIO_DIO_DIR_RADIO_B_DIO_DIR
    RegDesc { addr: 0x571B, offs: 7, leng: 1, dflt: 0, flags: 0x06 }, // 815: RADIO_FE_SIG_ANA_CFG_VALID
    RegDesc { addr: 0x571B, offs: 6, leng: 1, dflt: 0, flags: 0x06 }, // 816: RADIO_FE_SIG_ANA_CFG_BUSY
    RegDesc { addr: 0x571B, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 817: RADIO_FE_SIG_ANA_CFG_DURATION
    RegDesc { addr: 0x571B, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 818: RADIO_FE_SIG_ANA_CFG_FORCE_HAL_CTRL
    RegDesc { addr: 0x571B, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 819: RADIO_FE_SIG_ANA_CFG_START
    RegDesc { addr: 0x571B, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 820: RADIO_FE_SIG_ANA_CFG_RADIO_SEL
    RegDesc { addr: 0x571B, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 821: RADIO_FE_SIG_ANA_CFG_EN
    RegDesc { addr: 0x571C, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 822: RADIO_FE_SIG_ANA_FREQ_FREQ
    RegDesc { addr: 0x571D, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 823: RADIO_FE_SIG_ANA_ABS_MSB_CORR_ABS_OUT
    RegDesc { addr: 0x571E, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 824: RADIO_FE_SIG_ANA_ABS_LSB_CORR_ABS_OUT
    RegDesc { addr: 0x571F, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 825: RADIO_FE_DUMMY_DUMMY
    RegDesc { addr: 0x6180, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 826: OTP_BYTE_ADDR_ADDR
    RegDesc { addr: 0x6181, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 827: OTP_RD_DATA_RD_DATA
    RegDesc { addr: 0x6182, offs: 4, leng: 4, dflt: 0, flags: 0x06 }, // 828: OTP_STATUS_CHECKSUM_STATUS
    RegDesc { addr: 0x6182, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 829: OTP_STATUS_FSM_READY
    RegDesc { addr: 0x6183, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 830: OTP_CFG_ACCESS_MODE
    RegDesc { addr: 0x6184, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 831: OTP_BIT_POS_POS
    RegDesc { addr: 0x6185, offs: 4, leng: 4, dflt: 0, flags: 0x04 }, // 832: OTP_PIN_CTRL_0_TM
    RegDesc { addr: 0x6185, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 833: OTP_PIN_CTRL_0_STROBE
    RegDesc { addr: 0x6185, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 834: OTP_PIN_CTRL_0_PGENB
    RegDesc { addr: 0x6185, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 835: OTP_PIN_CTRL_0_LOAD
    RegDesc { addr: 0x6185, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 836: OTP_PIN_CTRL_0_CSB
    RegDesc { addr: 0x6186, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 837: OTP_PIN_CTRL_1_FSCK
    RegDesc { addr: 0x6186, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 838: OTP_PIN_CTRL_1_FSI
    RegDesc { addr: 0x6186, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 839: OTP_PIN_CTRL_1_FRST
    RegDesc { addr: 0x6187, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 840: OTP_PIN_STATUS_FSO
    RegDesc { addr: 0x6188, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 841: OTP_MODEM_EN_0_MODEM_EN
    RegDesc { addr: 0x6189, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 842: OTP_MODEM_EN_1_MODEM_EN
    RegDesc { addr: 0x618A, offs: 0, leng: 8, dflt: 255, flags: 0x04 }, // 843: OTP_MODEM_SF_EN_SF_EN
    RegDesc { addr: 0x618B, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 844: OTP_TIMESTAMP_EN_TIMESTAMP_EN
    RegDesc { addr: 0x618C, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 845: OTP_DUMMY_DUMMY
    RegDesc { addr: 0x5B00, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 846: RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_FREQ_MSB_IF_FREQ_0
    RegDesc { addr: 0x5B01, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 847: RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_FREQ_LSB_IF_FREQ_0
    RegDesc { addr: 0x5B02, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 848: RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_RADIO_SEL_RADIO_SELECT
    RegDesc { addr: 0x5B03, offs: 4, leng: 3, dflt: 0, flags: 0x04 }, // 849: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_BW_START
    RegDesc { addr: 0x5B03, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 850: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_AUTO_BW_RED
    RegDesc { addr: 0x5B03, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 851: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_NO_FAST_START
    RegDesc { addr: 0x5B03, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 852: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_BYPASS
    RegDesc { addr: 0x5B03, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 853: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_ENABLE
    RegDesc { addr: 0x5B04, offs: 3, leng: 3, dflt: 1, flags: 0x04 }, // 854: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG2_BW_LOCKED
    RegDesc { addr: 0x5B04, offs: 0, leng: 3, dflt: 5, flags: 0x04 }, // 855: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG2_BW
    RegDesc { addr: 0x5B05, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 856: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG3_BW_RED
    RegDesc { addr: 0x5B06, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 857: RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG4_IIR_DCC_TIME
    RegDesc { addr: 0x5B07, offs: 0, leng: 8, dflt: 2, flags: 0x05 }, // 858: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_0_FIR1_COEFF_0
    RegDesc { addr: 0x5B08, offs: 0, leng: 8, dflt: 3, flags: 0x05 }, // 859: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_1_FIR1_COEFF_1
    RegDesc { addr: 0x5B09, offs: 0, leng: 8, dflt: 2, flags: 0x05 }, // 860: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_2_FIR1_COEFF_2
    RegDesc { addr: 0x5B0A, offs: 0, leng: 8, dflt: 3, flags: 0x05 }, // 861: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_3_FIR1_COEFF_3
    RegDesc { addr: 0x5B0B, offs: 0, leng: 8, dflt: 5, flags: 0x05 }, // 862: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_4_FIR1_COEFF_4
    RegDesc { addr: 0x5B0C, offs: 0, leng: 8, dflt: 8, flags: 0x05 }, // 863: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_5_FIR1_COEFF_5
    RegDesc { addr: 0x5B0D, offs: 0, leng: 8, dflt: 6, flags: 0x05 }, // 864: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_6_FIR1_COEFF_6
    RegDesc { addr: 0x5B0E, offs: 0, leng: 8, dflt: 4, flags: 0x05 }, // 865: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_7_FIR1_COEFF_7
    RegDesc { addr: 0x5B0F, offs: 0, leng: 8, dflt: 2, flags: 0x05 }, // 866: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_0_FIR2_COEFF_0
    RegDesc { addr: 0x5B10, offs: 0, leng: 8, dflt: -2, flags: 0x05 }, // 867: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_1_FIR2_COEFF_1
    RegDesc { addr: 0x5B11, offs: 0, leng: 8, dflt: -4, flags: 0x05 }, // 868: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_2_FIR2_COEFF_2
    RegDesc { addr: 0x5B12, offs: 0, leng: 8, dflt: -3, flags: 0x05 }, // 869: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_3_FIR2_COEFF_3
    RegDesc { addr: 0x5B13, offs: 0, leng: 8, dflt: 3, flags: 0x05 }, // 870: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_4_FIR2_COEFF_4
    RegDesc { addr: 0x5B14, offs: 0, leng: 8, dflt: 11, flags: 0x05 }, // 871: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_5_FIR2_COEFF_5
    RegDesc { addr: 0x5B15, offs: 0, leng: 8, dflt: 19, flags: 0x05 }, // 872: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_6_FIR2_COEFF_6
    RegDesc { addr: 0x5B16, offs: 0, leng: 8, dflt: 10, flags: 0x05 }, // 873: RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_7_FIR2_COEFF_7
    RegDesc { addr: 0x5B17, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 874: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC0_RADIO_GAIN_RED_SEL
    RegDesc { addr: 0x5B17, offs: 0, leng: 7, dflt: 0, flags: 0x04 }, // 875: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC0_RADIO_GAIN_RED_DB
    RegDesc { addr: 0x5B18, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 876: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_DC_COMP_EN
    RegDesc { addr: 0x5B18, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 877: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_FORCE_DEFAULT_FIR
    RegDesc { addr: 0x5B18, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 878: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_RSSI_EARLY_LATCH
    RegDesc { addr: 0x5B18, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 879: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_FREEZE_ON_SYNC
    RegDesc { addr: 0x5B19, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 880: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_DAGC_IN_COMP
    RegDesc { addr: 0x5B19, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 881: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_DAGC_FIR_HYST
    RegDesc { addr: 0x5B19, offs: 3, leng: 2, dflt: 1, flags: 0x04 }, // 882: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_RSSI_MAX_SAMPLE
    RegDesc { addr: 0x5B19, offs: 0, leng: 3, dflt: 2, flags: 0x04 }, // 883: RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_RSSI_MIN_SAMPLE
    RegDesc { addr: 0x5B1A, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 884: RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_DAGC_FIR_FAST
    RegDesc { addr: 0x5B1A, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 885: RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_FORCE_GAIN_FIR
    RegDesc { addr: 0x5B1A, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 886: RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_GAIN_FIR1
    RegDesc { addr: 0x5B1A, offs: 0, leng: 3, dflt: 1, flags: 0x04 }, // 887: RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_GAIN_FIR2
    RegDesc { addr: 0x5B1B, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 888: RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_TARGET_LVL
    RegDesc { addr: 0x5B1B, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 889: RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_GAIN_INCR_STEP
    RegDesc { addr: 0x5B1B, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 890: RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_GAIN_DROP_COMP
    RegDesc { addr: 0x5B1B, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 891: RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_COMB_FILTER_EN
    RegDesc { addr: 0x5B1B, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 892: RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_NO_FREEZE_START
    RegDesc { addr: 0x5B1B, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 893: RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_FREEZE_ON_SYNC
    RegDesc { addr: 0x5B1C, offs: 0, leng: 8, dflt: 60, flags: 0x04 }, // 894: RX_TOP_LORA_SERVICE_FSK_DAGC_CNT0_SAMPLE
    RegDesc { addr: 0x5B1D, offs: 0, leng: 8, dflt: 6, flags: 0x04 }, // 895: RX_TOP_LORA_SERVICE_FSK_DAGC_CNT1_THR_M6
    RegDesc { addr: 0x5B1E, offs: 0, leng: 8, dflt: 25, flags: 0x04 }, // 896: RX_TOP_LORA_SERVICE_FSK_DAGC_CNT2_THR_M12
    RegDesc { addr: 0x5B1F, offs: 0, leng: 8, dflt: 42, flags: 0x04 }, // 897: RX_TOP_LORA_SERVICE_FSK_DAGC_CNT3_THR_M18
    RegDesc { addr: 0x5B20, offs: 4, leng: 4, dflt: 8, flags: 0x04 }, // 898: RX_TOP_LORA_SERVICE_FSK_DAGC_CNT4_GAIN
    RegDesc { addr: 0x5B20, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 899: RX_TOP_LORA_SERVICE_FSK_DAGC_CNT4_FORCE_GAIN
    RegDesc { addr: 0x5B21, offs: 4, leng: 4, dflt: 4, flags: 0x04 }, // 900: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG0_MODEM_BW
    RegDesc { addr: 0x5B21, offs: 0, leng: 4, dflt: 7, flags: 0x04 }, // 901: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG0_MODEM_SF
    RegDesc { addr: 0x5B22, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 902: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_PPM_OFFSET_HDR_CTRL
    RegDesc { addr: 0x5B22, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 903: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_PPM_OFFSET
    RegDesc { addr: 0x5B22, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 904: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_MODEM_EN
    RegDesc { addr: 0x5B22, offs: 0, leng: 3, dflt: 2, flags: 0x04 }, // 905: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_CODING_RATE
    RegDesc { addr: 0x5B23, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 906: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_FINE_SYNCH_EN
    RegDesc { addr: 0x5B23, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 907: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_MODEM_START
    RegDesc { addr: 0x5B23, offs: 2, leng: 2, dflt: 1, flags: 0x04 }, // 908: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_CADRXTX
    RegDesc { addr: 0x5B23, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 909: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_IMPLICIT_HEADER
    RegDesc { addr: 0x5B23, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 910: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_CRC_EN
    RegDesc { addr: 0x5B24, offs: 0, leng: 8, dflt: 12, flags: 0x04 }, // 911: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG3_PAYLOAD_LENGTH
    RegDesc { addr: 0x5B25, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 912: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG4_INT_STEP_ORIDE_EN
    RegDesc { addr: 0x5B25, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 913: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG4_INT_STEP_ORIDE
    RegDesc { addr: 0x5B26, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 914: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG5_HEADER_DIFF_MODE
    RegDesc { addr: 0x5B26, offs: 0, leng: 6, dflt: 0, flags: 0x04 }, // 915: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG5_ZERO_PAD
    RegDesc { addr: 0x5B27, offs: 0, leng: 8, dflt: 8, flags: 0x04 }, // 916: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG6_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5B28, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 917: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG7_PREAMBLE_SYMB_NB
    RegDesc { addr: 0x5B29, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 918: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_AUTO_ACK_INT_DELAY
    RegDesc { addr: 0x5B29, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 919: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_AUTO_ACK_RX
    RegDesc { addr: 0x5B29, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 920: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_AUTO_ACK_TX
    RegDesc { addr: 0x5B29, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 921: RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_POST_PREAMBLE_GAP_LONG
    RegDesc { addr: 0x5B2A, offs: 4, leng: 2, dflt: 3, flags: 0x04 }, // 922: RX_TOP_LORA_SERVICE_FSK_RX_CFG0_DFT_PEAK_EN
    RegDesc { addr: 0x5B2A, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 923: RX_TOP_LORA_SERVICE_FSK_RX_CFG0_CHIRP_INVERT
    RegDesc { addr: 0x5B2A, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 924: RX_TOP_LORA_SERVICE_FSK_RX_CFG0_SWAP_IQ
    RegDesc { addr: 0x5B2A, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 925: RX_TOP_LORA_SERVICE_FSK_RX_CFG0_CONTINUOUS
    RegDesc { addr: 0x5B2B, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 926: RX_TOP_LORA_SERVICE_FSK_RX_CFG1_DETECT_TIMEOUT
    RegDesc { addr: 0x5B2C, offs: 5, leng: 2, dflt: 2, flags: 0x04 }, // 927: RX_TOP_LORA_SERVICE_FSK_RX_CFG2_AUTO_ACK_RANGE
    RegDesc { addr: 0x5B2C, offs: 0, leng: 5, dflt: 22, flags: 0x04 }, // 928: RX_TOP_LORA_SERVICE_FSK_RX_CFG2_AUTO_ACK_DELAY
    RegDesc { addr: 0x5B2D, offs: 5, leng: 1, dflt: 1, flags: 0x04 }, // 929: RX_TOP_LORA_SERVICE_FSK_RX_CFG3_RESTART_ON_HDR_ERR
    RegDesc { addr: 0x5B2D, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 930: RX_TOP_LORA_SERVICE_FSK_RX_CFG3_CLK_EN_RESYNC_DIN
    RegDesc { addr: 0x5B2D, offs: 0, leng: 4, dflt: 11, flags: 0x04 }, // 931: RX_TOP_LORA_SERVICE_FSK_RX_CFG3_LLR_SCALE
    RegDesc { addr: 0x5B2E, offs: 0, leng: 5, dflt: 2, flags: 0x05 }, // 932: RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH0_PEAK1_POS
    RegDesc { addr: 0x5B2F, offs: 0, leng: 5, dflt: 4, flags: 0x05 }, // 933: RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH1_PEAK2_POS
    RegDesc { addr: 0x5B30, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 934: RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_FINETIME_ON_LAST
    RegDesc { addr: 0x5B30, offs: 4, leng: 1, dflt: 1, flags: 0x04 }, // 935: RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_AUTO_SCALE
    RegDesc { addr: 0x5B30, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 936: RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_DROP_ON_SYNCH
    RegDesc { addr: 0x5B30, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 937: RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_GAIN
    RegDesc { addr: 0x5B30, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 938: RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_TIMEOUT_OPT
    RegDesc { addr: 0x5B31, offs: 7, leng: 1, dflt: 1, flags: 0x04 }, // 939: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_GAIN_P_HDR_RED
    RegDesc { addr: 0x5B31, offs: 6, leng: 1, dflt: 0, flags: 0x04 }, // 940: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_ROUNDING
    RegDesc { addr: 0x5B31, offs: 4, leng: 2, dflt: 0, flags: 0x04 }, // 941: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_POS_LIMIT
    RegDesc { addr: 0x5B31, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 942: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_SUM_SIZE
    RegDesc { addr: 0x5B31, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 943: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_MODE
    RegDesc { addr: 0x5B32, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 944: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_AUTO
    RegDesc { addr: 0x5B32, offs: 3, leng: 3, dflt: 2, flags: 0x04 }, // 945: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_PAYLOAD
    RegDesc { addr: 0x5B32, offs: 0, leng: 3, dflt: 4, flags: 0x04 }, // 946: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_PREAMB
    RegDesc { addr: 0x5B33, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 947: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_EN
    RegDesc { addr: 0x5B33, offs: 3, leng: 3, dflt: 1, flags: 0x04 }, // 948: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_PAYLOAD
    RegDesc { addr: 0x5B33, offs: 0, leng: 3, dflt: 4, flags: 0x04 }, // 949: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_PREAMB
    RegDesc { addr: 0x5B34, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 950: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING3_FINESYNCH_SUM
    RegDesc { addr: 0x5B34, offs: 4, leng: 3, dflt: 5, flags: 0x04 }, // 951: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING3_FINESYNCH_GAIN
    RegDesc { addr: 0x5B34, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 952: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING3_GAIN_I_AUTO
    RegDesc { addr: 0x5B35, offs: 4, leng: 3, dflt: 0, flags: 0x04 }, // 953: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING4_GAIN_I_AUTO_MAX
    RegDesc { addr: 0x5B35, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 954: RX_TOP_LORA_SERVICE_FSK_FINE_TIMING4_GAIN_P_AUTO_MAX
    RegDesc { addr: 0x5B36, offs: 0, leng: 4, dflt: 9, flags: 0x04 }, // 955: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME0_FREQ_TO_TIME_DRIFT_MANT
    RegDesc { addr: 0x5B37, offs: 0, leng: 8, dflt: 112, flags: 0x04 }, // 956: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME1_FREQ_TO_TIME_DRIFT_MANT
    RegDesc { addr: 0x5B38, offs: 0, leng: 3, dflt: 3, flags: 0x04 }, // 957: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME2_FREQ_TO_TIME_DRIFT_EXP
    RegDesc { addr: 0x5B39, offs: 5, leng: 1, dflt: 0, flags: 0x04 }, // 958: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_DELTA
    RegDesc { addr: 0x5B39, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 959: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FINE_DELTA
    RegDesc { addr: 0x5B39, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 960: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_ERROR
    RegDesc { addr: 0x5B39, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 961: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_SYMB
    RegDesc { addr: 0x5B39, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 962: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_OFFSET
    RegDesc { addr: 0x5B39, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 963: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_DETECT
    RegDesc { addr: 0x5B3A, offs: 0, leng: 8, dflt: 33, flags: 0x04 }, // 964: RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME4_FREQ_TO_TIME_INVERT_RNG
    RegDesc { addr: 0x5B3B, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 965: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK0_FREQ_TRACK_FINE
    RegDesc { addr: 0x5B3B, offs: 4, leng: 3, dflt: 4, flags: 0x04 }, // 966: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK0_FREQ_TRACK_HDR_SKIP
    RegDesc { addr: 0x5B3B, offs: 0, leng: 2, dflt: 3, flags: 0x04 }, // 967: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK0_FREQ_TRACK_EN
    RegDesc { addr: 0x5B3C, offs: 4, leng: 3, dflt: 5, flags: 0x04 }, // 968: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK1_FREQ_SYNCH_GAIN
    RegDesc { addr: 0x5B3C, offs: 0, leng: 4, dflt: 3, flags: 0x04 }, // 969: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK1_FREQ_TRACK_AUTO_THR
    RegDesc { addr: 0x5B3D, offs: 5, leng: 2, dflt: 1, flags: 0x04 }, // 970: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK2_SNR_MIN_WINDOW
    RegDesc { addr: 0x5B3D, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 971: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK2_GAIN_AUTO_SNR_MIN
    RegDesc { addr: 0x5B3D, offs: 0, leng: 4, dflt: 8, flags: 0x04 }, // 972: RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK2_FREQ_SYNCH_THR
    RegDesc { addr: 0x5B3E, offs: 0, leng: 7, dflt: 24, flags: 0x04 }, // 973: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP0_MSP_PNR
    RegDesc { addr: 0x5B3F, offs: 0, leng: 7, dflt: 48, flags: 0x04 }, // 974: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP1_MSP2_PNR
    RegDesc { addr: 0x5B40, offs: 4, leng: 3, dflt: 7, flags: 0x04 }, // 975: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP2_MSP2_PEAK_NB
    RegDesc { addr: 0x5B40, offs: 0, leng: 3, dflt: 7, flags: 0x04 }, // 976: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP2_MSP_PEAK_NB
    RegDesc { addr: 0x5B41, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 977: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_ACC_MIN2
    RegDesc { addr: 0x5B41, offs: 4, leng: 2, dflt: 2, flags: 0x04 }, // 978: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_ACC_WIN_LEN
    RegDesc { addr: 0x5B41, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 979: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_MSP_POS_SEL
    RegDesc { addr: 0x5B41, offs: 0, leng: 2, dflt: 1, flags: 0x04 }, // 980: RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_MSP_CNT_MODE
    RegDesc { addr: 0x5B42, offs: 7, leng: 1, dflt: 0, flags: 0x04 }, // 981: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC1_USE_GAIN_SYMB
    RegDesc { addr: 0x5B42, offs: 0, leng: 7, dflt: 55, flags: 0x04 }, // 982: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC1_ACC_PNR
    RegDesc { addr: 0x5B43, offs: 6, leng: 2, dflt: 2, flags: 0x04 }, // 983: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_NOISE_COEFF
    RegDesc { addr: 0x5B43, offs: 4, leng: 2, dflt: 2, flags: 0x04 }, // 984: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_COEFF
    RegDesc { addr: 0x5B43, offs: 3, leng: 1, dflt: 1, flags: 0x04 }, // 985: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_2_SAME_PEAKS
    RegDesc { addr: 0x5B43, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 986: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_AUTO_RESCALE
    RegDesc { addr: 0x5B43, offs: 1, leng: 1, dflt: 1, flags: 0x04 }, // 987: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_PEAK_POS_SEL
    RegDesc { addr: 0x5B43, offs: 0, leng: 1, dflt: 1, flags: 0x04 }, // 988: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_PEAK_SUM_EN
    RegDesc { addr: 0x5B44, offs: 0, leng: 8, dflt: 11, flags: 0x04 }, // 989: RX_TOP_LORA_SERVICE_FSK_DETECT_ACC3_MIN_SINGLE_PEAK
    RegDesc { addr: 0x5B45, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 990: RX_TOP_LORA_SERVICE_FSK_TIMESTAMP_SEL_SNR_MIN
    RegDesc { addr: 0x5B45, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 991: RX_TOP_LORA_SERVICE_FSK_TIMESTAMP_ENABLE
    RegDesc { addr: 0x5B45, offs: 0, leng: 3, dflt: 7, flags: 0x04 }, // 992: RX_TOP_LORA_SERVICE_FSK_TIMESTAMP_NB_SYMB
    RegDesc { addr: 0x5B46, offs: 6, leng: 2, dflt: 1, flags: 0x04 }, // 993: RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_FSK_TRANSPOSE_CLK_OVERRIDE
    RegDesc { addr: 0x5B46, offs: 4, leng: 2, dflt: 1, flags: 0x04 }, // 994: RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_FSK_MODEM_CLK_OVERRIDE
    RegDesc { addr: 0x5B46, offs: 2, leng: 2, dflt: 1, flags: 0x04 }, // 995: RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_TRANSPOSE_CLK_OVERRIDE
    RegDesc { addr: 0x5B46, offs: 0, leng: 2, dflt: 1, flags: 0x04 }, // 996: RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_MODEM_CLK_OVERRIDE
    RegDesc { addr: 0x5B47, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 997: RX_TOP_LORA_SERVICE_FSK_DUMMY0_DUMMY0
    RegDesc { addr: 0x5B50, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 998: RX_TOP_LORA_SERVICE_FSK_FSK_FREQ_MSB_IF_FREQ_0
    RegDesc { addr: 0x5B51, offs: 0, leng: 8, dflt: 128, flags: 0x04 }, // 999: RX_TOP_LORA_SERVICE_FSK_FSK_FREQ_LSB_IF_FREQ_0
    RegDesc { addr: 0x5B52, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 1000: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_CRC_IBM
    RegDesc { addr: 0x5B52, offs: 2, leng: 2, dflt: 0, flags: 0x04 }, // 1001: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_DCFREE_ENC
    RegDesc { addr: 0x5B52, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 1002: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_CRC_EN
    RegDesc { addr: 0x5B52, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 1003: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_PKT_MODE
    RegDesc { addr: 0x5B53, offs: 6, leng: 2, dflt: 0, flags: 0x04 }, // 1004: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_ADRS_COMP
    RegDesc { addr: 0x5B53, offs: 3, leng: 3, dflt: 0, flags: 0x04 }, // 1005: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_PSIZE
    RegDesc { addr: 0x5B53, offs: 0, leng: 3, dflt: 0, flags: 0x04 }, // 1006: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_CH_BW_EXPO
    RegDesc { addr: 0x5B54, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 1007: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_MODEM_INVERT_IQ
    RegDesc { addr: 0x5B54, offs: 2, leng: 1, dflt: 1, flags: 0x04 }, // 1008: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_AUTO_AFC
    RegDesc { addr: 0x5B54, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 1009: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_RADIO_SELECT
    RegDesc { addr: 0x5B54, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 1010: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_RX_INVERT
    RegDesc { addr: 0x5B55, offs: 5, leng: 3, dflt: 4, flags: 0x04 }, // 1011: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_4_RSSI_LENGTH
    RegDesc { addr: 0x5B55, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 1012: RX_TOP_LORA_SERVICE_FSK_FSK_CFG_4_ERROR_OSR_TOL
    RegDesc { addr: 0x5B56, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1013: RX_TOP_LORA_SERVICE_FSK_FSK_NODE_ADRS_NODE_ADRS
    RegDesc { addr: 0x5B57, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1014: RX_TOP_LORA_SERVICE_FSK_FSK_BROADCAST_BROADCAST
    RegDesc { addr: 0x5B58, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1015: RX_TOP_LORA_SERVICE_FSK_FSK_PKT_LENGTH_PKT_LENGTH
    RegDesc { addr: 0x5B59, offs: 0, leng: 2, dflt: 0, flags: 0x04 }, // 1016: RX_TOP_LORA_SERVICE_FSK_FSK_TIMEOUT_MSB_TIMEOUT
    RegDesc { addr: 0x5B5A, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1017: RX_TOP_LORA_SERVICE_FSK_FSK_TIMEOUT_LSB_TIMEOUT
    RegDesc { addr: 0x5B5B, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1018: RX_TOP_LORA_SERVICE_FSK_BIT_RATE_MSB_BIT_RATE
    RegDesc { addr: 0x5B5C, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1019: RX_TOP_LORA_SERVICE_FSK_BIT_RATE_LSB_BIT_RATE
    RegDesc { addr: 0x5B5D, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1020: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN
    RegDesc { addr: 0x5B5E, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1021: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN
    RegDesc { addr: 0x5B5F, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1022: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN
    RegDesc { addr: 0x5B60, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1023: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN
    RegDesc { addr: 0x5B61, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1024: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN
    RegDesc { addr: 0x5B62, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1025: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN
    RegDesc { addr: 0x5B63, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1026: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN
    RegDesc { addr: 0x5B64, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1027: RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN
    RegDesc { addr: 0x5B65, offs: 0, leng: 4, dflt: 4, flags: 0x04 }, // 1028: RX_TOP_LORA_SERVICE_FSK_FSK_RSSI_FILTER_ALPHA_FSK_RSSI_FILTER_ALPHA
    RegDesc { addr: 0x5B66, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 1029: RX_TOP_LORA_SERVICE_FSK_DUMMY1_DUMMY1
    RegDesc { addr: 0x6000, offs: 4, leng: 1, dflt: 0, flags: 0x04 }, // 1030: CAPTURE_RAM_CAPTURE_CFG_ENABLE
    RegDesc { addr: 0x6000, offs: 3, leng: 1, dflt: 0, flags: 0x04 }, // 1031: CAPTURE_RAM_CAPTURE_CFG_CAPTUREWRAP
    RegDesc { addr: 0x6000, offs: 2, leng: 1, dflt: 0, flags: 0x04 }, // 1032: CAPTURE_RAM_CAPTURE_CFG_CAPTUREFORCETRIGGER
    RegDesc { addr: 0x6000, offs: 1, leng: 1, dflt: 0, flags: 0x04 }, // 1033: CAPTURE_RAM_CAPTURE_CFG_CAPTURESTART
    RegDesc { addr: 0x6000, offs: 0, leng: 1, dflt: 0, flags: 0x04 }, // 1034: CAPTURE_RAM_CAPTURE_CFG_RAMCONFIG
    RegDesc { addr: 0x6001, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 1035: CAPTURE_RAM_CAPTURE_SOURCE_A_SOURCEMUX
    RegDesc { addr: 0x6002, offs: 0, leng: 5, dflt: 0, flags: 0x04 }, // 1036: CAPTURE_RAM_CAPTURE_SOURCE_B_SOURCEMUX
    RegDesc { addr: 0x6003, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1037: CAPTURE_RAM_CAPTURE_PERIOD_0_CAPTUREPERIOD
    RegDesc { addr: 0x6004, offs: 0, leng: 8, dflt: 0, flags: 0x04 }, // 1038: CAPTURE_RAM_CAPTURE_PERIOD_1_CAPTUREPERIOD
    RegDesc { addr: 0x6005, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 1039: CAPTURE_RAM_STATUS_CAPCOMPLETE
    RegDesc { addr: 0x6006, offs: 0, leng: 8, dflt: 0, flags: 0x06 }, // 1040: CAPTURE_RAM_LAST_RAM_ADDR_0_LASTRAMADDR
    RegDesc { addr: 0x6007, offs: 0, leng: 4, dflt: 0, flags: 0x06 }, // 1041: CAPTURE_RAM_LAST_RAM_ADDR_1_LASTRAMADDR
    RegDesc { addr: 0x6008, offs: 0, leng: 2, dflt: 2, flags: 0x04 }, // 1042: CAPTURE_RAM_CLOCK_GATE_OVERRIDE_CLK_OVERRIDE
    RegDesc { addr: 0x6009, offs: 0, leng: 1, dflt: 0, flags: 0x06 }, // 1043: CAPTURE_RAM_DUMMY0_DUMMY0
];

// ---------------------------------------------------------------------------
// Register name constants (C: `#define SX1302_REG_* <index>`)
// ---------------------------------------------------------------------------

pub const SX1302_REG_COMMON_PAGE_PAGE: u16 = 0;
pub const SX1302_REG_COMMON_CTRL0_CLK32_RIF_CTRL: u16 = 1;
pub const SX1302_REG_COMMON_CTRL0_HOST_RADIO_CTRL: u16 = 2;
pub const SX1302_REG_COMMON_CTRL0_RADIO_MISC_EN: u16 = 3;
pub const SX1302_REG_COMMON_CTRL0_SX1261_MODE_RADIO_B: u16 = 4;
pub const SX1302_REG_COMMON_CTRL0_SX1261_MODE_RADIO_A: u16 = 5;
pub const SX1302_REG_COMMON_CTRL1_SWAP_IQ_RADIO_B: u16 = 6;
pub const SX1302_REG_COMMON_CTRL1_SAMPLING_EDGE_RADIO_B: u16 = 7;
pub const SX1302_REG_COMMON_CTRL1_SWAP_IQ_RADIO_A: u16 = 8;
pub const SX1302_REG_COMMON_CTRL1_SAMPLING_EDGE_RADIO_A: u16 = 9;
pub const SX1302_REG_COMMON_SPI_DIV_RATIO_SPI_HALF_PERIOD: u16 = 10;
pub const SX1302_REG_COMMON_RADIO_SELECT_RADIO_SELECT: u16 = 11;
pub const SX1302_REG_COMMON_GEN_GLOBAL_EN: u16 = 12;
pub const SX1302_REG_COMMON_GEN_FSK_MODEM_ENABLE: u16 = 13;
pub const SX1302_REG_COMMON_GEN_CONCENTRATOR_MODEM_ENABLE: u16 = 14;
pub const SX1302_REG_COMMON_GEN_MBWSSF_MODEM_ENABLE: u16 = 15;
pub const SX1302_REG_COMMON_VERSION_VERSION: u16 = 16;
pub const SX1302_REG_COMMON_DUMMY_DUMMY: u16 = 17;
pub const SX1302_REG_AGC_MCU_CTRL_CLK_EN: u16 = 18;
pub const SX1302_REG_AGC_MCU_CTRL_FORCE_HOST_FE_CTRL: u16 = 19;
pub const SX1302_REG_AGC_MCU_CTRL_MCU_CLEAR: u16 = 20;
pub const SX1302_REG_AGC_MCU_CTRL_HOST_PROG: u16 = 21;
pub const SX1302_REG_AGC_MCU_CTRL_PARITY_ERROR: u16 = 22;
pub const SX1302_REG_AGC_MCU_MCU_AGC_STATUS_MCU_AGC_STATUS: u16 = 23;
pub const SX1302_REG_AGC_MCU_PA_GAIN_PA_B_GAIN: u16 = 24;
pub const SX1302_REG_AGC_MCU_PA_GAIN_PA_A_GAIN: u16 = 25;
pub const SX1302_REG_AGC_MCU_RF_EN_A_RADIO_RST: u16 = 26;
pub const SX1302_REG_AGC_MCU_RF_EN_A_RADIO_EN: u16 = 27;
pub const SX1302_REG_AGC_MCU_RF_EN_A_PA_EN: u16 = 28;
pub const SX1302_REG_AGC_MCU_RF_EN_A_LNA_EN: u16 = 29;
pub const SX1302_REG_AGC_MCU_RF_EN_B_RADIO_RST: u16 = 30;
pub const SX1302_REG_AGC_MCU_RF_EN_B_RADIO_EN: u16 = 31;
pub const SX1302_REG_AGC_MCU_RF_EN_B_PA_EN: u16 = 32;
pub const SX1302_REG_AGC_MCU_RF_EN_B_LNA_EN: u16 = 33;
pub const SX1302_REG_AGC_MCU_LUT_TABLE_A_PA_LUT: u16 = 34;
pub const SX1302_REG_AGC_MCU_LUT_TABLE_A_LNA_LUT: u16 = 35;
pub const SX1302_REG_AGC_MCU_LUT_TABLE_B_PA_LUT: u16 = 36;
pub const SX1302_REG_AGC_MCU_LUT_TABLE_B_LNA_LUT: u16 = 37;
pub const SX1302_REG_AGC_MCU_UART_CFG_MSBF: u16 = 38;
pub const SX1302_REG_AGC_MCU_UART_CFG_PAR_EN: u16 = 39;
pub const SX1302_REG_AGC_MCU_UART_CFG_PAR_MODE: u16 = 40;
pub const SX1302_REG_AGC_MCU_UART_CFG_START_LEN: u16 = 41;
pub const SX1302_REG_AGC_MCU_UART_CFG_STOP_LEN: u16 = 42;
pub const SX1302_REG_AGC_MCU_UART_CFG_WORD_LEN: u16 = 43;
pub const SX1302_REG_AGC_MCU_UART_CFG2_BIT_RATE: u16 = 44;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE3_MCU_MAIL_BOX_WR_DATA: u16 = 45;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE2_MCU_MAIL_BOX_WR_DATA: u16 = 46;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE1_MCU_MAIL_BOX_WR_DATA: u16 = 47;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_WR_DATA_BYTE0_MCU_MAIL_BOX_WR_DATA: u16 = 48;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE3_MCU_MAIL_BOX_RD_DATA: u16 = 49;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE2_MCU_MAIL_BOX_RD_DATA: u16 = 50;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE1_MCU_MAIL_BOX_RD_DATA: u16 = 51;
pub const SX1302_REG_AGC_MCU_MCU_MAIL_BOX_RD_DATA_BYTE0_MCU_MAIL_BOX_RD_DATA: u16 = 52;
pub const SX1302_REG_AGC_MCU_DUMMY_DUMMY3: u16 = 53;
pub const SX1302_REG_CLK_CTRL_CLK_SEL_CLKDIV_EN: u16 = 54;
pub const SX1302_REG_CLK_CTRL_CLK_SEL_CLK_RADIO_B_SEL: u16 = 55;
pub const SX1302_REG_CLK_CTRL_CLK_SEL_CLK_RADIO_A_SEL: u16 = 56;
pub const SX1302_REG_CLK_CTRL_DUMMY_DUMMY: u16 = 57;
pub const SX1302_REG_TX_TOP_A_TX_TRIG_TX_FSM_CLR: u16 = 58;
pub const SX1302_REG_TX_TOP_A_TX_TRIG_TX_TRIG_GPS: u16 = 59;
pub const SX1302_REG_TX_TOP_A_TX_TRIG_TX_TRIG_DELAYED: u16 = 60;
pub const SX1302_REG_TX_TOP_A_TX_TRIG_TX_TRIG_IMMEDIATE: u16 = 61;
pub const SX1302_REG_TX_TOP_A_TIMER_TRIG_BYTE3_TIMER_DELAYED_TRIG: u16 = 62;
pub const SX1302_REG_TX_TOP_A_TIMER_TRIG_BYTE2_TIMER_DELAYED_TRIG: u16 = 63;
pub const SX1302_REG_TX_TOP_A_TIMER_TRIG_BYTE1_TIMER_DELAYED_TRIG: u16 = 64;
pub const SX1302_REG_TX_TOP_A_TIMER_TRIG_BYTE0_TIMER_DELAYED_TRIG: u16 = 65;
pub const SX1302_REG_TX_TOP_A_TX_START_DELAY_MSB_TX_START_DELAY: u16 = 66;
pub const SX1302_REG_TX_TOP_A_TX_START_DELAY_LSB_TX_START_DELAY: u16 = 67;
pub const SX1302_REG_TX_TOP_A_TX_CTRL_WRITE_BUFFER: u16 = 68;
pub const SX1302_REG_TX_TOP_A_TX_RAMP_DURATION_TX_RAMP_DURATION: u16 = 69;
pub const SX1302_REG_TX_TOP_A_GEN_CFG_0_MODULATION_TYPE: u16 = 70;
pub const SX1302_REG_TX_TOP_A_TEST_0_TX_ACTIVE_CTRL: u16 = 71;
pub const SX1302_REG_TX_TOP_A_TEST_0_TX_ACTIVE_SEL: u16 = 72;
pub const SX1302_REG_TX_TOP_A_TX_FLAG_TX_TIMEOUT: u16 = 73;
pub const SX1302_REG_TX_TOP_A_TX_FLAG_PKT_DONE: u16 = 74;
pub const SX1302_REG_TX_TOP_A_AGC_TX_BW_AGC_TX_BW: u16 = 75;
pub const SX1302_REG_TX_TOP_A_AGC_TX_PWR_AGC_TX_PWR: u16 = 76;
pub const SX1302_REG_TX_TOP_A_TIMEOUT_CNT_BYTE_2_TIMEOUT_CNT: u16 = 77;
pub const SX1302_REG_TX_TOP_A_TIMEOUT_CNT_BYTE_1_TIMEOUT_CNT: u16 = 78;
pub const SX1302_REG_TX_TOP_A_TIMEOUT_CNT_BYTE_0_TIMEOUT_CNT: u16 = 79;
pub const SX1302_REG_TX_TOP_A_TX_FSM_STATUS_TX_STATUS: u16 = 80;
pub const SX1302_REG_TX_TOP_A_DUMMY_CONTROL_DUMMY: u16 = 81;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_PLL_DIV_CTRL: u16 = 82;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_TX_CLK_EDGE: u16 = 83;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_TX_MODE: u16 = 84;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_TX_IF_DST: u16 = 85;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL_TX_IF_SRC: u16 = 86;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL2_SX125X_IQ_INVERT: u16 = 87;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_CTRL2_PLL_DIV_CTRL_AGC: u16 = 88;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_IQ_GAIN_IQ_GAIN: u16 = 89;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_I_OFFSET_I_OFFSET: u16 = 90;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_Q_OFFSET_Q_OFFSET: u16 = 91;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_FREQ_RF_H_FREQ_RF: u16 = 92;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_FREQ_RF_M_FREQ_RF: u16 = 93;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_FREQ_RF_L_FREQ_RF: u16 = 94;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_FREQ_DEV_H_FREQ_DEV: u16 = 95;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_FREQ_DEV_L_FREQ_DEV: u16 = 96;
pub const SX1302_REG_TX_TOP_A_TX_RFFE_IF_TEST_MOD_FREQ: u16 = 97;
pub const SX1302_REG_TX_TOP_A_DUMMY_MODULATOR_DUMMY: u16 = 98;
pub const SX1302_REG_TX_TOP_A_FSK_PKT_LEN_PKT_LENGTH: u16 = 99;
pub const SX1302_REG_TX_TOP_A_FSK_CFG_0_TX_CONT: u16 = 100;
pub const SX1302_REG_TX_TOP_A_FSK_CFG_0_CRC_IBM: u16 = 101;
pub const SX1302_REG_TX_TOP_A_FSK_CFG_0_DCFREE_ENC: u16 = 102;
pub const SX1302_REG_TX_TOP_A_FSK_CFG_0_CRC_EN: u16 = 103;
pub const SX1302_REG_TX_TOP_A_FSK_CFG_0_PKT_MODE: u16 = 104;
pub const SX1302_REG_TX_TOP_A_FSK_PREAMBLE_SIZE_MSB_PREAMBLE_SIZE: u16 = 105;
pub const SX1302_REG_TX_TOP_A_FSK_PREAMBLE_SIZE_LSB_PREAMBLE_SIZE: u16 = 106;
pub const SX1302_REG_TX_TOP_A_FSK_BIT_RATE_MSB_BIT_RATE: u16 = 107;
pub const SX1302_REG_TX_TOP_A_FSK_BIT_RATE_LSB_BIT_RATE: u16 = 108;
pub const SX1302_REG_TX_TOP_A_FSK_MOD_FSK_REF_PATTERN_SIZE: u16 = 109;
pub const SX1302_REG_TX_TOP_A_FSK_MOD_FSK_PREAMBLE_SEQ: u16 = 110;
pub const SX1302_REG_TX_TOP_A_FSK_MOD_FSK_REF_PATTERN_EN: u16 = 111;
pub const SX1302_REG_TX_TOP_A_FSK_MOD_FSK_GAUSSIAN_SELECT_BT: u16 = 112;
pub const SX1302_REG_TX_TOP_A_FSK_MOD_FSK_GAUSSIAN_EN: u16 = 113;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN: u16 = 114;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN: u16 = 115;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN: u16 = 116;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN: u16 = 117;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN: u16 = 118;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN: u16 = 119;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN: u16 = 120;
pub const SX1302_REG_TX_TOP_A_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN: u16 = 121;
pub const SX1302_REG_TX_TOP_A_DUMMY_GSFK_DUMMY: u16 = 122;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_0_MODEM_BW: u16 = 123;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_0_MODEM_SF: u16 = 124;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_1_PPM_OFFSET_HDR_CTRL: u16 = 125;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_1_PPM_OFFSET: u16 = 126;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_1_POST_PREAMBLE_GAP_LONG: u16 = 127;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_1_CODING_RATE: u16 = 128;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_2_FINE_SYNCH_EN: u16 = 129;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_2_MODEM_EN: u16 = 130;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_2_CADRXTX: u16 = 131;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_2_IMPLICIT_HEADER: u16 = 132;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_2_CRC_EN: u16 = 133;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG0_3_PAYLOAD_LENGTH: u16 = 134;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_0_INT_STEP_ORIDE_EN: u16 = 135;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_0_INT_STEP_ORIDE: u16 = 136;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_1_MODEM_START: u16 = 137;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_1_HEADER_DIFF_MODE: u16 = 138;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_1_ZERO_PAD: u16 = 139;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_2_PREAMBLE_SYMB_NB: u16 = 140;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_3_PREAMBLE_SYMB_NB: u16 = 141;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_4_AUTO_ACK_INT_DELAY: u16 = 142;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_4_AUTO_ACK_RX: u16 = 143;
pub const SX1302_REG_TX_TOP_A_TXRX_CFG1_4_AUTO_ACK_TX: u16 = 144;
pub const SX1302_REG_TX_TOP_A_TX_CFG0_0_CHIRP_LOWPASS: u16 = 145;
pub const SX1302_REG_TX_TOP_A_TX_CFG0_0_PPM_OFFSET_SIG: u16 = 146;
pub const SX1302_REG_TX_TOP_A_TX_CFG0_0_CONTCHIRP: u16 = 147;
pub const SX1302_REG_TX_TOP_A_TX_CFG0_0_CHIRP_INVERT: u16 = 148;
pub const SX1302_REG_TX_TOP_A_TX_CFG0_0_CONTINUOUS: u16 = 149;
pub const SX1302_REG_TX_TOP_A_TX_CFG0_1_POWER_RANGING: u16 = 150;
pub const SX1302_REG_TX_TOP_A_TX_CFG1_0_FRAME_NB: u16 = 151;
pub const SX1302_REG_TX_TOP_A_TX_CFG1_1_HOP_CTRL: u16 = 152;
pub const SX1302_REG_TX_TOP_A_TX_CFG1_1_IFS: u16 = 153;
pub const SX1302_REG_TX_TOP_A_FRAME_SYNCH_0_AUTO_SCALE: u16 = 154;
pub const SX1302_REG_TX_TOP_A_FRAME_SYNCH_0_DROP_ON_SYNCH: u16 = 155;
pub const SX1302_REG_TX_TOP_A_FRAME_SYNCH_0_GAIN: u16 = 156;
pub const SX1302_REG_TX_TOP_A_FRAME_SYNCH_0_PEAK1_POS: u16 = 157;
pub const SX1302_REG_TX_TOP_A_FRAME_SYNCH_1_FINETIME_ON_LAST: u16 = 158;
pub const SX1302_REG_TX_TOP_A_FRAME_SYNCH_1_TIMEOUT_OPT: u16 = 159;
pub const SX1302_REG_TX_TOP_A_FRAME_SYNCH_1_PEAK2_POS: u16 = 160;
pub const SX1302_REG_TX_TOP_A_LORA_TX_STATE_STATUS: u16 = 161;
pub const SX1302_REG_TX_TOP_A_LORA_TX_FLAG_FRAME_DONE: u16 = 162;
pub const SX1302_REG_TX_TOP_A_LORA_TX_FLAG_CONT_DONE: u16 = 163;
pub const SX1302_REG_TX_TOP_A_LORA_TX_FLAG_PLD_DONE: u16 = 164;
pub const SX1302_REG_TX_TOP_A_DUMMY_LORA_DUMMY: u16 = 165;
pub const SX1302_REG_TX_TOP_B_TX_TRIG_TX_FSM_CLR: u16 = 166;
pub const SX1302_REG_TX_TOP_B_TX_TRIG_TX_TRIG_GPS: u16 = 167;
pub const SX1302_REG_TX_TOP_B_TX_TRIG_TX_TRIG_DELAYED: u16 = 168;
pub const SX1302_REG_TX_TOP_B_TX_TRIG_TX_TRIG_IMMEDIATE: u16 = 169;
pub const SX1302_REG_TX_TOP_B_TIMER_TRIG_BYTE3_TIMER_DELAYED_TRIG: u16 = 170;
pub const SX1302_REG_TX_TOP_B_TIMER_TRIG_BYTE2_TIMER_DELAYED_TRIG: u16 = 171;
pub const SX1302_REG_TX_TOP_B_TIMER_TRIG_BYTE1_TIMER_DELAYED_TRIG: u16 = 172;
pub const SX1302_REG_TX_TOP_B_TIMER_TRIG_BYTE0_TIMER_DELAYED_TRIG: u16 = 173;
pub const SX1302_REG_TX_TOP_B_TX_START_DELAY_MSB_TX_START_DELAY: u16 = 174;
pub const SX1302_REG_TX_TOP_B_TX_START_DELAY_LSB_TX_START_DELAY: u16 = 175;
pub const SX1302_REG_TX_TOP_B_TX_CTRL_WRITE_BUFFER: u16 = 176;
pub const SX1302_REG_TX_TOP_B_TX_RAMP_DURATION_TX_RAMP_DURATION: u16 = 177;
pub const SX1302_REG_TX_TOP_B_GEN_CFG_0_MODULATION_TYPE: u16 = 178;
pub const SX1302_REG_TX_TOP_B_TEST_0_TX_ACTIVE_CTRL: u16 = 179;
pub const SX1302_REG_TX_TOP_B_TEST_0_TX_ACTIVE_SEL: u16 = 180;
pub const SX1302_REG_TX_TOP_B_TX_FLAG_TX_TIMEOUT: u16 = 181;
pub const SX1302_REG_TX_TOP_B_TX_FLAG_PKT_DONE: u16 = 182;
pub const SX1302_REG_TX_TOP_B_AGC_TX_BW_AGC_TX_BW: u16 = 183;
pub const SX1302_REG_TX_TOP_B_AGC_TX_PWR_AGC_TX_PWR: u16 = 184;
pub const SX1302_REG_TX_TOP_B_TIMEOUT_CNT_BYTE_2_TIMEOUT_CNT: u16 = 185;
pub const SX1302_REG_TX_TOP_B_TIMEOUT_CNT_BYTE_1_TIMEOUT_CNT: u16 = 186;
pub const SX1302_REG_TX_TOP_B_TIMEOUT_CNT_BYTE_0_TIMEOUT_CNT: u16 = 187;
pub const SX1302_REG_TX_TOP_B_TX_FSM_STATUS_TX_STATUS: u16 = 188;
pub const SX1302_REG_TX_TOP_B_DUMMY_CONTROL_DUMMY: u16 = 189;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_PLL_DIV_CTRL: u16 = 190;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_TX_CLK_EDGE: u16 = 191;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_TX_MODE: u16 = 192;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_TX_IF_DST: u16 = 193;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL_TX_IF_SRC: u16 = 194;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL2_SX125X_IQ_INVERT: u16 = 195;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_CTRL2_PLL_DIV_CTRL_AGC: u16 = 196;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_IQ_GAIN_IQ_GAIN: u16 = 197;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_I_OFFSET_I_OFFSET: u16 = 198;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_Q_OFFSET_Q_OFFSET: u16 = 199;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_FREQ_RF_H_FREQ_RF: u16 = 200;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_FREQ_RF_M_FREQ_RF: u16 = 201;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_FREQ_RF_L_FREQ_RF: u16 = 202;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_FREQ_DEV_H_FREQ_DEV: u16 = 203;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_FREQ_DEV_L_FREQ_DEV: u16 = 204;
pub const SX1302_REG_TX_TOP_B_TX_RFFE_IF_TEST_MOD_FREQ: u16 = 205;
pub const SX1302_REG_TX_TOP_B_DUMMY_MODULATOR_DUMMY: u16 = 206;
pub const SX1302_REG_TX_TOP_B_FSK_PKT_LEN_PKT_LENGTH: u16 = 207;
pub const SX1302_REG_TX_TOP_B_FSK_CFG_0_TX_CONT: u16 = 208;
pub const SX1302_REG_TX_TOP_B_FSK_CFG_0_CRC_IBM: u16 = 209;
pub const SX1302_REG_TX_TOP_B_FSK_CFG_0_DCFREE_ENC: u16 = 210;
pub const SX1302_REG_TX_TOP_B_FSK_CFG_0_CRC_EN: u16 = 211;
pub const SX1302_REG_TX_TOP_B_FSK_CFG_0_PKT_MODE: u16 = 212;
pub const SX1302_REG_TX_TOP_B_FSK_PREAMBLE_SIZE_MSB_PREAMBLE_SIZE: u16 = 213;
pub const SX1302_REG_TX_TOP_B_FSK_PREAMBLE_SIZE_LSB_PREAMBLE_SIZE: u16 = 214;
pub const SX1302_REG_TX_TOP_B_FSK_BIT_RATE_MSB_BIT_RATE: u16 = 215;
pub const SX1302_REG_TX_TOP_B_FSK_BIT_RATE_LSB_BIT_RATE: u16 = 216;
pub const SX1302_REG_TX_TOP_B_FSK_MOD_FSK_REF_PATTERN_SIZE: u16 = 217;
pub const SX1302_REG_TX_TOP_B_FSK_MOD_FSK_PREAMBLE_SEQ: u16 = 218;
pub const SX1302_REG_TX_TOP_B_FSK_MOD_FSK_REF_PATTERN_EN: u16 = 219;
pub const SX1302_REG_TX_TOP_B_FSK_MOD_FSK_GAUSSIAN_SELECT_BT: u16 = 220;
pub const SX1302_REG_TX_TOP_B_FSK_MOD_FSK_GAUSSIAN_EN: u16 = 221;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN: u16 = 222;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN: u16 = 223;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN: u16 = 224;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN: u16 = 225;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN: u16 = 226;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN: u16 = 227;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN: u16 = 228;
pub const SX1302_REG_TX_TOP_B_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN: u16 = 229;
pub const SX1302_REG_TX_TOP_B_DUMMY_GSFK_DUMMY: u16 = 230;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_0_MODEM_BW: u16 = 231;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_0_MODEM_SF: u16 = 232;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_1_PPM_OFFSET_HDR_CTRL: u16 = 233;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_1_PPM_OFFSET: u16 = 234;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_1_POST_PREAMBLE_GAP_LONG: u16 = 235;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_1_CODING_RATE: u16 = 236;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_2_FINE_SYNCH_EN: u16 = 237;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_2_MODEM_EN: u16 = 238;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_2_CADRXTX: u16 = 239;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_2_IMPLICIT_HEADER: u16 = 240;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_2_CRC_EN: u16 = 241;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG0_3_PAYLOAD_LENGTH: u16 = 242;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_0_INT_STEP_ORIDE_EN: u16 = 243;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_0_INT_STEP_ORIDE: u16 = 244;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_1_MODEM_START: u16 = 245;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_1_HEADER_DIFF_MODE: u16 = 246;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_1_ZERO_PAD: u16 = 247;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_2_PREAMBLE_SYMB_NB: u16 = 248;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_3_PREAMBLE_SYMB_NB: u16 = 249;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_4_AUTO_ACK_INT_DELAY: u16 = 250;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_4_AUTO_ACK_RX: u16 = 251;
pub const SX1302_REG_TX_TOP_B_TXRX_CFG1_4_AUTO_ACK_TX: u16 = 252;
pub const SX1302_REG_TX_TOP_B_TX_CFG0_0_CHIRP_LOWPASS: u16 = 253;
pub const SX1302_REG_TX_TOP_B_TX_CFG0_0_PPM_OFFSET_SIG: u16 = 254;
pub const SX1302_REG_TX_TOP_B_TX_CFG0_0_CONTCHIRP: u16 = 255;
pub const SX1302_REG_TX_TOP_B_TX_CFG0_0_CHIRP_INVERT: u16 = 256;
pub const SX1302_REG_TX_TOP_B_TX_CFG0_0_CONTINUOUS: u16 = 257;
pub const SX1302_REG_TX_TOP_B_TX_CFG0_1_POWER_RANGING: u16 = 258;
pub const SX1302_REG_TX_TOP_B_TX_CFG1_0_FRAME_NB: u16 = 259;
pub const SX1302_REG_TX_TOP_B_TX_CFG1_1_HOP_CTRL: u16 = 260;
pub const SX1302_REG_TX_TOP_B_TX_CFG1_1_IFS: u16 = 261;
pub const SX1302_REG_TX_TOP_B_FRAME_SYNCH_0_AUTO_SCALE: u16 = 262;
pub const SX1302_REG_TX_TOP_B_FRAME_SYNCH_0_DROP_ON_SYNCH: u16 = 263;
pub const SX1302_REG_TX_TOP_B_FRAME_SYNCH_0_GAIN: u16 = 264;
pub const SX1302_REG_TX_TOP_B_FRAME_SYNCH_0_PEAK1_POS: u16 = 265;
pub const SX1302_REG_TX_TOP_B_FRAME_SYNCH_1_FINETIME_ON_LAST: u16 = 266;
pub const SX1302_REG_TX_TOP_B_FRAME_SYNCH_1_TIMEOUT_OPT: u16 = 267;
pub const SX1302_REG_TX_TOP_B_FRAME_SYNCH_1_PEAK2_POS: u16 = 268;
pub const SX1302_REG_TX_TOP_B_LORA_TX_STATE_STATUS: u16 = 269;
pub const SX1302_REG_TX_TOP_B_LORA_TX_FLAG_FRAME_DONE: u16 = 270;
pub const SX1302_REG_TX_TOP_B_LORA_TX_FLAG_CONT_DONE: u16 = 271;
pub const SX1302_REG_TX_TOP_B_LORA_TX_FLAG_PLD_DONE: u16 = 272;
pub const SX1302_REG_TX_TOP_B_DUMMY_LORA_DUMMY: u16 = 273;
pub const SX1302_REG_GPIO_GPIO_DIR_H_DIRECTION: u16 = 274;
pub const SX1302_REG_GPIO_GPIO_DIR_L_DIRECTION: u16 = 275;
pub const SX1302_REG_GPIO_GPIO_OUT_H_OUT_VALUE: u16 = 276;
pub const SX1302_REG_GPIO_GPIO_OUT_L_OUT_VALUE: u16 = 277;
pub const SX1302_REG_GPIO_GPIO_IN_H_IN_VALUE: u16 = 278;
pub const SX1302_REG_GPIO_GPIO_IN_L_IN_VALUE: u16 = 279;
pub const SX1302_REG_GPIO_GPIO_PD_H_PD_VALUE: u16 = 280;
pub const SX1302_REG_GPIO_GPIO_PD_L_PD_VALUE: u16 = 281;
pub const SX1302_REG_GPIO_GPIO_SEL_0_SELECTION: u16 = 282;
pub const SX1302_REG_GPIO_GPIO_SEL_1_SELECTION: u16 = 283;
pub const SX1302_REG_GPIO_GPIO_SEL_2_SELECTION: u16 = 284;
pub const SX1302_REG_GPIO_GPIO_SEL_3_SELECTION: u16 = 285;
pub const SX1302_REG_GPIO_GPIO_SEL_4_SELECTION: u16 = 286;
pub const SX1302_REG_GPIO_GPIO_SEL_5_SELECTION: u16 = 287;
pub const SX1302_REG_GPIO_GPIO_SEL_6_SELECTION: u16 = 288;
pub const SX1302_REG_GPIO_GPIO_SEL_7_SELECTION: u16 = 289;
pub const SX1302_REG_GPIO_GPIO_SEL_8_11_GPIO_11_9_SEL: u16 = 290;
pub const SX1302_REG_GPIO_GPIO_SEL_8_11_GPIO_8_SEL: u16 = 291;
pub const SX1302_REG_GPIO_HOST_IRQ_TX_TIMEOUT_B: u16 = 292;
pub const SX1302_REG_GPIO_HOST_IRQ_TX_TIMEOUT_A: u16 = 293;
pub const SX1302_REG_GPIO_HOST_IRQ_TX_DONE_B: u16 = 294;
pub const SX1302_REG_GPIO_HOST_IRQ_TX_DONE_A: u16 = 295;
pub const SX1302_REG_GPIO_HOST_IRQ_TIMESTAMP: u16 = 296;
pub const SX1302_REG_GPIO_HOST_IRQ_RX_BUFFER_WATERMARK: u16 = 297;
pub const SX1302_REG_GPIO_HOST_IRQ_EN_TX_TIMEOUT_B: u16 = 298;
pub const SX1302_REG_GPIO_HOST_IRQ_EN_TX_TIMEOUT_A: u16 = 299;
pub const SX1302_REG_GPIO_HOST_IRQ_EN_TX_DONE_B: u16 = 300;
pub const SX1302_REG_GPIO_HOST_IRQ_EN_TX_DONE_A: u16 = 301;
pub const SX1302_REG_GPIO_HOST_IRQ_EN_TIMESTAMP: u16 = 302;
pub const SX1302_REG_GPIO_HOST_IRQ_EN_RX_BUFFER_WATERMARK: u16 = 303;
pub const SX1302_REG_GPIO_DUMMY_DUMMY: u16 = 304;
pub const SX1302_REG_TIMESTAMP_GPS_CTRL_GPS_POL: u16 = 305;
pub const SX1302_REG_TIMESTAMP_GPS_CTRL_GPS_EN: u16 = 306;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_PPS_MSB2_TIMESTAMP_PPS: u16 = 307;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_PPS_MSB1_TIMESTAMP_PPS: u16 = 308;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_PPS_LSB2_TIMESTAMP_PPS: u16 = 309;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_PPS_LSB1_TIMESTAMP_PPS: u16 = 310;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_MSB2_TIMESTAMP: u16 = 311;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_MSB1_TIMESTAMP: u16 = 312;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_LSB2_TIMESTAMP: u16 = 313;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_LSB1_TIMESTAMP: u16 = 314;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_SET3_TIMESTAMP: u16 = 315;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_SET2_TIMESTAMP: u16 = 316;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_SET1_TIMESTAMP: u16 = 317;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_SET0_TIMESTAMP: u16 = 318;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_IRQ_3_TIMESTAMP: u16 = 319;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_IRQ_2_TIMESTAMP: u16 = 320;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_IRQ_1_TIMESTAMP: u16 = 321;
pub const SX1302_REG_TIMESTAMP_TIMESTAMP_IRQ_0_TIMESTAMP: u16 = 322;
pub const SX1302_REG_TIMESTAMP_DUMMY_DUMMY: u16 = 323;
pub const SX1302_REG_RX_TOP_FREQ_0_MSB_IF_FREQ_0: u16 = 324;
pub const SX1302_REG_RX_TOP_FREQ_0_LSB_IF_FREQ_0: u16 = 325;
pub const SX1302_REG_RX_TOP_FREQ_1_MSB_IF_FREQ_1: u16 = 326;
pub const SX1302_REG_RX_TOP_FREQ_1_LSB_IF_FREQ_1: u16 = 327;
pub const SX1302_REG_RX_TOP_FREQ_2_MSB_IF_FREQ_2: u16 = 328;
pub const SX1302_REG_RX_TOP_FREQ_2_LSB_IF_FREQ_2: u16 = 329;
pub const SX1302_REG_RX_TOP_FREQ_3_MSB_IF_FREQ_3: u16 = 330;
pub const SX1302_REG_RX_TOP_FREQ_3_LSB_IF_FREQ_3: u16 = 331;
pub const SX1302_REG_RX_TOP_FREQ_4_MSB_IF_FREQ_4: u16 = 332;
pub const SX1302_REG_RX_TOP_FREQ_4_LSB_IF_FREQ_4: u16 = 333;
pub const SX1302_REG_RX_TOP_FREQ_5_MSB_IF_FREQ_5: u16 = 334;
pub const SX1302_REG_RX_TOP_FREQ_5_LSB_IF_FREQ_5: u16 = 335;
pub const SX1302_REG_RX_TOP_FREQ_6_MSB_IF_FREQ_6: u16 = 336;
pub const SX1302_REG_RX_TOP_FREQ_6_LSB_IF_FREQ_6: u16 = 337;
pub const SX1302_REG_RX_TOP_FREQ_7_MSB_IF_FREQ_7: u16 = 338;
pub const SX1302_REG_RX_TOP_FREQ_7_LSB_IF_FREQ_7: u16 = 339;
pub const SX1302_REG_RX_TOP_RADIO_SELECT_RADIO_SELECT: u16 = 340;
pub const SX1302_REG_RX_TOP_RSSI_CONTROL_RSSI_FILTER_ALPHA: u16 = 341;
pub const SX1302_REG_RX_TOP_RSSI_CONTROL_SELECT_RSSI: u16 = 342;
pub const SX1302_REG_RX_TOP_RSSI_DEF_VALUE_CHAN_RSSI_DEF_VALUE: u16 = 343;
pub const SX1302_REG_RX_TOP_CHANN_DAGC_CFG1_CHAN_DAGC_THRESHOLD_HIGH: u16 = 344;
pub const SX1302_REG_RX_TOP_CHANN_DAGC_CFG2_CHAN_DAGC_THRESHOLD_LOW: u16 = 345;
pub const SX1302_REG_RX_TOP_CHANN_DAGC_CFG3_CHAN_DAGC_MAX_ATTEN: u16 = 346;
pub const SX1302_REG_RX_TOP_CHANN_DAGC_CFG3_CHAN_DAGC_MIN_ATTEN: u16 = 347;
pub const SX1302_REG_RX_TOP_CHANN_DAGC_CFG4_CHAN_DAGC_STEP: u16 = 348;
pub const SX1302_REG_RX_TOP_CHANN_DAGC_CFG5_CHAN_DAGC_MODE: u16 = 349;
pub const SX1302_REG_RX_TOP_RSSI_VALUE_CHAN_RSSI: u16 = 350;
pub const SX1302_REG_RX_TOP_GAIN_CONTROL_CHAN_GAIN_VALID: u16 = 351;
pub const SX1302_REG_RX_TOP_GAIN_CONTROL_CHAN_GAIN: u16 = 352;
pub const SX1302_REG_RX_TOP_CLK_CONTROL_CHAN_CLK_EN: u16 = 353;
pub const SX1302_REG_RX_TOP_DUMMY0_DUMMY0: u16 = 354;
pub const SX1302_REG_RX_TOP_CORR_CLOCK_ENABLE_CLK_EN: u16 = 355;
pub const SX1302_REG_RX_TOP_CORRELATOR_EN_CORR_EN: u16 = 356;
pub const SX1302_REG_RX_TOP_CORRELATOR_SF_EN_CORR_SF_EN: u16 = 357;
pub const SX1302_REG_RX_TOP_CORRELATOR_ENABLE_ONLY_FIRST_DET_EDGE_ENABLE_ONLY_FIRST_DET_EDGE: u16 = 358;
pub const SX1302_REG_RX_TOP_CORRELATOR_ENABLE_ACC_CLEAR_ENABLE_CORR_ACC_CLEAR: u16 = 359;
pub const SX1302_REG_RX_TOP_SF5_CFG1_ACC_WIN_LEN: u16 = 360;
pub const SX1302_REG_RX_TOP_SF5_CFG1_ACC_PEAK_SUM_EN: u16 = 361;
pub const SX1302_REG_RX_TOP_SF5_CFG1_ACC_PEAK_POS_SEL: u16 = 362;
pub const SX1302_REG_RX_TOP_SF5_CFG1_ACC_COEFF: u16 = 363;
pub const SX1302_REG_RX_TOP_SF5_CFG1_ACC_AUTO_RESCALE: u16 = 364;
pub const SX1302_REG_RX_TOP_SF5_CFG1_ACC_2_SAME_PEAKS: u16 = 365;
pub const SX1302_REG_RX_TOP_SF5_CFG2_ACC_MIN2: u16 = 366;
pub const SX1302_REG_RX_TOP_SF5_CFG2_ACC_PNR: u16 = 367;
pub const SX1302_REG_RX_TOP_SF5_CFG3_MIN_SINGLE_PEAK: u16 = 368;
pub const SX1302_REG_RX_TOP_SF5_CFG4_MSP_PNR: u16 = 369;
pub const SX1302_REG_RX_TOP_SF5_CFG5_MSP2_PNR: u16 = 370;
pub const SX1302_REG_RX_TOP_SF5_CFG6_MSP_PEAK_NB: u16 = 371;
pub const SX1302_REG_RX_TOP_SF5_CFG6_MSP_CNT_MODE: u16 = 372;
pub const SX1302_REG_RX_TOP_SF5_CFG6_MSP_POS_SEL: u16 = 373;
pub const SX1302_REG_RX_TOP_SF5_CFG7_MSP2_PEAK_NB: u16 = 374;
pub const SX1302_REG_RX_TOP_SF5_CFG7_NOISE_COEFF: u16 = 375;
pub const SX1302_REG_RX_TOP_SF6_CFG1_ACC_WIN_LEN: u16 = 376;
pub const SX1302_REG_RX_TOP_SF6_CFG1_ACC_PEAK_SUM_EN: u16 = 377;
pub const SX1302_REG_RX_TOP_SF6_CFG1_ACC_PEAK_POS_SEL: u16 = 378;
pub const SX1302_REG_RX_TOP_SF6_CFG1_ACC_COEFF: u16 = 379;
pub const SX1302_REG_RX_TOP_SF6_CFG1_ACC_AUTO_RESCALE: u16 = 380;
pub const SX1302_REG_RX_TOP_SF6_CFG1_ACC_2_SAME_PEAKS: u16 = 381;
pub const SX1302_REG_RX_TOP_SF6_CFG2_ACC_MIN2: u16 = 382;
pub const SX1302_REG_RX_TOP_SF6_CFG2_ACC_PNR: u16 = 383;
pub const SX1302_REG_RX_TOP_SF6_CFG3_MIN_SINGLE_PEAK: u16 = 384;
pub const SX1302_REG_RX_TOP_SF6_CFG4_MSP_PNR: u16 = 385;
pub const SX1302_REG_RX_TOP_SF6_CFG5_MSP2_PNR: u16 = 386;
pub const SX1302_REG_RX_TOP_SF6_CFG6_MSP_PEAK_NB: u16 = 387;
pub const SX1302_REG_RX_TOP_SF6_CFG6_MSP_CNT_MODE: u16 = 388;
pub const SX1302_REG_RX_TOP_SF6_CFG6_MSP_POS_SEL: u16 = 389;
pub const SX1302_REG_RX_TOP_SF6_CFG7_MSP2_PEAK_NB: u16 = 390;
pub const SX1302_REG_RX_TOP_SF6_CFG7_NOISE_COEFF: u16 = 391;
pub const SX1302_REG_RX_TOP_SF7_CFG1_ACC_WIN_LEN: u16 = 392;
pub const SX1302_REG_RX_TOP_SF7_CFG1_ACC_PEAK_SUM_EN: u16 = 393;
pub const SX1302_REG_RX_TOP_SF7_CFG1_ACC_PEAK_POS_SEL: u16 = 394;
pub const SX1302_REG_RX_TOP_SF7_CFG1_ACC_COEFF: u16 = 395;
pub const SX1302_REG_RX_TOP_SF7_CFG1_ACC_AUTO_RESCALE: u16 = 396;
pub const SX1302_REG_RX_TOP_SF7_CFG1_ACC_2_SAME_PEAKS: u16 = 397;
pub const SX1302_REG_RX_TOP_SF7_CFG2_ACC_MIN2: u16 = 398;
pub const SX1302_REG_RX_TOP_SF7_CFG2_ACC_PNR: u16 = 399;
pub const SX1302_REG_RX_TOP_SF7_CFG3_MIN_SINGLE_PEAK: u16 = 400;
pub const SX1302_REG_RX_TOP_SF7_CFG4_MSP_PNR: u16 = 401;
pub const SX1302_REG_RX_TOP_SF7_CFG5_MSP2_PNR: u16 = 402;
pub const SX1302_REG_RX_TOP_SF7_CFG6_MSP_PEAK_NB: u16 = 403;
pub const SX1302_REG_RX_TOP_SF7_CFG6_MSP_CNT_MODE: u16 = 404;
pub const SX1302_REG_RX_TOP_SF7_CFG6_MSP_POS_SEL: u16 = 405;
pub const SX1302_REG_RX_TOP_SF7_CFG7_MSP2_PEAK_NB: u16 = 406;
pub const SX1302_REG_RX_TOP_SF7_CFG7_NOISE_COEFF: u16 = 407;
pub const SX1302_REG_RX_TOP_SF8_CFG1_ACC_WIN_LEN: u16 = 408;
pub const SX1302_REG_RX_TOP_SF8_CFG1_ACC_PEAK_SUM_EN: u16 = 409;
pub const SX1302_REG_RX_TOP_SF8_CFG1_ACC_PEAK_POS_SEL: u16 = 410;
pub const SX1302_REG_RX_TOP_SF8_CFG1_ACC_COEFF: u16 = 411;
pub const SX1302_REG_RX_TOP_SF8_CFG1_ACC_AUTO_RESCALE: u16 = 412;
pub const SX1302_REG_RX_TOP_SF8_CFG1_ACC_2_SAME_PEAKS: u16 = 413;
pub const SX1302_REG_RX_TOP_SF8_CFG2_ACC_MIN2: u16 = 414;
pub const SX1302_REG_RX_TOP_SF8_CFG2_ACC_PNR: u16 = 415;
pub const SX1302_REG_RX_TOP_SF8_CFG3_MIN_SINGLE_PEAK: u16 = 416;
pub const SX1302_REG_RX_TOP_SF8_CFG4_MSP_PNR: u16 = 417;
pub const SX1302_REG_RX_TOP_SF8_CFG5_MSP2_PNR: u16 = 418;
pub const SX1302_REG_RX_TOP_SF8_CFG6_MSP_PEAK_NB: u16 = 419;
pub const SX1302_REG_RX_TOP_SF8_CFG6_MSP_CNT_MODE: u16 = 420;
pub const SX1302_REG_RX_TOP_SF8_CFG6_MSP_POS_SEL: u16 = 421;
pub const SX1302_REG_RX_TOP_SF8_CFG7_MSP2_PEAK_NB: u16 = 422;
pub const SX1302_REG_RX_TOP_SF8_CFG7_NOISE_COEFF: u16 = 423;
pub const SX1302_REG_RX_TOP_SF9_CFG1_ACC_WIN_LEN: u16 = 424;
pub const SX1302_REG_RX_TOP_SF9_CFG1_ACC_PEAK_SUM_EN: u16 = 425;
pub const SX1302_REG_RX_TOP_SF9_CFG1_ACC_PEAK_POS_SEL: u16 = 426;
pub const SX1302_REG_RX_TOP_SF9_CFG1_ACC_COEFF: u16 = 427;
pub const SX1302_REG_RX_TOP_SF9_CFG1_ACC_AUTO_RESCALE: u16 = 428;
pub const SX1302_REG_RX_TOP_SF9_CFG1_ACC_2_SAME_PEAKS: u16 = 429;
pub const SX1302_REG_RX_TOP_SF9_CFG2_ACC_MIN2: u16 = 430;
pub const SX1302_REG_RX_TOP_SF9_CFG2_ACC_PNR: u16 = 431;
pub const SX1302_REG_RX_TOP_SF9_CFG3_MIN_SINGLE_PEAK: u16 = 432;
pub const SX1302_REG_RX_TOP_SF9_CFG4_MSP_PNR: u16 = 433;
pub const SX1302_REG_RX_TOP_SF9_CFG5_MSP2_PNR: u16 = 434;
pub const SX1302_REG_RX_TOP_SF9_CFG6_MSP_PEAK_NB: u16 = 435;
pub const SX1302_REG_RX_TOP_SF9_CFG6_MSP_CNT_MODE: u16 = 436;
pub const SX1302_REG_RX_TOP_SF9_CFG6_MSP_POS_SEL: u16 = 437;
pub const SX1302_REG_RX_TOP_SF9_CFG7_MSP2_PEAK_NB: u16 = 438;
pub const SX1302_REG_RX_TOP_SF9_CFG7_NOISE_COEFF: u16 = 439;
pub const SX1302_REG_RX_TOP_SF10_CFG1_ACC_WIN_LEN: u16 = 440;
pub const SX1302_REG_RX_TOP_SF10_CFG1_ACC_PEAK_SUM_EN: u16 = 441;
pub const SX1302_REG_RX_TOP_SF10_CFG1_ACC_PEAK_POS_SEL: u16 = 442;
pub const SX1302_REG_RX_TOP_SF10_CFG1_ACC_COEFF: u16 = 443;
pub const SX1302_REG_RX_TOP_SF10_CFG1_ACC_AUTO_RESCALE: u16 = 444;
pub const SX1302_REG_RX_TOP_SF10_CFG1_ACC_2_SAME_PEAKS: u16 = 445;
pub const SX1302_REG_RX_TOP_SF10_CFG2_ACC_MIN2: u16 = 446;
pub const SX1302_REG_RX_TOP_SF10_CFG2_ACC_PNR: u16 = 447;
pub const SX1302_REG_RX_TOP_SF10_CFG3_MIN_SINGLE_PEAK: u16 = 448;
pub const SX1302_REG_RX_TOP_SF10_CFG4_MSP_PNR: u16 = 449;
pub const SX1302_REG_RX_TOP_SF10_CFG5_MSP2_PNR: u16 = 450;
pub const SX1302_REG_RX_TOP_SF10_CFG6_MSP_PEAK_NB: u16 = 451;
pub const SX1302_REG_RX_TOP_SF10_CFG6_MSP_CNT_MODE: u16 = 452;
pub const SX1302_REG_RX_TOP_SF10_CFG6_MSP_POS_SEL: u16 = 453;
pub const SX1302_REG_RX_TOP_SF10_CFG7_MSP2_PEAK_NB: u16 = 454;
pub const SX1302_REG_RX_TOP_SF10_CFG7_NOISE_COEFF: u16 = 455;
pub const SX1302_REG_RX_TOP_SF11_CFG1_ACC_WIN_LEN: u16 = 456;
pub const SX1302_REG_RX_TOP_SF11_CFG1_ACC_PEAK_SUM_EN: u16 = 457;
pub const SX1302_REG_RX_TOP_SF11_CFG1_ACC_PEAK_POS_SEL: u16 = 458;
pub const SX1302_REG_RX_TOP_SF11_CFG1_ACC_COEFF: u16 = 459;
pub const SX1302_REG_RX_TOP_SF11_CFG1_ACC_AUTO_RESCALE: u16 = 460;
pub const SX1302_REG_RX_TOP_SF11_CFG1_ACC_2_SAME_PEAKS: u16 = 461;
pub const SX1302_REG_RX_TOP_SF11_CFG2_ACC_MIN2: u16 = 462;
pub const SX1302_REG_RX_TOP_SF11_CFG2_ACC_PNR: u16 = 463;
pub const SX1302_REG_RX_TOP_SF11_CFG3_MIN_SINGLE_PEAK: u16 = 464;
pub const SX1302_REG_RX_TOP_SF11_CFG4_MSP_PNR: u16 = 465;
pub const SX1302_REG_RX_TOP_SF11_CFG5_MSP2_PNR: u16 = 466;
pub const SX1302_REG_RX_TOP_SF11_CFG6_MSP_PEAK_NB: u16 = 467;
pub const SX1302_REG_RX_TOP_SF11_CFG6_MSP_CNT_MODE: u16 = 468;
pub const SX1302_REG_RX_TOP_SF11_CFG6_MSP_POS_SEL: u16 = 469;
pub const SX1302_REG_RX_TOP_SF11_CFG7_MSP2_PEAK_NB: u16 = 470;
pub const SX1302_REG_RX_TOP_SF11_CFG7_NOISE_COEFF: u16 = 471;
pub const SX1302_REG_RX_TOP_SF12_CFG1_ACC_WIN_LEN: u16 = 472;
pub const SX1302_REG_RX_TOP_SF12_CFG1_ACC_PEAK_SUM_EN: u16 = 473;
pub const SX1302_REG_RX_TOP_SF12_CFG1_ACC_PEAK_POS_SEL: u16 = 474;
pub const SX1302_REG_RX_TOP_SF12_CFG1_ACC_COEFF: u16 = 475;
pub const SX1302_REG_RX_TOP_SF12_CFG1_ACC_AUTO_RESCALE: u16 = 476;
pub const SX1302_REG_RX_TOP_SF12_CFG1_ACC_2_SAME_PEAKS: u16 = 477;
pub const SX1302_REG_RX_TOP_SF12_CFG2_ACC_MIN2: u16 = 478;
pub const SX1302_REG_RX_TOP_SF12_CFG2_ACC_PNR: u16 = 479;
pub const SX1302_REG_RX_TOP_SF12_CFG3_MIN_SINGLE_PEAK: u16 = 480;
pub const SX1302_REG_RX_TOP_SF12_CFG4_MSP_PNR: u16 = 481;
pub const SX1302_REG_RX_TOP_SF12_CFG5_MSP2_PNR: u16 = 482;
pub const SX1302_REG_RX_TOP_SF12_CFG6_MSP_PEAK_NB: u16 = 483;
pub const SX1302_REG_RX_TOP_SF12_CFG6_MSP_CNT_MODE: u16 = 484;
pub const SX1302_REG_RX_TOP_SF12_CFG6_MSP_POS_SEL: u16 = 485;
pub const SX1302_REG_RX_TOP_SF12_CFG7_MSP2_PEAK_NB: u16 = 486;
pub const SX1302_REG_RX_TOP_SF12_CFG7_NOISE_COEFF: u16 = 487;
pub const SX1302_REG_RX_TOP_DUMMY1_DUMMY1: u16 = 488;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG1_BW_START: u16 = 489;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG1_AUTO_BW_RED: u16 = 490;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG1_NO_FAST_START: u16 = 491;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG1_BYPASS: u16 = 492;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG1_ENABLE: u16 = 493;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG2_BW_LOCKED: u16 = 494;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG2_BW: u16 = 495;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG3_BW_RED: u16 = 496;
pub const SX1302_REG_RX_TOP_DC_NOTCH_CFG4_IIR_DCC_TIME: u16 = 497;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_0_FIR1_COEFF_0: u16 = 498;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_1_FIR1_COEFF_1: u16 = 499;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_2_FIR1_COEFF_2: u16 = 500;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_3_FIR1_COEFF_3: u16 = 501;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_4_FIR1_COEFF_4: u16 = 502;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_5_FIR1_COEFF_5: u16 = 503;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_6_FIR1_COEFF_6: u16 = 504;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR1_7_FIR1_COEFF_7: u16 = 505;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_0_FIR2_COEFF_0: u16 = 506;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_1_FIR2_COEFF_1: u16 = 507;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_2_FIR2_COEFF_2: u16 = 508;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_3_FIR2_COEFF_3: u16 = 509;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_4_FIR2_COEFF_4: u16 = 510;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_5_FIR2_COEFF_5: u16 = 511;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_6_FIR2_COEFF_6: u16 = 512;
pub const SX1302_REG_RX_TOP_RX_DFE_FIR2_7_FIR2_COEFF_7: u16 = 513;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC0_RADIO_GAIN_RED_SEL: u16 = 514;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC0_RADIO_GAIN_RED_DB: u16 = 515;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC1_DC_COMP_EN: u16 = 516;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC1_FORCE_DEFAULT_FIR: u16 = 517;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC1_RSSI_EARLY_LATCH: u16 = 518;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC1_FREEZE_ON_SYNC: u16 = 519;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC2_DAGC_IN_COMP: u16 = 520;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC2_DAGC_FIR_HYST: u16 = 521;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC2_RSSI_MAX_SAMPLE: u16 = 522;
pub const SX1302_REG_RX_TOP_RX_DFE_AGC2_RSSI_MIN_SAMPLE: u16 = 523;
pub const SX1302_REG_RX_TOP_RX_DFE_GAIN0_DAGC_FIR_FAST: u16 = 524;
pub const SX1302_REG_RX_TOP_RX_DFE_GAIN0_FORCE_GAIN_FIR: u16 = 525;
pub const SX1302_REG_RX_TOP_RX_DFE_GAIN0_GAIN_FIR1: u16 = 526;
pub const SX1302_REG_RX_TOP_RX_DFE_GAIN0_GAIN_FIR2: u16 = 527;
pub const SX1302_REG_RX_TOP_DAGC_CFG_TARGET_LVL: u16 = 528;
pub const SX1302_REG_RX_TOP_DAGC_CFG_GAIN_INCR_STEP: u16 = 529;
pub const SX1302_REG_RX_TOP_DAGC_CFG_GAIN_DROP_COMP: u16 = 530;
pub const SX1302_REG_RX_TOP_DAGC_CFG_COMB_FILTER_EN: u16 = 531;
pub const SX1302_REG_RX_TOP_DAGC_CFG_NO_FREEZE_START: u16 = 532;
pub const SX1302_REG_RX_TOP_DAGC_CFG_FREEZE_ON_SYNC: u16 = 533;
pub const SX1302_REG_RX_TOP_DAGC_CNT0_SAMPLE: u16 = 534;
pub const SX1302_REG_RX_TOP_DAGC_CNT1_THR_M6: u16 = 535;
pub const SX1302_REG_RX_TOP_DAGC_CNT2_THR_M12: u16 = 536;
pub const SX1302_REG_RX_TOP_DAGC_CNT3_THR_M18: u16 = 537;
pub const SX1302_REG_RX_TOP_DAGC_CNT4_GAIN: u16 = 538;
pub const SX1302_REG_RX_TOP_DAGC_CNT4_FORCE_GAIN: u16 = 539;
pub const SX1302_REG_RX_TOP_TXRX_CFG1_PPM_OFFSET_HDR_CTRL: u16 = 540;
pub const SX1302_REG_RX_TOP_TXRX_CFG1_PPM_OFFSET: u16 = 541;
pub const SX1302_REG_RX_TOP_TXRX_CFG1_MODEM_EN: u16 = 542;
pub const SX1302_REG_RX_TOP_TXRX_CFG1_CODING_RATE: u16 = 543;
pub const SX1302_REG_RX_TOP_TXRX_CFG2_MODEM_START: u16 = 544;
pub const SX1302_REG_RX_TOP_TXRX_CFG2_CADRXTX: u16 = 545;
pub const SX1302_REG_RX_TOP_TXRX_CFG2_IMPLICIT_HEADER: u16 = 546;
pub const SX1302_REG_RX_TOP_TXRX_CFG2_CRC_EN: u16 = 547;
pub const SX1302_REG_RX_TOP_TXRX_CFG3_PAYLOAD_LENGTH: u16 = 548;
pub const SX1302_REG_RX_TOP_TXRX_CFG4_INT_STEP_ORIDE_EN: u16 = 549;
pub const SX1302_REG_RX_TOP_TXRX_CFG4_INT_STEP_ORIDE: u16 = 550;
pub const SX1302_REG_RX_TOP_TXRX_CFG5_HEADER_DIFF_MODE: u16 = 551;
pub const SX1302_REG_RX_TOP_TXRX_CFG5_ZERO_PAD: u16 = 552;
pub const SX1302_REG_RX_TOP_TXRX_CFG6_PREAMBLE_SYMB_NB: u16 = 553;
pub const SX1302_REG_RX_TOP_TXRX_CFG7_PREAMBLE_SYMB_NB: u16 = 554;
pub const SX1302_REG_RX_TOP_TXRX_CFG8_AUTO_ACK_INT_DELAY: u16 = 555;
pub const SX1302_REG_RX_TOP_TXRX_CFG8_AUTO_ACK_RX: u16 = 556;
pub const SX1302_REG_RX_TOP_TXRX_CFG8_AUTO_ACK_TX: u16 = 557;
pub const SX1302_REG_RX_TOP_TXRX_CFG8_POST_PREAMBLE_GAP_LONG: u16 = 558;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF12: u16 = 559;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF11: u16 = 560;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF10: u16 = 561;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF9: u16 = 562;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF8: u16 = 563;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF7: u16 = 564;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF6: u16 = 565;
pub const SX1302_REG_RX_TOP_TXRX_CFG9_FINE_SYNCH_EN_SF5: u16 = 566;
pub const SX1302_REG_RX_TOP_RX_CFG0_DFT_PEAK_EN: u16 = 567;
pub const SX1302_REG_RX_TOP_RX_CFG0_CHIRP_INVERT: u16 = 568;
pub const SX1302_REG_RX_TOP_RX_CFG0_SWAP_IQ: u16 = 569;
pub const SX1302_REG_RX_TOP_RX_CFG0_CONTINUOUS: u16 = 570;
pub const SX1302_REG_RX_TOP_RX_CFG1_DETECT_TIMEOUT: u16 = 571;
pub const SX1302_REG_RX_TOP_RX_CFG2_CLK_EN_RESYNC_DIN: u16 = 572;
pub const SX1302_REG_RX_TOP_RX_CFG2_LLR_SCALE: u16 = 573;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH0_SF5_PEAK1_POS_SF5: u16 = 574;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH1_SF5_PEAK2_POS_SF5: u16 = 575;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH0_SF6_PEAK1_POS_SF6: u16 = 576;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH1_SF6_PEAK2_POS_SF6: u16 = 577;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH0_SF7TO12_PEAK1_POS_SF7TO12: u16 = 578;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH1_SF7TO12_PEAK2_POS_SF7TO12: u16 = 579;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH2_FINETIME_ON_LAST: u16 = 580;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH2_AUTO_SCALE: u16 = 581;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH2_DROP_ON_SYNCH: u16 = 582;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH2_GAIN: u16 = 583;
pub const SX1302_REG_RX_TOP_FRAME_SYNCH2_TIMEOUT_OPT: u16 = 584;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_0_GAIN_P_HDR_RED: u16 = 585;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_0_ROUNDING: u16 = 586;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_0_POS_LIMIT: u16 = 587;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_0_SUM_SIZE: u16 = 588;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_0_MODE: u16 = 589;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_1_GAIN_P_AUTO: u16 = 590;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_1_GAIN_P_PAYLOAD: u16 = 591;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_1_GAIN_P_PREAMB: u16 = 592;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_2_GAIN_I_AUTO: u16 = 593;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_2_GAIN_I_PAYLOAD: u16 = 594;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_2_GAIN_I_PREAMB: u16 = 595;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_3_FINESYNCH_SUM: u16 = 596;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_3_FINESYNCH_GAIN: u16 = 597;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF8: u16 = 598;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF7: u16 = 599;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF6: u16 = 600;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_4_GAIN_I_EN_SF5: u16 = 601;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF12: u16 = 602;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF11: u16 = 603;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF10: u16 = 604;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_5_GAIN_I_EN_SF9: u16 = 605;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_6_GAIN_P_PREAMB_SF12: u16 = 606;
pub const SX1302_REG_RX_TOP_FINE_TIMING_A_6_GAIN_P_PREAMB_SF5_6: u16 = 607;
pub const SX1302_REG_RX_TOP_FINE_TIMING_7_GAIN_I_AUTO_MAX: u16 = 608;
pub const SX1302_REG_RX_TOP_FINE_TIMING_7_GAIN_P_AUTO_MAX: u16 = 609;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_0_GAIN_P_HDR_RED: u16 = 610;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_0_ROUNDING: u16 = 611;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_0_POS_LIMIT: u16 = 612;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_0_SUM_SIZE: u16 = 613;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_0_MODE: u16 = 614;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_1_GAIN_P_AUTO: u16 = 615;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_1_GAIN_P_PAYLOAD: u16 = 616;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_1_GAIN_P_PREAMB: u16 = 617;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_2_GAIN_I_AUTO: u16 = 618;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_2_GAIN_I_PAYLOAD: u16 = 619;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_2_GAIN_I_PREAMB: u16 = 620;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_3_FINESYNCH_SUM: u16 = 621;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_3_FINESYNCH_GAIN: u16 = 622;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF8: u16 = 623;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF7: u16 = 624;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF6: u16 = 625;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_4_GAIN_I_EN_SF5: u16 = 626;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF12: u16 = 627;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF11: u16 = 628;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF10: u16 = 629;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_5_GAIN_I_EN_SF9: u16 = 630;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_6_GAIN_P_PREAMB_SF12: u16 = 631;
pub const SX1302_REG_RX_TOP_FINE_TIMING_B_6_GAIN_P_PREAMB_SF5_6: u16 = 632;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME0_FREQ_TO_TIME_DRIFT_MANT: u16 = 633;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME1_FREQ_TO_TIME_DRIFT_MANT: u16 = 634;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME2_FREQ_TO_TIME_DRIFT_EXP: u16 = 635;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_DELTA: u16 = 636;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FINE_DELTA: u16 = 637;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_ERROR: u16 = 638;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_SYMB: u16 = 639;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_OFFSET: u16 = 640;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_DETECT: u16 = 641;
pub const SX1302_REG_RX_TOP_FREQ_TO_TIME4_FREQ_TO_TIME_INVERT_RNG: u16 = 642;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF8: u16 = 643;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF7: u16 = 644;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF6: u16 = 645;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_0_FREQ_TRACK_EN_SF5: u16 = 646;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF12: u16 = 647;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF11: u16 = 648;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF10: u16 = 649;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_A_1_FREQ_TRACK_EN_SF9: u16 = 650;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF8: u16 = 651;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF7: u16 = 652;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF6: u16 = 653;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_0_FREQ_TRACK_EN_SF5: u16 = 654;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF12: u16 = 655;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF11: u16 = 656;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF10: u16 = 657;
pub const SX1302_REG_RX_TOP_FREQ_TRACK_B_1_FREQ_TRACK_EN_SF9: u16 = 658;
pub const SX1302_REG_RX_TOP_FREQ_TRACK2_FREQ_TRACK_FINE: u16 = 659;
pub const SX1302_REG_RX_TOP_FREQ_TRACK2_FREQ_TRACK_HDR_SKIP: u16 = 660;
pub const SX1302_REG_RX_TOP_FREQ_TRACK3_FREQ_SYNCH_GAIN: u16 = 661;
pub const SX1302_REG_RX_TOP_FREQ_TRACK3_FREQ_TRACK_AUTO_THR: u16 = 662;
pub const SX1302_REG_RX_TOP_FREQ_TRACK4_SNR_MIN_WINDOW: u16 = 663;
pub const SX1302_REG_RX_TOP_FREQ_TRACK4_GAIN_AUTO_SNR_MIN: u16 = 664;
pub const SX1302_REG_RX_TOP_FREQ_TRACK4_FREQ_SYNCH_THR: u16 = 665;
pub const SX1302_REG_RX_TOP_DETECT_MSP0_MSP_PNR: u16 = 666;
pub const SX1302_REG_RX_TOP_DETECT_MSP1_MSP2_PNR: u16 = 667;
pub const SX1302_REG_RX_TOP_DETECT_MSP2_MSP2_PEAK_NB: u16 = 668;
pub const SX1302_REG_RX_TOP_DETECT_MSP2_MSP_PEAK_NB: u16 = 669;
pub const SX1302_REG_RX_TOP_DETECT_MSP3_ACC_MIN2: u16 = 670;
pub const SX1302_REG_RX_TOP_DETECT_MSP3_ACC_WIN_LEN: u16 = 671;
pub const SX1302_REG_RX_TOP_DETECT_MSP3_MSP_POS_SEL: u16 = 672;
pub const SX1302_REG_RX_TOP_DETECT_MSP3_MSP_CNT_MODE: u16 = 673;
pub const SX1302_REG_RX_TOP_DETECT_ACC1_USE_GAIN_SYMB: u16 = 674;
pub const SX1302_REG_RX_TOP_DETECT_ACC1_ACC_PNR: u16 = 675;
pub const SX1302_REG_RX_TOP_DETECT_ACC2_NOISE_COEFF: u16 = 676;
pub const SX1302_REG_RX_TOP_DETECT_ACC2_ACC_COEFF: u16 = 677;
pub const SX1302_REG_RX_TOP_DETECT_ACC2_ACC_2_SAME_PEAKS: u16 = 678;
pub const SX1302_REG_RX_TOP_DETECT_ACC2_ACC_AUTO_RESCALE: u16 = 679;
pub const SX1302_REG_RX_TOP_DETECT_ACC2_ACC_PEAK_POS_SEL: u16 = 680;
pub const SX1302_REG_RX_TOP_DETECT_ACC2_ACC_PEAK_SUM_EN: u16 = 681;
pub const SX1302_REG_RX_TOP_DETECT_ACC3_MIN_SINGLE_PEAK: u16 = 682;
pub const SX1302_REG_RX_TOP_TIMESTAMP_SEL_SNR_MIN: u16 = 683;
pub const SX1302_REG_RX_TOP_TIMESTAMP_ENABLE: u16 = 684;
pub const SX1302_REG_RX_TOP_TIMESTAMP_NB_SYMB: u16 = 685;
pub const SX1302_REG_RX_TOP_MODEM_BUSY_MSB_RX_MODEM_BUSY: u16 = 686;
pub const SX1302_REG_RX_TOP_MODEM_BUSY_LSB_RX_MODEM_BUSY: u16 = 687;
pub const SX1302_REG_RX_TOP_MODEM_STATE_RX_MODEM_STS_SPARE: u16 = 688;
pub const SX1302_REG_RX_TOP_MODEM_STATE_RX_MODEM_STATE: u16 = 689;
pub const SX1302_REG_RX_TOP_MODEM_SYNC_DELTA_MSB_PEAK_POS_FINE_GAIN_H: u16 = 690;
pub const SX1302_REG_RX_TOP_MODEM_SYNC_DELTA_MSB_PEAK_POS_FINE_GAIN_L: u16 = 691;
pub const SX1302_REG_RX_TOP_MODEM_SYNC_DELTA_MSB_PEAK_POS_FINE_SIGN: u16 = 692;
pub const SX1302_REG_RX_TOP_MODEM_SYNC_DELTA_MSB_MODEM_SYNC_DELTA: u16 = 693;
pub const SX1302_REG_RX_TOP_MODEM_SYNC_DELTA_LSB_MODEM_SYNC_DELTA: u16 = 694;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF8: u16 = 695;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF7: u16 = 696;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF6: u16 = 697;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET1_PPM_OFFSET_SF5: u16 = 698;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF12: u16 = 699;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF11: u16 = 700;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF10: u16 = 701;
pub const SX1302_REG_RX_TOP_MODEM_PPM_OFFSET2_PPM_OFFSET_SF9: u16 = 702;
pub const SX1302_REG_RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_3_CLK_OVERRIDE: u16 = 703;
pub const SX1302_REG_RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_2_CLK_OVERRIDE: u16 = 704;
pub const SX1302_REG_RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_1_CLK_OVERRIDE: u16 = 705;
pub const SX1302_REG_RX_TOP_MODEM_CLOCK_GATE_OVERRIDE_0_CLK_OVERRIDE: u16 = 706;
pub const SX1302_REG_RX_TOP_DUMMY2_DUMMY2: u16 = 707;
pub const SX1302_REG_RX_TOP_RX_BUFFER_DEBUG_MODE: u16 = 708;
pub const SX1302_REG_RX_TOP_RX_BUFFER_DIRECT_RAM_IF: u16 = 709;
pub const SX1302_REG_RX_TOP_RX_BUFFER_LEGACY_TIMESTAMP: u16 = 710;
pub const SX1302_REG_RX_TOP_RX_BUFFER_STORE_HEADER_ERR_META: u16 = 711;
pub const SX1302_REG_RX_TOP_RX_BUFFER_STORE_SYNC_FAIL_META: u16 = 712;
pub const SX1302_REG_RX_TOP_RX_BUFFER_TIMESTAMP_CFG_MAX_TS_METRICS: u16 = 713;
pub const SX1302_REG_RX_TOP_RX_BUFFER_IRQ_CTRL_MSB_RX_BUFFER_IRQ_THRESHOLD: u16 = 714;
pub const SX1302_REG_RX_TOP_RX_BUFFER_IRQ_CTRL_LSB_RX_BUFFER_IRQ_THRESHOLD: u16 = 715;
pub const SX1302_REG_RX_TOP_RX_BUFFER_LAST_ADDR_READ_MSB_LAST_ADDR_READ: u16 = 716;
pub const SX1302_REG_RX_TOP_RX_BUFFER_LAST_ADDR_READ_LSB_LAST_ADDR_READ: u16 = 717;
pub const SX1302_REG_RX_TOP_RX_BUFFER_LAST_ADDR_WRITE_MSB_LAST_ADDR_WRITE: u16 = 718;
pub const SX1302_REG_RX_TOP_RX_BUFFER_LAST_ADDR_WRITE_LSB_LAST_ADDR_WRITE: u16 = 719;
pub const SX1302_REG_RX_TOP_RX_BUFFER_NB_BYTES_MSB_RX_BUFFER_NB_BYTES: u16 = 720;
pub const SX1302_REG_RX_TOP_RX_BUFFER_NB_BYTES_LSB_RX_BUFFER_NB_BYTES: u16 = 721;
pub const SX1302_REG_RX_TOP_MULTI_SF_SYNC_ERR_PKT_CNT_MULTI_SF_SYNC_ERR_PKTS: u16 = 722;
pub const SX1302_REG_RX_TOP_MULTI_SF_PLD_ERR_PKT_CNT_MULTI_SF_PLD_ERR_PKTS: u16 = 723;
pub const SX1302_REG_RX_TOP_MULTI_SF_GOOD_PKT_CNT_MULTI_SF_GOOD_PKTS: u16 = 724;
pub const SX1302_REG_RX_TOP_SERV_MODEM_SYNC_ERR_PKT_CNT_SERV_MODEM_SYNC_ERR_PKTS: u16 = 725;
pub const SX1302_REG_RX_TOP_SERV_MODEM_PLD_ERR_PKT_CNT_SERV_MODEM_PLD_ERR_PKTS: u16 = 726;
pub const SX1302_REG_RX_TOP_SERV_MODEM_GOOD_PKT_CNT_SERV_MODEM_GOOD_PKTS: u16 = 727;
pub const SX1302_REG_RX_TOP_GFSK_MODEM_SYNC_ERR_PKT_CNT_GFSK_MODEM_SYNC_ERR_PKTS: u16 = 728;
pub const SX1302_REG_RX_TOP_GFSK_MODEM_PLD_ERR_PKT_CNT_GFSK_MODEM_PLD_ERR_PKTS: u16 = 729;
pub const SX1302_REG_RX_TOP_GFSK_MODEM_GOOD_PKT_CNT_GFSK_MODEM_GOOD_PKTS: u16 = 730;
pub const SX1302_REG_RX_TOP_BAD_MODEM_ID_WRITE_0_BAD_MODEM_ID_WRITE: u16 = 731;
pub const SX1302_REG_RX_TOP_BAD_MODEM_ID_WRITE_1_BAD_MODEM_ID_WRITE: u16 = 732;
pub const SX1302_REG_RX_TOP_BAD_MODEM_ID_WRITE_2_BAD_MODEM_ID_WRITE: u16 = 733;
pub const SX1302_REG_RX_TOP_BAD_MODEM_ID_READ_0_BAD_MODEM_ID_READ: u16 = 734;
pub const SX1302_REG_RX_TOP_BAD_MODEM_ID_READ_1_BAD_MODEM_ID_READ: u16 = 735;
pub const SX1302_REG_RX_TOP_BAD_MODEM_ID_READ_2_BAD_MODEM_ID_READ: u16 = 736;
pub const SX1302_REG_RX_TOP_CLOCK_GATE_OVERRIDE_0_CLK_OVERRIDE: u16 = 737;
pub const SX1302_REG_RX_TOP_SAMPLE_4_MSPS_LATCHED_125K_SAMPLE_4_MSPS_LATCHED_125K: u16 = 738;
pub const SX1302_REG_RX_TOP_DUMMY3_DUMMY3: u16 = 739;
pub const SX1302_REG_ARB_MCU_CTRL_CLK_EN: u16 = 740;
pub const SX1302_REG_ARB_MCU_CTRL_RADIO_RST: u16 = 741;
pub const SX1302_REG_ARB_MCU_CTRL_FORCE_HOST_FE_CTRL: u16 = 742;
pub const SX1302_REG_ARB_MCU_CTRL_MCU_CLEAR: u16 = 743;
pub const SX1302_REG_ARB_MCU_CTRL_HOST_PROG: u16 = 744;
pub const SX1302_REG_ARB_MCU_CTRL_PARITY_ERROR: u16 = 745;
pub const SX1302_REG_ARB_MCU_MCU_ARB_STATUS_MCU_ARB_STATUS: u16 = 746;
pub const SX1302_REG_ARB_MCU_UART_CFG_MSBF: u16 = 747;
pub const SX1302_REG_ARB_MCU_UART_CFG_PAR_EN: u16 = 748;
pub const SX1302_REG_ARB_MCU_UART_CFG_PAR_MODE: u16 = 749;
pub const SX1302_REG_ARB_MCU_UART_CFG_START_LEN: u16 = 750;
pub const SX1302_REG_ARB_MCU_UART_CFG_STOP_LEN: u16 = 751;
pub const SX1302_REG_ARB_MCU_UART_CFG_WORD_LEN: u16 = 752;
pub const SX1302_REG_ARB_MCU_UART_CFG2_BIT_RATE: u16 = 753;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_CFG_0_ARB_DEBUG_CFG_0: u16 = 754;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_CFG_1_ARB_DEBUG_CFG_1: u16 = 755;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_CFG_2_ARB_DEBUG_CFG_2: u16 = 756;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_CFG_3_ARB_DEBUG_CFG_3: u16 = 757;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_0_ARB_DEBUG_STS_0: u16 = 758;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_1_ARB_DEBUG_STS_1: u16 = 759;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_2_ARB_DEBUG_STS_2: u16 = 760;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_3_ARB_DEBUG_STS_3: u16 = 761;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_4_ARB_DEBUG_STS_4: u16 = 762;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_5_ARB_DEBUG_STS_5: u16 = 763;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_6_ARB_DEBUG_STS_6: u16 = 764;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_7_ARB_DEBUG_STS_7: u16 = 765;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_8_ARB_DEBUG_STS_8: u16 = 766;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_9_ARB_DEBUG_STS_9: u16 = 767;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_10_ARB_DEBUG_STS_10: u16 = 768;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_11_ARB_DEBUG_STS_11: u16 = 769;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_12_ARB_DEBUG_STS_12: u16 = 770;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_13_ARB_DEBUG_STS_13: u16 = 771;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_14_ARB_DEBUG_STS_14: u16 = 772;
pub const SX1302_REG_ARB_MCU_ARB_DEBUG_STS_15_ARB_DEBUG_STS_15: u16 = 773;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_01_CHANNEL_1_OFFSET: u16 = 774;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_01_CHANNEL_0_OFFSET: u16 = 775;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_23_CHANNEL_3_OFFSET: u16 = 776;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_23_CHANNEL_2_OFFSET: u16 = 777;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_45_CHANNEL_5_OFFSET: u16 = 778;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_45_CHANNEL_4_OFFSET: u16 = 779;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_67_CHANNEL_7_OFFSET: u16 = 780;
pub const SX1302_REG_ARB_MCU_CHANNEL_SYNC_OFFSET_67_CHANNEL_6_OFFSET: u16 = 781;
pub const SX1302_REG_ARB_MCU_DUMMY_DUMMY3: u16 = 782;
pub const SX1302_REG_RADIO_FE_GLBL_CTRL_DECIM_B_CLR: u16 = 783;
pub const SX1302_REG_RADIO_FE_GLBL_CTRL_DECIM_A_CLR: u16 = 784;
pub const SX1302_REG_RADIO_FE_CTRL0_RADIO_A_DC_NOTCH_EN: u16 = 785;
pub const SX1302_REG_RADIO_FE_CTRL0_RADIO_A_FORCE_HOST_FILTER_GAIN: u16 = 786;
pub const SX1302_REG_RADIO_FE_CTRL0_RADIO_A_HOST_FILTER_GAIN: u16 = 787;
pub const SX1302_REG_RADIO_FE_RSSI_DB_DEF_RADIO_A_RSSI_DB_DEFAULT_VALUE: u16 = 788;
pub const SX1302_REG_RADIO_FE_RSSI_DEC_DEF_RADIO_A_RSSI_DEC_DEFAULT_VALUE: u16 = 789;
pub const SX1302_REG_RADIO_FE_RSSI_DEC_RD_RADIO_A_RSSI_DEC_OUT: u16 = 790;
pub const SX1302_REG_RADIO_FE_RSSI_BB_RD_RADIO_A_RSSI_BB_OUT: u16 = 791;
pub const SX1302_REG_RADIO_FE_DEC_FILTER_RD_RADIO_A_DEC_FILTER_GAIN: u16 = 792;
pub const SX1302_REG_RADIO_FE_RSSI_BB_FILTER_ALPHA_RADIO_A_RSSI_BB_FILTER_ALPHA: u16 = 793;
pub const SX1302_REG_RADIO_FE_RSSI_DEC_FILTER_ALPHA_RADIO_A_RSSI_DEC_FILTER_ALPHA: u16 = 794;
pub const SX1302_REG_RADIO_FE_IQ_COMP_AMP_COEFF_RADIO_A_AMP_COEFF: u16 = 795;
pub const SX1302_REG_RADIO_FE_IQ_COMP_PHI_COEFF_RADIO_A_PHI_COEFF: u16 = 796;
pub const SX1302_REG_RADIO_FE_RADIO_DIO_TEST_MODE_RADIO_A_DIO_TEST_MODE: u16 = 797;
pub const SX1302_REG_RADIO_FE_RADIO_DIO_TEST_DIR_RADIO_A_DIO_TEST_DIR: u16 = 798;
pub const SX1302_REG_RADIO_FE_RADIO_DIO_DIR_RADIO_A_DIO_DIR: u16 = 799;
pub const SX1302_REG_RADIO_FE_CTRL0_RADIO_B_DC_NOTCH_EN: u16 = 800;
pub const SX1302_REG_RADIO_FE_CTRL0_RADIO_B_FORCE_HOST_FILTER_GAIN: u16 = 801;
pub const SX1302_REG_RADIO_FE_CTRL0_RADIO_B_HOST_FILTER_GAIN: u16 = 802;
pub const SX1302_REG_RADIO_FE_RSSI_DB_DEF_RADIO_B_RSSI_DB_DEFAULT_VALUE: u16 = 803;
pub const SX1302_REG_RADIO_FE_RSSI_DEC_DEF_RADIO_B_RSSI_DEC_DEFAULT_VALUE: u16 = 804;
pub const SX1302_REG_RADIO_FE_RSSI_DEC_RD_RADIO_B_RSSI_DEC_OUT: u16 = 805;
pub const SX1302_REG_RADIO_FE_RSSI_BB_RD_RADIO_B_RSSI_BB_OUT: u16 = 806;
pub const SX1302_REG_RADIO_FE_DEC_FILTER_RD_RADIO_B_DEC_FILTER_GAIN: u16 = 807;
pub const SX1302_REG_RADIO_FE_RSSI_BB_FILTER_ALPHA_RADIO_B_RSSI_BB_FILTER_ALPHA: u16 = 808;
pub const SX1302_REG_RADIO_FE_RSSI_DEC_FILTER_ALPHA_RADIO_B_RSSI_DEC_FILTER_ALPHA: u16 = 809;
pub const SX1302_REG_RADIO_FE_IQ_COMP_AMP_COEFF_RADIO_B_AMP_COEFF: u16 = 810;
pub const SX1302_REG_RADIO_FE_IQ_COMP_PHI_COEFF_RADIO_B_PHI_COEFF: u16 = 811;
pub const SX1302_REG_RADIO_FE_RADIO_DIO_TEST_MODE_RADIO_B_DIO_TEST_MODE: u16 = 812;
pub const SX1302_REG_RADIO_FE_RADIO_DIO_TEST_DIR_RADIO_B_DIO_TEST_DIR: u16 = 813;
pub const SX1302_REG_RADIO_FE_RADIO_DIO_DIR_RADIO_B_DIO_DIR: u16 = 814;
pub const SX1302_REG_RADIO_FE_SIG_ANA_CFG_VALID: u16 = 815;
pub const SX1302_REG_RADIO_FE_SIG_ANA_CFG_BUSY: u16 = 816;
pub const SX1302_REG_RADIO_FE_SIG_ANA_CFG_DURATION: u16 = 817;
pub const SX1302_REG_RADIO_FE_SIG_ANA_CFG_FORCE_HAL_CTRL: u16 = 818;
pub const SX1302_REG_RADIO_FE_SIG_ANA_CFG_START: u16 = 819;
pub const SX1302_REG_RADIO_FE_SIG_ANA_CFG_RADIO_SEL: u16 = 820;
pub const SX1302_REG_RADIO_FE_SIG_ANA_CFG_EN: u16 = 821;
pub const SX1302_REG_RADIO_FE_SIG_ANA_FREQ_FREQ: u16 = 822;
pub const SX1302_REG_RADIO_FE_SIG_ANA_ABS_MSB_CORR_ABS_OUT: u16 = 823;
pub const SX1302_REG_RADIO_FE_SIG_ANA_ABS_LSB_CORR_ABS_OUT: u16 = 824;
pub const SX1302_REG_RADIO_FE_DUMMY_DUMMY: u16 = 825;
pub const SX1302_REG_OTP_BYTE_ADDR_ADDR: u16 = 826;
pub const SX1302_REG_OTP_RD_DATA_RD_DATA: u16 = 827;
pub const SX1302_REG_OTP_STATUS_CHECKSUM_STATUS: u16 = 828;
pub const SX1302_REG_OTP_STATUS_FSM_READY: u16 = 829;
pub const SX1302_REG_OTP_CFG_ACCESS_MODE: u16 = 830;
pub const SX1302_REG_OTP_BIT_POS_POS: u16 = 831;
pub const SX1302_REG_OTP_PIN_CTRL_0_TM: u16 = 832;
pub const SX1302_REG_OTP_PIN_CTRL_0_STROBE: u16 = 833;
pub const SX1302_REG_OTP_PIN_CTRL_0_PGENB: u16 = 834;
pub const SX1302_REG_OTP_PIN_CTRL_0_LOAD: u16 = 835;
pub const SX1302_REG_OTP_PIN_CTRL_0_CSB: u16 = 836;
pub const SX1302_REG_OTP_PIN_CTRL_1_FSCK: u16 = 837;
pub const SX1302_REG_OTP_PIN_CTRL_1_FSI: u16 = 838;
pub const SX1302_REG_OTP_PIN_CTRL_1_FRST: u16 = 839;
pub const SX1302_REG_OTP_PIN_STATUS_FSO: u16 = 840;
pub const SX1302_REG_OTP_MODEM_EN_0_MODEM_EN: u16 = 841;
pub const SX1302_REG_OTP_MODEM_EN_1_MODEM_EN: u16 = 842;
pub const SX1302_REG_OTP_MODEM_SF_EN_SF_EN: u16 = 843;
pub const SX1302_REG_OTP_TIMESTAMP_EN_TIMESTAMP_EN: u16 = 844;
pub const SX1302_REG_OTP_DUMMY_DUMMY: u16 = 845;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_FREQ_MSB_IF_FREQ_0: u16 = 846;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_FREQ_LSB_IF_FREQ_0: u16 = 847;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_LORA_SERVICE_RADIO_SEL_RADIO_SELECT: u16 = 848;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_BW_START: u16 = 849;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_AUTO_BW_RED: u16 = 850;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_NO_FAST_START: u16 = 851;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_BYPASS: u16 = 852;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG1_ENABLE: u16 = 853;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG2_BW_LOCKED: u16 = 854;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG2_BW: u16 = 855;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG3_BW_RED: u16 = 856;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DC_NOTCH_CFG4_IIR_DCC_TIME: u16 = 857;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_0_FIR1_COEFF_0: u16 = 858;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_1_FIR1_COEFF_1: u16 = 859;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_2_FIR1_COEFF_2: u16 = 860;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_3_FIR1_COEFF_3: u16 = 861;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_4_FIR1_COEFF_4: u16 = 862;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_5_FIR1_COEFF_5: u16 = 863;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_6_FIR1_COEFF_6: u16 = 864;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR1_7_FIR1_COEFF_7: u16 = 865;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_0_FIR2_COEFF_0: u16 = 866;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_1_FIR2_COEFF_1: u16 = 867;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_2_FIR2_COEFF_2: u16 = 868;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_3_FIR2_COEFF_3: u16 = 869;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_4_FIR2_COEFF_4: u16 = 870;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_5_FIR2_COEFF_5: u16 = 871;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_6_FIR2_COEFF_6: u16 = 872;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_FIR2_7_FIR2_COEFF_7: u16 = 873;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC0_RADIO_GAIN_RED_SEL: u16 = 874;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC0_RADIO_GAIN_RED_DB: u16 = 875;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_DC_COMP_EN: u16 = 876;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_FORCE_DEFAULT_FIR: u16 = 877;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_RSSI_EARLY_LATCH: u16 = 878;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC1_FREEZE_ON_SYNC: u16 = 879;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_DAGC_IN_COMP: u16 = 880;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_DAGC_FIR_HYST: u16 = 881;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_RSSI_MAX_SAMPLE: u16 = 882;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_AGC2_RSSI_MIN_SAMPLE: u16 = 883;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_DAGC_FIR_FAST: u16 = 884;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_FORCE_GAIN_FIR: u16 = 885;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_GAIN_FIR1: u16 = 886;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_DFE_GAIN0_GAIN_FIR2: u16 = 887;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_TARGET_LVL: u16 = 888;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_GAIN_INCR_STEP: u16 = 889;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_GAIN_DROP_COMP: u16 = 890;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_COMB_FILTER_EN: u16 = 891;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_NO_FREEZE_START: u16 = 892;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CFG_FREEZE_ON_SYNC: u16 = 893;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CNT0_SAMPLE: u16 = 894;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CNT1_THR_M6: u16 = 895;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CNT2_THR_M12: u16 = 896;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CNT3_THR_M18: u16 = 897;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CNT4_GAIN: u16 = 898;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DAGC_CNT4_FORCE_GAIN: u16 = 899;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG0_MODEM_BW: u16 = 900;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG0_MODEM_SF: u16 = 901;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_PPM_OFFSET_HDR_CTRL: u16 = 902;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_PPM_OFFSET: u16 = 903;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_MODEM_EN: u16 = 904;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG1_CODING_RATE: u16 = 905;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_FINE_SYNCH_EN: u16 = 906;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_MODEM_START: u16 = 907;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_CADRXTX: u16 = 908;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_IMPLICIT_HEADER: u16 = 909;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG2_CRC_EN: u16 = 910;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG3_PAYLOAD_LENGTH: u16 = 911;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG4_INT_STEP_ORIDE_EN: u16 = 912;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG4_INT_STEP_ORIDE: u16 = 913;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG5_HEADER_DIFF_MODE: u16 = 914;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG5_ZERO_PAD: u16 = 915;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG6_PREAMBLE_SYMB_NB: u16 = 916;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG7_PREAMBLE_SYMB_NB: u16 = 917;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_AUTO_ACK_INT_DELAY: u16 = 918;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_AUTO_ACK_RX: u16 = 919;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_AUTO_ACK_TX: u16 = 920;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TXRX_CFG8_POST_PREAMBLE_GAP_LONG: u16 = 921;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG0_DFT_PEAK_EN: u16 = 922;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG0_CHIRP_INVERT: u16 = 923;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG0_SWAP_IQ: u16 = 924;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG0_CONTINUOUS: u16 = 925;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG1_DETECT_TIMEOUT: u16 = 926;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG2_AUTO_ACK_RANGE: u16 = 927;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG2_AUTO_ACK_DELAY: u16 = 928;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG3_RESTART_ON_HDR_ERR: u16 = 929;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG3_CLK_EN_RESYNC_DIN: u16 = 930;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_RX_CFG3_LLR_SCALE: u16 = 931;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH0_PEAK1_POS: u16 = 932;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH1_PEAK2_POS: u16 = 933;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_FINETIME_ON_LAST: u16 = 934;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_AUTO_SCALE: u16 = 935;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_DROP_ON_SYNCH: u16 = 936;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_GAIN: u16 = 937;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FRAME_SYNCH2_TIMEOUT_OPT: u16 = 938;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_GAIN_P_HDR_RED: u16 = 939;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_ROUNDING: u16 = 940;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_POS_LIMIT: u16 = 941;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_SUM_SIZE: u16 = 942;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING0_MODE: u16 = 943;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_AUTO: u16 = 944;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_PAYLOAD: u16 = 945;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING1_GAIN_P_PREAMB: u16 = 946;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_EN: u16 = 947;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_PAYLOAD: u16 = 948;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING2_GAIN_I_PREAMB: u16 = 949;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING3_FINESYNCH_SUM: u16 = 950;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING3_FINESYNCH_GAIN: u16 = 951;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING3_GAIN_I_AUTO: u16 = 952;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING4_GAIN_I_AUTO_MAX: u16 = 953;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FINE_TIMING4_GAIN_P_AUTO_MAX: u16 = 954;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME0_FREQ_TO_TIME_DRIFT_MANT: u16 = 955;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME1_FREQ_TO_TIME_DRIFT_MANT: u16 = 956;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME2_FREQ_TO_TIME_DRIFT_EXP: u16 = 957;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_DELTA: u16 = 958;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FINE_DELTA: u16 = 959;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_FREQ_ERROR: u16 = 960;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_SYMB: u16 = 961;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_TIME_OFFSET: u16 = 962;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME3_FREQ_TO_TIME_INVERT_DETECT: u16 = 963;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TO_TIME4_FREQ_TO_TIME_INVERT_RNG: u16 = 964;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK0_FREQ_TRACK_FINE: u16 = 965;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK0_FREQ_TRACK_HDR_SKIP: u16 = 966;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK0_FREQ_TRACK_EN: u16 = 967;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK1_FREQ_SYNCH_GAIN: u16 = 968;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK1_FREQ_TRACK_AUTO_THR: u16 = 969;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK2_SNR_MIN_WINDOW: u16 = 970;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK2_GAIN_AUTO_SNR_MIN: u16 = 971;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FREQ_TRACK2_FREQ_SYNCH_THR: u16 = 972;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP0_MSP_PNR: u16 = 973;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP1_MSP2_PNR: u16 = 974;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP2_MSP2_PEAK_NB: u16 = 975;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP2_MSP_PEAK_NB: u16 = 976;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_ACC_MIN2: u16 = 977;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_ACC_WIN_LEN: u16 = 978;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_MSP_POS_SEL: u16 = 979;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_MSP3_MSP_CNT_MODE: u16 = 980;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC1_USE_GAIN_SYMB: u16 = 981;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC1_ACC_PNR: u16 = 982;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_NOISE_COEFF: u16 = 983;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_COEFF: u16 = 984;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_2_SAME_PEAKS: u16 = 985;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_AUTO_RESCALE: u16 = 986;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_PEAK_POS_SEL: u16 = 987;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC2_ACC_PEAK_SUM_EN: u16 = 988;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DETECT_ACC3_MIN_SINGLE_PEAK: u16 = 989;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TIMESTAMP_SEL_SNR_MIN: u16 = 990;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TIMESTAMP_ENABLE: u16 = 991;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_TIMESTAMP_NB_SYMB: u16 = 992;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_FSK_TRANSPOSE_CLK_OVERRIDE: u16 = 993;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_FSK_MODEM_CLK_OVERRIDE: u16 = 994;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_TRANSPOSE_CLK_OVERRIDE: u16 = 995;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_CLOCK_GATE_OVERRIDE_MODEM_CLK_OVERRIDE: u16 = 996;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DUMMY0_DUMMY0: u16 = 997;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_FREQ_MSB_IF_FREQ_0: u16 = 998;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_FREQ_LSB_IF_FREQ_0: u16 = 999;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_CRC_IBM: u16 = 1000;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_DCFREE_ENC: u16 = 1001;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_CRC_EN: u16 = 1002;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_0_PKT_MODE: u16 = 1003;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_ADRS_COMP: u16 = 1004;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_PSIZE: u16 = 1005;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_1_CH_BW_EXPO: u16 = 1006;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_MODEM_INVERT_IQ: u16 = 1007;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_AUTO_AFC: u16 = 1008;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_RADIO_SELECT: u16 = 1009;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_3_RX_INVERT: u16 = 1010;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_4_RSSI_LENGTH: u16 = 1011;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_CFG_4_ERROR_OSR_TOL: u16 = 1012;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_NODE_ADRS_NODE_ADRS: u16 = 1013;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_BROADCAST_BROADCAST: u16 = 1014;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_PKT_LENGTH_PKT_LENGTH: u16 = 1015;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_TIMEOUT_MSB_TIMEOUT: u16 = 1016;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_TIMEOUT_LSB_TIMEOUT: u16 = 1017;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_BIT_RATE_MSB_BIT_RATE: u16 = 1018;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_BIT_RATE_LSB_BIT_RATE: u16 = 1019;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN: u16 = 1020;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN: u16 = 1021;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN: u16 = 1022;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN: u16 = 1023;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN: u16 = 1024;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN: u16 = 1025;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN: u16 = 1026;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN: u16 = 1027;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_FSK_RSSI_FILTER_ALPHA_FSK_RSSI_FILTER_ALPHA: u16 = 1028;
pub const SX1302_REG_RX_TOP_LORA_SERVICE_FSK_DUMMY1_DUMMY1: u16 = 1029;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_CFG_ENABLE: u16 = 1030;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_CFG_CAPTUREWRAP: u16 = 1031;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_CFG_CAPTUREFORCETRIGGER: u16 = 1032;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_CFG_CAPTURESTART: u16 = 1033;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_CFG_RAMCONFIG: u16 = 1034;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_SOURCE_A_SOURCEMUX: u16 = 1035;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_SOURCE_B_SOURCEMUX: u16 = 1036;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_PERIOD_0_CAPTUREPERIOD: u16 = 1037;
pub const SX1302_REG_CAPTURE_RAM_CAPTURE_PERIOD_1_CAPTUREPERIOD: u16 = 1038;
pub const SX1302_REG_CAPTURE_RAM_STATUS_CAPCOMPLETE: u16 = 1039;
pub const SX1302_REG_CAPTURE_RAM_LAST_RAM_ADDR_0_LASTRAMADDR: u16 = 1040;
pub const SX1302_REG_CAPTURE_RAM_LAST_RAM_ADDR_1_LASTRAMADDR: u16 = 1041;
pub const SX1302_REG_CAPTURE_RAM_CLOCK_GATE_OVERRIDE_CLK_OVERRIDE: u16 = 1042;
pub const SX1302_REG_CAPTURE_RAM_DUMMY0_DUMMY0: u16 = 1043;

// ---------------------------------------------------------------------------
// RF-chain specific register selectors (C: `#define NAME(rf_chain) ...`)
//
// `rf_chain` is 0 for radio A and 1 for radio B.
// ---------------------------------------------------------------------------

/// C: `SX1302_REG_TX_TOP_TX_TRIG_TX_FSM_CLR(rf_chain)`
pub const fn tx_top_tx_trig_tx_fsm_clr(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 58 } else { 166 }
}

/// C: `SX1302_REG_TX_TOP_TX_TRIG_TX_TRIG_GPS(rf_chain)`
pub const fn tx_top_tx_trig_tx_trig_gps(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 59 } else { 167 }
}

/// C: `SX1302_REG_TX_TOP_TX_TRIG_TX_TRIG_DELAYED(rf_chain)`
pub const fn tx_top_tx_trig_tx_trig_delayed(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 60 } else { 168 }
}

/// C: `SX1302_REG_TX_TOP_TX_TRIG_TX_TRIG_IMMEDIATE(rf_chain)`
pub const fn tx_top_tx_trig_tx_trig_immediate(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 61 } else { 169 }
}

/// C: `SX1302_REG_TX_TOP_TIMER_TRIG_BYTE3_TIMER_DELAYED_TRIG(rf_chain)`
pub const fn tx_top_timer_trig_byte3_timer_delayed_trig(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 62 } else { 170 }
}

/// C: `SX1302_REG_TX_TOP_TIMER_TRIG_BYTE2_TIMER_DELAYED_TRIG(rf_chain)`
pub const fn tx_top_timer_trig_byte2_timer_delayed_trig(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 63 } else { 171 }
}

/// C: `SX1302_REG_TX_TOP_TIMER_TRIG_BYTE1_TIMER_DELAYED_TRIG(rf_chain)`
pub const fn tx_top_timer_trig_byte1_timer_delayed_trig(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 64 } else { 172 }
}

/// C: `SX1302_REG_TX_TOP_TIMER_TRIG_BYTE0_TIMER_DELAYED_TRIG(rf_chain)`
pub const fn tx_top_timer_trig_byte0_timer_delayed_trig(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 65 } else { 173 }
}

/// C: `SX1302_REG_TX_TOP_TX_START_DELAY_MSB_TX_START_DELAY(rf_chain)`
pub const fn tx_top_tx_start_delay_msb_tx_start_delay(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 66 } else { 174 }
}

/// C: `SX1302_REG_TX_TOP_TX_START_DELAY_LSB_TX_START_DELAY(rf_chain)`
pub const fn tx_top_tx_start_delay_lsb_tx_start_delay(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 67 } else { 175 }
}

/// C: `SX1302_REG_TX_TOP_TX_CTRL_WRITE_BUFFER(rf_chain)`
pub const fn tx_top_tx_ctrl_write_buffer(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 68 } else { 176 }
}

/// C: `SX1302_REG_TX_TOP_TX_RAMP_DURATION_TX_RAMP_DURATION(rf_chain)`
pub const fn tx_top_tx_ramp_duration_tx_ramp_duration(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 69 } else { 177 }
}

/// C: `SX1302_REG_TX_TOP_GEN_CFG_0_MODULATION_TYPE(rf_chain)`
pub const fn tx_top_gen_cfg_0_modulation_type(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 70 } else { 178 }
}

/// C: `SX1302_REG_TX_TOP_TEST_0_TX_ACTIVE_CTRL(rf_chain)`
pub const fn tx_top_test_0_tx_active_ctrl(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 71 } else { 179 }
}

/// C: `SX1302_REG_TX_TOP_TEST_0_TX_ACTIVE_SEL(rf_chain)`
pub const fn tx_top_test_0_tx_active_sel(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 72 } else { 180 }
}

/// C: `SX1302_REG_TX_TOP_TX_FLAG_TX_TIMEOUT(rf_chain)`
pub const fn tx_top_tx_flag_tx_timeout(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 73 } else { 181 }
}

/// C: `SX1302_REG_TX_TOP_TX_FLAG_PKT_DONE(rf_chain)`
pub const fn tx_top_tx_flag_pkt_done(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 74 } else { 182 }
}

/// C: `SX1302_REG_TX_TOP_AGC_TX_BW_AGC_TX_BW(rf_chain)`
pub const fn tx_top_agc_tx_bw_agc_tx_bw(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 75 } else { 183 }
}

/// C: `SX1302_REG_TX_TOP_AGC_TX_PWR_AGC_TX_PWR(rf_chain)`
pub const fn tx_top_agc_tx_pwr_agc_tx_pwr(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 76 } else { 184 }
}

/// C: `SX1302_REG_TX_TOP_TIMEOUT_CNT_BYTE_2_TIMEOUT_CNT(rf_chain)`
pub const fn tx_top_timeout_cnt_byte_2_timeout_cnt(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 77 } else { 185 }
}

/// C: `SX1302_REG_TX_TOP_TIMEOUT_CNT_BYTE_1_TIMEOUT_CNT(rf_chain)`
pub const fn tx_top_timeout_cnt_byte_1_timeout_cnt(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 78 } else { 186 }
}

/// C: `SX1302_REG_TX_TOP_TIMEOUT_CNT_BYTE_0_TIMEOUT_CNT(rf_chain)`
pub const fn tx_top_timeout_cnt_byte_0_timeout_cnt(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 79 } else { 187 }
}

/// C: `SX1302_REG_TX_TOP_TX_FSM_STATUS_TX_STATUS(rf_chain)`
pub const fn tx_top_tx_fsm_status_tx_status(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 80 } else { 188 }
}

/// C: `SX1302_REG_TX_TOP_DUMMY_CONTROL_DUMMY(rf_chain)`
pub const fn tx_top_dummy_control_dummy(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 81 } else { 189 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_CTRL_PLL_DIV_CTRL(rf_chain)`
pub const fn tx_top_tx_rffe_if_ctrl_pll_div_ctrl(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 82 } else { 190 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_CTRL_TX_CLK_EDGE(rf_chain)`
pub const fn tx_top_tx_rffe_if_ctrl_tx_clk_edge(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 83 } else { 191 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_CTRL_TX_MODE(rf_chain)`
pub const fn tx_top_tx_rffe_if_ctrl_tx_mode(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 84 } else { 192 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_CTRL_TX_IF_DST(rf_chain)`
pub const fn tx_top_tx_rffe_if_ctrl_tx_if_dst(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 85 } else { 193 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_CTRL_TX_IF_SRC(rf_chain)`
pub const fn tx_top_tx_rffe_if_ctrl_tx_if_src(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 86 } else { 194 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_CTRL2_SX125X_IQ_INVERT(rf_chain)`
pub const fn tx_top_tx_rffe_if_ctrl2_sx125x_iq_invert(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 87 } else { 195 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_CTRL2_PLL_DIV_CTRL_AGC(rf_chain)`
pub const fn tx_top_tx_rffe_if_ctrl2_pll_div_ctrl_agc(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 88 } else { 196 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_IQ_GAIN_IQ_GAIN(rf_chain)`
pub const fn tx_top_tx_rffe_if_iq_gain_iq_gain(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 89 } else { 197 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_I_OFFSET_I_OFFSET(rf_chain)`
pub const fn tx_top_tx_rffe_if_i_offset_i_offset(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 90 } else { 198 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_Q_OFFSET_Q_OFFSET(rf_chain)`
pub const fn tx_top_tx_rffe_if_q_offset_q_offset(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 91 } else { 199 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_FREQ_RF_H_FREQ_RF(rf_chain)`
pub const fn tx_top_tx_rffe_if_freq_rf_h_freq_rf(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 92 } else { 200 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_FREQ_RF_M_FREQ_RF(rf_chain)`
pub const fn tx_top_tx_rffe_if_freq_rf_m_freq_rf(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 93 } else { 201 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_FREQ_RF_L_FREQ_RF(rf_chain)`
pub const fn tx_top_tx_rffe_if_freq_rf_l_freq_rf(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 94 } else { 202 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_FREQ_DEV_H_FREQ_DEV(rf_chain)`
pub const fn tx_top_tx_rffe_if_freq_dev_h_freq_dev(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 95 } else { 203 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_FREQ_DEV_L_FREQ_DEV(rf_chain)`
pub const fn tx_top_tx_rffe_if_freq_dev_l_freq_dev(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 96 } else { 204 }
}

/// C: `SX1302_REG_TX_TOP_TX_RFFE_IF_TEST_MOD_FREQ(rf_chain)`
pub const fn tx_top_tx_rffe_if_test_mod_freq(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 97 } else { 205 }
}

/// C: `SX1302_REG_TX_TOP_DUMMY_MODULATOR_DUMMY(rf_chain)`
pub const fn tx_top_dummy_modulator_dummy(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 98 } else { 206 }
}

/// C: `SX1302_REG_TX_TOP_FSK_PKT_LEN_PKT_LENGTH(rf_chain)`
pub const fn tx_top_fsk_pkt_len_pkt_length(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 99 } else { 207 }
}

/// C: `SX1302_REG_TX_TOP_FSK_CFG_0_TX_CONT(rf_chain)`
pub const fn tx_top_fsk_cfg_0_tx_cont(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 100 } else { 208 }
}

/// C: `SX1302_REG_TX_TOP_FSK_CFG_0_CRC_IBM(rf_chain)`
pub const fn tx_top_fsk_cfg_0_crc_ibm(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 101 } else { 209 }
}

/// C: `SX1302_REG_TX_TOP_FSK_CFG_0_DCFREE_ENC(rf_chain)`
pub const fn tx_top_fsk_cfg_0_dcfree_enc(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 102 } else { 210 }
}

/// C: `SX1302_REG_TX_TOP_FSK_CFG_0_CRC_EN(rf_chain)`
pub const fn tx_top_fsk_cfg_0_crc_en(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 103 } else { 211 }
}

/// C: `SX1302_REG_TX_TOP_FSK_CFG_0_PKT_MODE(rf_chain)`
pub const fn tx_top_fsk_cfg_0_pkt_mode(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 104 } else { 212 }
}

/// C: `SX1302_REG_TX_TOP_FSK_PREAMBLE_SIZE_MSB_PREAMBLE_SIZE(rf_chain)`
pub const fn tx_top_fsk_preamble_size_msb_preamble_size(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 105 } else { 213 }
}

/// C: `SX1302_REG_TX_TOP_FSK_PREAMBLE_SIZE_LSB_PREAMBLE_SIZE(rf_chain)`
pub const fn tx_top_fsk_preamble_size_lsb_preamble_size(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 106 } else { 214 }
}

/// C: `SX1302_REG_TX_TOP_FSK_BIT_RATE_MSB_BIT_RATE(rf_chain)`
pub const fn tx_top_fsk_bit_rate_msb_bit_rate(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 107 } else { 215 }
}

/// C: `SX1302_REG_TX_TOP_FSK_BIT_RATE_LSB_BIT_RATE(rf_chain)`
pub const fn tx_top_fsk_bit_rate_lsb_bit_rate(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 108 } else { 216 }
}

/// C: `SX1302_REG_TX_TOP_FSK_MOD_FSK_REF_PATTERN_SIZE(rf_chain)`
pub const fn tx_top_fsk_mod_fsk_ref_pattern_size(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 109 } else { 217 }
}

/// C: `SX1302_REG_TX_TOP_FSK_MOD_FSK_PREAMBLE_SEQ(rf_chain)`
pub const fn tx_top_fsk_mod_fsk_preamble_seq(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 110 } else { 218 }
}

/// C: `SX1302_REG_TX_TOP_FSK_MOD_FSK_REF_PATTERN_EN(rf_chain)`
pub const fn tx_top_fsk_mod_fsk_ref_pattern_en(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 111 } else { 219 }
}

/// C: `SX1302_REG_TX_TOP_FSK_MOD_FSK_GAUSSIAN_SELECT_BT(rf_chain)`
pub const fn tx_top_fsk_mod_fsk_gaussian_select_bt(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 112 } else { 220 }
}

/// C: `SX1302_REG_TX_TOP_FSK_MOD_FSK_GAUSSIAN_EN(rf_chain)`
pub const fn tx_top_fsk_mod_fsk_gaussian_en(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 113 } else { 221 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE7_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte7_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 114 } else { 222 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE6_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte6_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 115 } else { 223 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE5_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte5_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 116 } else { 224 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE4_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte4_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 117 } else { 225 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE3_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte3_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 118 } else { 226 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE2_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte2_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 119 } else { 227 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE1_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte1_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 120 } else { 228 }
}

/// C: `SX1302_REG_TX_TOP_FSK_REF_PATTERN_BYTE0_FSK_REF_PATTERN(rf_chain)`
pub const fn tx_top_fsk_ref_pattern_byte0_fsk_ref_pattern(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 121 } else { 229 }
}

/// C: `SX1302_REG_TX_TOP_DUMMY_GSFK_DUMMY(rf_chain)`
pub const fn tx_top_dummy_gsfk_dummy(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 122 } else { 230 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_0_MODEM_BW(rf_chain)`
pub const fn tx_top_txrx_cfg0_0_modem_bw(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 123 } else { 231 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_0_MODEM_SF(rf_chain)`
pub const fn tx_top_txrx_cfg0_0_modem_sf(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 124 } else { 232 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_1_PPM_OFFSET_HDR_CTRL(rf_chain)`
pub const fn tx_top_txrx_cfg0_1_ppm_offset_hdr_ctrl(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 125 } else { 233 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_1_PPM_OFFSET(rf_chain)`
pub const fn tx_top_txrx_cfg0_1_ppm_offset(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 126 } else { 234 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_1_POST_PREAMBLE_GAP_LONG(rf_chain)`
pub const fn tx_top_txrx_cfg0_1_post_preamble_gap_long(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 127 } else { 235 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_1_CODING_RATE(rf_chain)`
pub const fn tx_top_txrx_cfg0_1_coding_rate(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 128 } else { 236 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_2_FINE_SYNCH_EN(rf_chain)`
pub const fn tx_top_txrx_cfg0_2_fine_synch_en(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 129 } else { 237 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_2_MODEM_EN(rf_chain)`
pub const fn tx_top_txrx_cfg0_2_modem_en(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 130 } else { 238 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_2_CADRXTX(rf_chain)`
pub const fn tx_top_txrx_cfg0_2_cadrxtx(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 131 } else { 239 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_2_IMPLICIT_HEADER(rf_chain)`
pub const fn tx_top_txrx_cfg0_2_implicit_header(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 132 } else { 240 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_2_CRC_EN(rf_chain)`
pub const fn tx_top_txrx_cfg0_2_crc_en(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 133 } else { 241 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG0_3_PAYLOAD_LENGTH(rf_chain)`
pub const fn tx_top_txrx_cfg0_3_payload_length(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 134 } else { 242 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_0_INT_STEP_ORIDE_EN(rf_chain)`
pub const fn tx_top_txrx_cfg1_0_int_step_oride_en(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 135 } else { 243 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_0_INT_STEP_ORIDE(rf_chain)`
pub const fn tx_top_txrx_cfg1_0_int_step_oride(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 136 } else { 244 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_1_MODEM_START(rf_chain)`
pub const fn tx_top_txrx_cfg1_1_modem_start(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 137 } else { 245 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_1_HEADER_DIFF_MODE(rf_chain)`
pub const fn tx_top_txrx_cfg1_1_header_diff_mode(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 138 } else { 246 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_1_ZERO_PAD(rf_chain)`
pub const fn tx_top_txrx_cfg1_1_zero_pad(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 139 } else { 247 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_2_PREAMBLE_SYMB_NB(rf_chain)`
pub const fn tx_top_txrx_cfg1_2_preamble_symb_nb(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 140 } else { 248 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_3_PREAMBLE_SYMB_NB(rf_chain)`
pub const fn tx_top_txrx_cfg1_3_preamble_symb_nb(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 141 } else { 249 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_4_AUTO_ACK_INT_DELAY(rf_chain)`
pub const fn tx_top_txrx_cfg1_4_auto_ack_int_delay(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 142 } else { 250 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_4_AUTO_ACK_RX(rf_chain)`
pub const fn tx_top_txrx_cfg1_4_auto_ack_rx(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 143 } else { 251 }
}

/// C: `SX1302_REG_TX_TOP_TXRX_CFG1_4_AUTO_ACK_TX(rf_chain)`
pub const fn tx_top_txrx_cfg1_4_auto_ack_tx(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 144 } else { 252 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG0_0_CHIRP_LOWPASS(rf_chain)`
pub const fn tx_top_tx_cfg0_0_chirp_lowpass(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 145 } else { 253 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG0_0_PPM_OFFSET_SIG(rf_chain)`
pub const fn tx_top_tx_cfg0_0_ppm_offset_sig(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 146 } else { 254 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG0_0_CONTCHIRP(rf_chain)`
pub const fn tx_top_tx_cfg0_0_contchirp(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 147 } else { 255 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG0_0_CHIRP_INVERT(rf_chain)`
pub const fn tx_top_tx_cfg0_0_chirp_invert(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 148 } else { 256 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG0_0_CONTINUOUS(rf_chain)`
pub const fn tx_top_tx_cfg0_0_continuous(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 149 } else { 257 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG0_1_POWER_RANGING(rf_chain)`
pub const fn tx_top_tx_cfg0_1_power_ranging(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 150 } else { 258 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG1_0_FRAME_NB(rf_chain)`
pub const fn tx_top_tx_cfg1_0_frame_nb(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 151 } else { 259 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG1_1_HOP_CTRL(rf_chain)`
pub const fn tx_top_tx_cfg1_1_hop_ctrl(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 152 } else { 260 }
}

/// C: `SX1302_REG_TX_TOP_TX_CFG1_1_IFS(rf_chain)`
pub const fn tx_top_tx_cfg1_1_ifs(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 153 } else { 261 }
}

/// C: `SX1302_REG_TX_TOP_FRAME_SYNCH_0_AUTO_SCALE(rf_chain)`
pub const fn tx_top_frame_synch_0_auto_scale(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 154 } else { 262 }
}

/// C: `SX1302_REG_TX_TOP_FRAME_SYNCH_0_DROP_ON_SYNCH(rf_chain)`
pub const fn tx_top_frame_synch_0_drop_on_synch(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 155 } else { 263 }
}

/// C: `SX1302_REG_TX_TOP_FRAME_SYNCH_0_GAIN(rf_chain)`
pub const fn tx_top_frame_synch_0_gain(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 156 } else { 264 }
}

/// C: `SX1302_REG_TX_TOP_FRAME_SYNCH_0_PEAK1_POS(rf_chain)`
pub const fn tx_top_frame_synch_0_peak1_pos(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 157 } else { 265 }
}

/// C: `SX1302_REG_TX_TOP_FRAME_SYNCH_1_FINETIME_ON_LAST(rf_chain)`
pub const fn tx_top_frame_synch_1_finetime_on_last(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 158 } else { 266 }
}

/// C: `SX1302_REG_TX_TOP_FRAME_SYNCH_1_TIMEOUT_OPT(rf_chain)`
pub const fn tx_top_frame_synch_1_timeout_opt(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 159 } else { 267 }
}

/// C: `SX1302_REG_TX_TOP_FRAME_SYNCH_1_PEAK2_POS(rf_chain)`
pub const fn tx_top_frame_synch_1_peak2_pos(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 160 } else { 268 }
}

/// C: `SX1302_REG_TX_TOP_LORA_TX_STATE_STATUS(rf_chain)`
pub const fn tx_top_lora_tx_state_status(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 161 } else { 269 }
}

/// C: `SX1302_REG_TX_TOP_LORA_TX_FLAG_FRAME_DONE(rf_chain)`
pub const fn tx_top_lora_tx_flag_frame_done(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 162 } else { 270 }
}

/// C: `SX1302_REG_TX_TOP_LORA_TX_FLAG_CONT_DONE(rf_chain)`
pub const fn tx_top_lora_tx_flag_cont_done(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 163 } else { 271 }
}

/// C: `SX1302_REG_TX_TOP_LORA_TX_FLAG_PLD_DONE(rf_chain)`
pub const fn tx_top_lora_tx_flag_pld_done(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 164 } else { 272 }
}

/// C: `SX1302_REG_TX_TOP_DUMMY_LORA_DUMMY(rf_chain)`
pub const fn tx_top_dummy_lora_dummy(rf_chain: u8) -> u16 {
    if rf_chain == 0 { 165 } else { 273 }
}
