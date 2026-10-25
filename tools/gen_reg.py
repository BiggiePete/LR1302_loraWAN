#!/usr/bin/env python3
"""Generate lr1302/src/reg.rs from the Semtech SX1302 HAL C sources.

Inputs:
  LR1302_HAL/sx1302_hal/libloragw/inc/loragw_reg.h  (register name -> index)
  LR1302_HAL/sx1302_hal/libloragw/src/loragw_reg.c  (loregs[] descriptor table)

Output:
  lr1302/src/reg.rs
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
HAL = ROOT / "LR1302_HAL" / "sx1302_hal" / "libloragw"
REG_H = HAL / "inc" / "loragw_reg.h"
REG_C = HAL / "src" / "loragw_reg.c"
OUT = ROOT / "lr1302" / "src" / "reg.rs"

# ---------------------------------------------------------------------------
# 1. base addresses (defined in loragw_reg.c)
# ---------------------------------------------------------------------------

def parse_bases(text):
    bases = {}
    for name, val in re.findall(
        r"^#define\s+(SX1302_REG_\w*BASE_ADDR)\s+(0x[0-9A-Fa-f]+|\d+)",
        text,
        re.M,
    ):
        bases[name] = int(val, 0)
    return bases


# ---------------------------------------------------------------------------
# 2. loregs[] rows
# ---------------------------------------------------------------------------

ROW_RE = re.compile(r"^\s*\{([^{}]+)\}\s*,\s*//\s*(\S+)\s*$")
TERM_RE = re.compile(r"^\s*\{0,0,0,0,0,0,0,0\}\s*,?\s*$")


def eval_addr(expr, bases):
    expr = expr.strip()
    if "+" in expr:
        name, off = expr.split("+", 1)
        return bases[name.strip()] + int(off.strip(), 0)
    if expr in bases:
        return bases[expr]
    return int(expr, 0)


def parse_loregs(text, bases):
    rows = []
    for line in text.splitlines():
        if TERM_RE.match(line):
            continue
        m = ROW_RE.match(line)
        if not m:
            continue
        fields = [f.strip() for f in m.group(1).split(",")]
        if len(fields) != 8:
            raise SystemExit(f"bad row field count {len(fields)}: {line!r}")
        page, addr, offs, sign, leng, rdon, chck, dflt = fields
        rows.append(
            dict(
                addr=eval_addr(addr, bases),
                offs=int(offs, 0),
                leng=int(leng, 0),
                sign=bool(int(sign, 0)) if sign not in ("true", "false") else sign == "true",
                rdon=bool(int(rdon, 0)) if rdon not in ("true", "false") else rdon == "true",
                chck=bool(int(chck, 0)) if chck not in ("true", "false") else chck == "true",
                dflt=int(dflt, 0),
                comment=m.group(2),
            )
        )
    return rows


# ---------------------------------------------------------------------------
# 3. header: name -> index, and rf_chain selector macros
# ---------------------------------------------------------------------------

DEFINE_RE = re.compile(r"^#define\s+(SX1302_REG_\w+)\s+(\d+)\s*$", re.M)

CHAIN_RE = re.compile(
    r"#define\s+(SX1302_REG_\w+)\(rf_chain\)\s*"
    r"\(\(rf_chain == 0\) \? \\?\s*(\w+) : \\?\s*(\w+)\)",
)


def parse_header(text):
    defines = [(name, int(idx)) for name, idx in DEFINE_RE.findall(text)]
    chains = CHAIN_RE.findall(text)
    return defines, chains


# ---------------------------------------------------------------------------
# 4. emit rust
# ---------------------------------------------------------------------------

def rust_ident(name):
    # SX1302_REG_TX_TOP_... -> keep as-is (valid SCREAMING_SNAKE identifier)
    return name


def snake(name):
    # macro names are SCREAMING_SNAKE already; just lowercase
    return name.lower()


def main():
    reg_h = REG_H.read_text(encoding="utf-8", errors="replace")
    reg_c = REG_C.read_text(encoding="utf-8", errors="replace")

    bases = parse_bases(reg_c)
    rows = parse_loregs(reg_c, bases)
    defines, chains = parse_header(reg_h)

    total = None
    m = re.search(r"^#define\s+LGW_TOTALREGS\s+(\d+)", reg_h, re.M)
    if m:
        total = int(m.group(1))

    if total is None:
        total = len(defines)
    if len(rows) != total:
        raise SystemExit(f"row count {len(rows)} != LGW_TOTALREGS {total}")
    if len(defines) != total:
        raise SystemExit(f"define count {len(defines)} != LGW_TOTALREGS {total}")

    # index alignment check: define[i] must correspond to row[i] comment
    by_idx = {idx: name for name, idx in defines}
    for i, row in enumerate(rows):
        name = by_idx.get(i)
        if name is None:
            raise SystemExit(f"missing define for index {i}")
        c = row["comment"]
        if not (name == c or name.endswith(c)):
            raise SystemExit(
                f"index {i} mismatch: define {name} vs row comment {c}"
            )

    dflt_fits_i16 = all(-32768 <= r["dflt"] <= 32767 for r in rows)
    if not dflt_fits_i16:
        raise SystemExit("dflt value out of i16 range")

    lines = []
    a = lines.append
    a("//! Register map for the Semtech SX1302 concentrator.")
    a("//!")
    a("//! GENERATED FILE - do not edit by hand.")
    a("//! Produced by `tools/gen_reg.py` from the Semtech SX1302 HAL v2.1.0")
    a("//! sources (`loragw_reg.h` / `loragw_reg.c`).")
    a("//!")
    a("//! Names and indices are identical to the C HAL `#define` values, so a")
    a("//! register referenced as `SX1302_REG_...` in the C sources maps 1:1 to")
    a("//! the Rust constant of the same name.")
    a("")
    a("/// Number of registers in [`LOREGS`] (C: `LGW_TOTALREGS`).")
    a(f"pub const LGW_TOTALREGS: u16 = {total};")
    a("")
    a("/// Descriptor of one register, mirroring `struct lgw_reg_s`.")
    a("///")
    a("/// The legacy C `page` field is dropped: it is never read by the C")
    a("/// register access routines.")
    a("#[derive(Clone, Copy, Debug)]")
    a("pub struct RegDesc {")
    a("    /// 15-bit register address")
    a("    pub addr: u16,")
    a("    /// bit position of the LSB inside the addressed byte (0..7)")
    a("    pub offs: u8,")
    a("    /// field width in bits")
    a("    pub leng: u8,")
    a("    /// reset/default value")
    a("    pub dflt: i16,")
    a("    /// bit flags, see `FLAG_*`")
    a("    pub flags: u8,")
    a("}")
    a("")
    a("/// Field is signed (2's complement)")
    a("pub const FLAG_SIGN: u8 = 0x01;")
    a("/// Field is read-only")
    a("pub const FLAG_RDON: u8 = 0x02;")
    a("/// Field can be checked (pulse, w0clr, w1clr)")
    a("pub const FLAG_CHCK: u8 = 0x04;")
    a("")
    a("impl RegDesc {")
    a("    pub const fn is_signed(&self) -> bool { self.flags & FLAG_SIGN != 0 }")
    a("    pub const fn is_read_only(&self) -> bool { self.flags & FLAG_RDON != 0 }")
    a("    pub const fn is_checkable(&self) -> bool { self.flags & FLAG_CHCK != 0 }")
    a("}")
    a("")
    a("/// Register descriptors, indexed by the register name constants below")
    a(f"/// (C: `loregs[]`, {total} entries).")
    a("pub static LOREGS: [RegDesc; LGW_TOTALREGS as usize] = [")
    for i, r in enumerate(rows):
        flags = (1 if r["sign"] else 0) | (2 if r["rdon"] else 0) | (4 if r["chck"] else 0)
        a(
            f"    RegDesc {{ addr: 0x{r['addr']:04X}, offs: {r['offs']}, "
            f"leng: {r['leng']}, dflt: {r['dflt']}, flags: 0x{flags:02X} }}, "
            f"// {i}: {r['comment']}"
        )
    a("];")
    a("")
    a("// ---------------------------------------------------------------------------")
    a("// Register name constants (C: `#define SX1302_REG_* <index>`)")
    a("// ---------------------------------------------------------------------------")
    a("")
    for name, idx in defines:
        a(f"pub const {rust_ident(name)}: u16 = {idx};")
    a("")
    a("// ---------------------------------------------------------------------------")
    a("// RF-chain specific register selectors (C: `#define NAME(rf_chain) ...`)")
    a("//")
    a("// `rf_chain` is 0 for radio A and 1 for radio B.")
    a("// ---------------------------------------------------------------------------")
    a("")
    for macro, a_name, b_name in chains:
        a_idx = dict(defines)[a_name]
        b_idx = dict(defines)[b_name]
        fn = snake(macro[len("SX1302_REG_"):])
        a(f"/// C: `{macro}(rf_chain)`")
        a(f"pub const fn {fn}(rf_chain: u8) -> u16 {{")
        a(f"    if rf_chain == 0 {{ {a_idx} }} else {{ {b_idx} }}")
        a("}")
        a("")

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text("\n".join(lines), encoding="utf-8")
    print(f"wrote {OUT} ({total} registers, {len(chains)} rf-chain selectors)")


if __name__ == "__main__":
    sys.exit(main())
