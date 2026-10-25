/*
 * Link stubs for the C conformance harness.
 *
 * None of the functions exercised by harness.c performs I/O, but the objects
 * they live in (loragw_hal.c, loragw_sx1302.c, ...) still reference the
 * SPI/I2C/GPS/register layers at link time.  Every symbol below is a no-op
 * (or a plausible constant) that is never reached at run time — if it ever
 * *is* reached, the harness prints `error stub-called` and aborts so a
 * silently-wrong comparison cannot happen.
 *
 * Exceptions handled here for real:
 *  - qsort_r: MinGW's mingwrt has no qsort_r(); loragw_hal.c needs it for
 *    merge_packets().  We implement it with a plain merge sort, using the
 *    comparator the C HAL passes in (so only the algorithm is ours).
 *    `log_file` is provided by loragw_hal.c itself (normally the packet
 *    forwarder defines it — the include in harness.c gives it to us).
 */

#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <stdbool.h>
#include <time.h>

#include "loragw_hal.h"
#include "loragw_sx1302.h"

/* -------------------------------------------------------------------------- */
/* --- POSIX bits missing from MinGW's runtime ------------------------------ */

/* loragw_aux.c calls clock_nanosleep(CLOCK_MONOTONIC, ...) for wait_ms /
 * wait_us; mingw.org's runtime has nanosleep() but neither CLOCK_MONOTONIC
 * (defined on the command line by build.ps1) nor clock_nanosleep().  This
 * wrapper forwards to nanosleep(), which has the identical semantics for a
 * 0-flags, non-absolutetime request.  Never reached by the harness: none of
 * the tested functions delays. */
extern int nanosleep(const struct timespec *req, struct timespec *rem);

int clock_nanosleep(int clock_id, int flags, const struct timespec *req,
                    struct timespec *rem) {
    (void)clock_id;
    (void)flags;
    return nanosleep(req, rem);
}

/* -------------------------------------------------------------------------- */
/* --- qsort_r (merge sort) ------------------------------------------------- */

typedef int (*qsort_r_cmp)(const void *, const void *, void *);

static void merge_sort_rec(char *base, size_t n, size_t size, qsort_r_cmp cmp,
                           void *arg, char *tmp) {
    if (n < 2) return;
    size_t mid = n / 2;
    merge_sort_rec(base, mid, size, cmp, arg, tmp);
    merge_sort_rec(base + mid * size, n - mid, size, cmp, arg, tmp);

    /* merge into tmp, stable: left element wins on cmp() == 0 */
    char *l = base;
    char *r = base + mid * size;
    char *le = base + mid * size;
    char *re = base + n * size;
    char *o = tmp;
    while (l != le && r != re) {
        if (cmp(r, l, arg) < 0) {
            memcpy(o, r, size);
            r += size;
        } else {
            memcpy(o, l, size);
            l += size;
        }
        o += size;
    }
    while (l != le) { memcpy(o, l, size); l += size; o += size; }
    while (r != re) { memcpy(o, r, size); r += size; o += size; }
    memcpy(base, tmp, n * size);
}

void qsort_r(void *base, size_t nmemb, size_t size, qsort_r_cmp compar,
             void *arg) {
    if (base == NULL || nmemb < 2 || size == 0) return;
    char *tmp = (char *)malloc(nmemb * size);
    if (tmp == NULL) return; /* merge_packets() reports its own failure */
    merge_sort_rec((char *)base, nmemb, size, compar, arg, tmp);
    free(tmp);
}

/* -------------------------------------------------------------------------- */
/* --- never-reached guard -------------------------------------------------- */

#define STUB(...) do { \
    fprintf(stderr, "STUB CALLED: %s (this should be unreachable)\n", __VA_ARGS__); \
    abort(); \
} while (0)

/* The prototypes below are the vendor's own (loragw_*.h), included so that a
 * signature mismatch is a compile error rather than undefined behaviour. */

#include "loragw_com.h"
#include "loragw_reg.h"
#include "loragw_spi.h"
#include "loragw_i2c.h"
#include "loragw_lbt.h"
#include "loragw_cal.h"
#include "loragw_ad5338r.h"
#include "loragw_stts751.h"
#include "loragw_sx1250.h"
#include "loragw_sx125x.h"
#include "loragw_sx1261.h"

/* -------------------------------------------------------------------------- */
/* --- connection / register / bus layer (loragw_reg.c, loragw_com.c) -------- */

int lgw_connect(const lgw_com_type_t com_type, const char *com_path) {
    (void)com_type; (void)com_path; STUB("lgw_connect");
}
int lgw_disconnect(void) { STUB("lgw_disconnect"); }

int lgw_reg_r(uint16_t address, int32_t *value) { (void)address; (void)value; STUB("lgw_reg_r"); }
int lgw_reg_w(uint16_t address, int32_t value) { (void)address; (void)value; STUB("lgw_reg_w"); }
int lgw_reg_rbw(uint16_t address, uint8_t *value) { (void)address; (void)value; STUB("lgw_reg_rbw"); }
int lgw_reg_wb(uint16_t register_id, uint8_t *data, uint16_t size) {
    (void)register_id; (void)data; (void)size; STUB("lgw_reg_wb");
}
int lgw_reg_rb(uint16_t register_id, uint8_t *data, uint16_t size) {
    (void)register_id; (void)data; (void)size; STUB("lgw_reg_rb");
}
int lgw_reg_set_bits(uint16_t address, uint32_t bits) { (void)address; (void)bits; STUB("lgw_reg_set_bits"); }
int lgw_reg_clr_bits(uint16_t address, uint32_t bits) { (void)address; (void)bits; STUB("lgw_reg_clr_bits"); }
int lgw_reg_tmst_set(uint32_t value) { (void)value; STUB("lgw_reg_tmst_set"); }
int lgw_mem_wb(uint16_t mem_addr, const uint8_t *data, uint16_t size) {
    (void)mem_addr; (void)data; (void)size; STUB("lgw_mem_wb");
}
int lgw_mem_rb(uint16_t mem_addr, uint8_t *data, uint16_t size, bool fifo_mode) {
    (void)mem_addr; (void)data; (void)size; (void)fifo_mode; STUB("lgw_mem_rb");
}

int lgw_com_open(const lgw_com_type_t com_type, const char *com_path) {
    (void)com_type; (void)com_path; STUB("lgw_com_open");
}
int lgw_com_close(void) { STUB("lgw_com_close"); }
int lgw_com_w(uint8_t spi_mux_target, uint16_t address, uint8_t data) {
    (void)spi_mux_target; (void)address; (void)data; STUB("lgw_com_w");
}
int lgw_com_r(uint8_t spi_mux_target, uint16_t address, uint8_t *data) {
    (void)spi_mux_target; (void)address; (void)data; STUB("lgw_com_r");
}
int lgw_com_rmw(uint8_t spi_mux_target, uint16_t address, uint8_t offs,
                uint8_t leng, uint8_t data) {
    (void)spi_mux_target; (void)address; (void)offs; (void)leng; (void)data;
    STUB("lgw_com_rmw");
}
int lgw_com_wb(uint8_t spi_mux_target, uint16_t address, const uint8_t *data,
               uint16_t size) {
    (void)spi_mux_target; (void)address; (void)data; (void)size;
    STUB("lgw_com_wb");
}
int lgw_com_rb(uint8_t spi_mux_target, uint16_t address, uint8_t *data,
               uint16_t size) {
    (void)spi_mux_target; (void)address; (void)data; (void)size;
    STUB("lgw_com_rb");
}
int lgw_com_set_write_mode(lgw_com_write_mode_t write_mode) {
    (void)write_mode; STUB("lgw_com_set_write_mode");
}
int lgw_com_flush(void) { STUB("lgw_com_flush"); }
int lgw_com_get_temperature(float *temperature) { (void)temperature; STUB("lgw_com_get_temperature"); }

/* --- SPI (loragw_spi.c) ---------------------------------------------------- */

int lgw_spi_open(const char *path, void **com_target) { (void)path; (void)com_target; STUB("lgw_spi_open"); }
int lgw_spi_close(void *com_target) { (void)com_target; STUB("lgw_spi_close"); }
int lgw_spi_w(void *com_target, uint8_t spi_mux_target, uint16_t address,
              uint8_t data) {
    (void)com_target; (void)spi_mux_target; (void)address; (void)data;
    STUB("lgw_spi_w");
}
int lgw_spi_r(void *com_target, uint8_t spi_mux_target, uint16_t address,
              uint8_t *data) {
    (void)com_target; (void)spi_mux_target; (void)address; (void)data;
    STUB("lgw_spi_r");
}
int lgw_spi_rmw(void *com_target, uint8_t spi_mux_target, uint16_t address,
                uint8_t offs, uint8_t leng, uint8_t data) {
    (void)com_target; (void)spi_mux_target; (void)address; (void)offs;
    (void)leng; (void)data; STUB("lgw_spi_rmw");
}
int lgw_spi_wb(void *com_target, uint8_t spi_mux_target, uint16_t address,
               const uint8_t *data, uint16_t size) {
    (void)com_target; (void)spi_mux_target; (void)address; (void)data;
    (void)size; STUB("lgw_spi_wb");
}
int lgw_spi_rb(void *com_target, uint8_t spi_mux_target, uint16_t address,
               uint8_t *data, uint16_t size) {
    (void)com_target; (void)spi_mux_target; (void)address; (void)data;
    (void)size; STUB("lgw_spi_rb");
}
uint16_t lgw_spi_chunk_size(void) { return 0; }

/* --- I2C (loragw_i2c.c) ---------------------------------------------------- */

int i2c_linuxdev_open(const char *path, uint8_t device_addr, int *i2c_fd) {
    (void)path; (void)device_addr; (void)i2c_fd; STUB("i2c_linuxdev_open");
}
int i2c_linuxdev_close(int i2c_fd) { (void)i2c_fd; STUB("i2c_linuxdev_close"); }

/* --- temperature sensor (loragw_stts751.c) --------------------------------- */

int stts751_configure(int i2c_fd, uint8_t i2c_addr) {
    (void)i2c_fd; (void)i2c_addr; STUB("stts751_configure");
}
int stts751_get_temperature(int i2c_fd, uint8_t i2c_addr, float *temperature) {
    (void)i2c_fd; (void)i2c_addr; (void)temperature; STUB("stts751_get_temperature");
}

/* --- TX DAC (loragw_ad5338r.c) --------------------------------------------- */

int ad5338r_configure(int i2c_fd, uint8_t i2c_addr) {
    (void)i2c_fd; (void)i2c_addr; STUB("ad5338r_configure");
}
int ad5338r_write(int i2c_fd, uint8_t i2c_addr, uint8_t buf[static 4]) {
    (void)i2c_fd; (void)i2c_addr; (void)buf; STUB("ad5338r_write");
}

/* --- radio bring-up / calibration (loragw_sx1250.c, loragw_cal.c, ...) ----- */

int sx1250_setup(uint8_t rf_chain, uint32_t freq_hz, bool single_input_mode) {
    (void)rf_chain; (void)freq_hz; (void)single_input_mode; STUB("sx1250_setup");
}
int sx1250_calibrate(uint8_t rf_chain, uint32_t freq_hz) {
    (void)rf_chain; (void)freq_hz; STUB("sx1250_calibrate");
}
int sx125x_setup(uint8_t rf_chain, uint8_t rf_clkout, bool rf_enable,
                 uint8_t rf_radio_type, uint32_t freq_hz) {
    (void)rf_chain; (void)rf_clkout; (void)rf_enable; (void)rf_radio_type;
    (void)freq_hz; STUB("sx125x_setup");
}
int sx1302_cal_start(uint8_t version, struct lgw_conf_rxrf_s *rf_chain_cfg,
                     struct lgw_tx_gain_lut_s *txgain_lut) {
    (void)version; (void)rf_chain_cfg; (void)txgain_lut; STUB("sx1302_cal_start");
}

/* --- SX1261 (LBT / spectral scan — not part of this port) ------------------ */

int sx1261_connect(lgw_com_type_t com_type, const char *com_path) {
    (void)com_type; (void)com_path; STUB("sx1261_connect");
}
int sx1261_load_pram(void) { STUB("sx1261_load_pram"); }
int sx1261_calibrate(uint32_t freq_hz) { (void)freq_hz; STUB("sx1261_calibrate"); }
int sx1261_setup(void) { STUB("sx1261_setup"); }
int sx1261_set_rx_params(uint32_t freq_hz, uint8_t bandwidth) {
    (void)freq_hz; (void)bandwidth; STUB("sx1261_set_rx_params");
}
int sx1261_spectral_scan_start(uint16_t nb_scan) { (void)nb_scan; STUB("sx1261_spectral_scan_start"); }
int sx1261_spectral_scan_status(lgw_spectral_scan_status_t *status) {
    (void)status; STUB("sx1261_spectral_scan_status");
}
int sx1261_spectral_scan_get_results(int8_t rssi_offset, int16_t *levels_dbm,
                                     uint16_t *results) {
    (void)rssi_offset; (void)levels_dbm; (void)results;
    STUB("sx1261_spectral_scan_get_results");
}
int sx1261_spectral_scan_abort(void) { STUB("sx1261_spectral_scan_abort"); }

/* --- LBT (loragw_lbt.c) ---------------------------------------------------- */

int lgw_lbt_start(const struct lgw_conf_sx1261_s *sx1261_context,
                  const struct lgw_pkt_tx_s *pkt) {
    (void)sx1261_context; (void)pkt; STUB("lgw_lbt_start");
}
int lgw_lbt_stop(void) { STUB("lgw_lbt_stop"); }
int lgw_lbt_tx_status(uint8_t rf_chain, bool *tx_ok) {
    (void)rf_chain; (void)tx_ok; STUB("lgw_lbt_tx_status");
}

/* --- debug helpers (loragw_debug.c, which would drag in tinymt32) ---------- */
/* Both are no-ops in the real library when no log_file is set (DEBUG_SX1302
 * is 0 in our config.h and log_file is NULL), so replacing them with no-ops
 * changes nothing observable. */

void dbg_log_buffer_to_file(FILE *file, uint8_t *buffer, uint16_t size) {
    (void)file; (void)buffer; (void)size;
}
void dbg_init_random(void) { }

