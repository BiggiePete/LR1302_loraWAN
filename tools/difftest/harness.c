/*
 * Conformance harness for the C HAL.
 *
 * Compiles the *unmodified* Semtech SX1302 HAL sources (loragw_hal.c is
 * #included so that the file-static helpers `merge_packets` / `is_same_pkt`
 * and the global `lgw_context` are reachable) and evaluates a small set of
 * pure, hardware-independent functions on commands read from stdin, printing
 * one result line per command on stdout.
 *
 * The Rust side (`lr1302/src/conformance.rs`) computes the same values with
 * the ported implementations and asserts that the two outputs are identical.
 * See tools/difftest/README.md for the protocol.
 *
 * Everything that touches SPI / I2C / USB / GPS is stubbed out in stubs.c:
 * none of the functions exercised here performs I/O (that is the point — they
 * are the parts of the port that must match bit-for-bit).
 *
 * Build: tools/difftest/build.ps1  (invoked automatically by `cargo test`)
 */

#include <stdio.h>
#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

/* Header order matters: loragw_sx1302.h uses types from loragw_hal.h. */
#include "loragw_hal.h"
#include "loragw_sx1302.h"
#include "loragw_sx1302_timestamp.h"

/* Not exported by any vendor header (loragw_hal.c declares it itself). */
int calculate_freq_to_time_drift(uint32_t freq_hz, uint8_t bw, uint16_t *mant,
                                 uint8_t *exp);

/* ---------------------------------------------------------------------------
 * Symbols the C HAL expects from its application / platform.
 * ------------------------------------------------------------------------- */

/* MinGW has no qsort_r(); loragw_hal.c calls it to sort the merged packet
 * array. Declared with the glibc signature and implemented in stubs.c with a
 * plain merge sort — the comparator (compare_pkt_tmst) is the C HAL's, only
 * the sorting algorithm is ours. */
typedef int (*difftest_qsort_r_cmp)(const void *, const void *, void *);
void qsort_r(void *base, size_t nmemb, size_t size, difftest_qsort_r_cmp compar,
             void *arg);

/* Pull in the whole of loragw_hal.c: gives us lgw_* public functions, the
 * file-static merge helpers and the global lgw_context. */
#include "loragw_hal.c"

/* ---------------------------------------------------------------------------
 * stdin helpers
 * ------------------------------------------------------------------------- */

#define LINE_MAX 8192

static char line[LINE_MAX];
static char *toks[64];
static int ntok;

/* split the current line into whitespace-separated tokens */
static void split(void) {
    ntok = 0;
    for (char *p = line; *p != '\0' && ntok < 64;) {
        while (*p == ' ' || *p == '\t' || *p == '\r' || *p == '\n') {
            *p++ = '\0';
        }
        if (*p == '\0') break;
        toks[ntok++] = p;
        while (*p != '\0' && *p != ' ' && *p != '\t' && *p != '\r' && *p != '\n') p++;
    }
}

static uint32_t u32t(int i) { return (uint32_t)strtoul(toks[i], NULL, 10); }
static uint8_t u8t(int i) { return (uint8_t)strtoul(toks[i], NULL, 10); }
static bool boolt(int i) { return u32t(i) != 0; }

/* floats travel as raw IEEE-754 bit patterns so no precision is lost between
 * the two languages */
static float f32t(int i) {
    uint32_t bits = (uint32_t)strtoul(toks[i], NULL, 16);
    float f;
    memcpy(&f, &bits, sizeof f);
    return f;
}

static void print_f32(float f) {
    uint32_t bits;
    memcpy(&bits, &f, sizeof bits);
    printf("%08x", bits);
}

/* decode a hex string into a byte buffer; returns the byte count */
static int unhex(const char *s, uint8_t *out, int max) {
    int n = 0;
    while (s[0] != '\0' && s[1] != '\0' && n < max) {
        unsigned v;
        if (sscanf(s, "%2x", &v) != 1) break;
        out[n++] = (uint8_t)v;
        s += 2;
    }
    return n;
}

/* ---------------------------------------------------------------------------
 * command handlers
 * ------------------------------------------------------------------------- */

/* toa_lora <bw> <sf> <cr> <preamble> <no_header> <no_crc> <size>
 *   -> toa_lora <toa_us> <t_symbol_us> <nb_symbols_payload> <nb_symbols_quarters> */
static void cmd_toa_lora(void) {
    double nb_symbols = -1.0;
    uint32_t nb_payload = 0;
    uint16_t t_symbol = 0;
    uint32_t toa_us = lora_packet_time_on_air(u8t(1), u8t(2), u8t(3), (uint16_t)u32t(4),
                                              boolt(5), boolt(6), u8t(7), &nb_symbols,
                                              &nb_payload, &t_symbol);
    /* nb_symbols has a 1/4 symbol granularity, hence the *4 */
    uint32_t quarters = (uint32_t)(nb_symbols * 4.0 + 0.5);
    printf("toa_lora %u %u %u %u\n", toa_us, (unsigned)t_symbol, nb_payload, quarters);
}

/* toa_hal_lora <bw> <sf> <cr> <preamble> <no_header> <no_crc> <size>
 *   -> toa_hal_lora <ms>                                              (lgw_time_on_air) */
static void cmd_toa_hal_lora(void) {
    struct lgw_pkt_tx_s p;
    memset(&p, 0, sizeof p);
    p.modulation = MOD_LORA;
    p.bandwidth = u8t(1);
    p.datarate = u32t(2);
    p.coderate = u8t(3);
    p.preamble = (uint16_t)u32t(4);
    p.no_header = boolt(5);
    p.no_crc = boolt(6);
    p.size = (uint16_t)u32t(7);
    printf("toa_hal_lora %u\n", lgw_time_on_air(&p));
}

/* toa_fsk <datarate> <preamble> <sync_word_size> <size> <no_crc>
 *   -> toa_fsk <ms>                                                   (lgw_time_on_air) */
static void cmd_toa_fsk(void) {
    struct lgw_pkt_tx_s p;
    memset(&p, 0, sizeof p);
    p.modulation = MOD_FSK;
    p.datarate = u32t(1);
    p.preamble = (uint16_t)u32t(2);
    p.size = (uint16_t)u32t(4);
    p.no_crc = boolt(5);
    /* lgw_time_on_air() takes the FSK sync word size from the context; the
     * real API sets it through lgw_rxif_setconf() on IF chain 9 */
    lgw_context.fsk_cfg.sync_word_size = u8t(3);
    printf("toa_fsk %u\n", lgw_time_on_air(&p));
}

/* crc <size> <hex-payload> -> crc <u16> */
static void cmd_crc(void) {
    uint8_t buf[256];
    /* the payload token is omitted for a zero-length payload */
    int n = (ntok > 2) ? unhex(toks[2], buf, (int)sizeof buf) : 0;
    printf("crc %u\n", (unsigned)sx1302_lora_payload_crc(buf, (uint8_t)n));
}

/* rssioff <a> <b> <c> <d> <e> <temperature>   (all as f32 bit patterns)
 *   -> rssioff <f32-bit-pattern of the offset> */
static void cmd_rssioff(void) {
    struct lgw_rssi_tcomp_s c;
    c.coeff_a = f32t(1);
    c.coeff_b = f32t(2);
    c.coeff_c = f32t(3);
    c.coeff_d = f32t(4);
    c.coeff_e = f32t(5);
    float temp = f32t(6);
    float off = sx1302_rssi_get_temperature_offset(&c, temp);
    printf("rssioff ");
    print_f32(off);
    printf("\n");
}

/* drift <freq_hz> <bw> -> drift <ret> <mantissa> <exponent> */
static void cmd_drift(void) {
    uint16_t mant = 0;
    uint8_t exp = 0;
    int ret = calculate_freq_to_time_drift(u32t(1), u8t(2), &mant, &exp);
    printf("drift %d %u %u\n", ret, (unsigned)mant, (unsigned)exp);
}

/* ifmod <if_chain> -> ifmod <value> */
static void cmd_ifmod(void) {
    printf("ifmod %u\n", (unsigned)sx1302_get_ifmod_config(u8t(1)));
}

/* tscorr <ftime_enable> <bw> <dr> <cr> <crc_en> <payload_len> <dft_peak_mode>
 *   -> tscorr <correction> */
static void cmd_tscorr(void) {
    lgw_context.ftime_cfg.enable = boolt(1);
    int32_t c = timestamp_counter_correction(&lgw_context, u8t(2), u8t(3), u8t(4), boolt(5),
                                             u8t(6), (sx1302_rx_dft_peak_mode_t)u8t(7));
    printf("tscorr %d\n", c);
}

/* merge <n>                                            (n packets follow, one per line)
 *     <count_us> <if_chain> <rf_chain> <datarate> <size> <status> <hex-payload>
 *   -> merge <k> <count_us of the k surviving packets, ascending> */
static void cmd_merge(void) {
    unsigned n = u32t(1);
    if (n > 255) n = 255;
    static struct lgw_pkt_rx_s pkts[256];
    memset(pkts, 0, sizeof pkts);
    for (unsigned i = 0; i < n; i++) {
        if (fgets(line, sizeof line, stdin) == NULL) break;
        split();
        if (ntok < 7) break;
        pkts[i].count_us = u32t(0);
        pkts[i].if_chain = u8t(1);
        pkts[i].rf_chain = u8t(2);
        pkts[i].datarate = u32t(3);
        pkts[i].size = (uint16_t)u32t(4);
        pkts[i].status = u8t(5);
        if (ntok > 6) {
            unhex(toks[6], pkts[i].payload, (int)sizeof pkts[i].payload);
        }
        /* no fine timestamps in this port: both sides keep ftime_received = 0 */
        pkts[i].ftime_received = false;
    }
    uint8_t nb = (uint8_t)n;
    merge_packets(pkts, &nb);
    printf("merge %u", (unsigned)nb);
    for (unsigned i = 0; i < nb; i++) {
        printf(" %u", pkts[i].count_us);
    }
    printf("\n");
}

/* ---------------------------------------------------------------------------
 * main
 * ------------------------------------------------------------------------- */

int main(void) {
    setvbuf(stdout, NULL, _IOLBF, 0);
    while (fgets(line, sizeof line, stdin) != NULL) {
        split();
        if (ntok == 0) continue;
        const char *cmd = toks[0];
        if (strcmp(cmd, "toa_lora") == 0) {
            cmd_toa_lora();
        } else if (strcmp(cmd, "toa_hal_lora") == 0) {
            cmd_toa_hal_lora();
        } else if (strcmp(cmd, "toa_fsk") == 0) {
            cmd_toa_fsk();
        } else if (strcmp(cmd, "crc") == 0) {
            cmd_crc();
        } else if (strcmp(cmd, "rssioff") == 0) {
            cmd_rssioff();
        } else if (strcmp(cmd, "drift") == 0) {
            cmd_drift();
        } else if (strcmp(cmd, "ifmod") == 0) {
            cmd_ifmod();
        } else if (strcmp(cmd, "tscorr") == 0) {
            cmd_tscorr();
        } else if (strcmp(cmd, "merge") == 0) {
            cmd_merge();
        } else if (strcmp(cmd, "quit") == 0) {
            break;
        } else {
            printf("error unknown-command\n");
        }
    }
    return 0;
}
