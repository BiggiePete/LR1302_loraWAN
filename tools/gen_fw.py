#!/usr/bin/env python3
"""Convert Semtech SX1302 firmware `.var` files (ASCII hex) to raw binaries.

Output: lr1302/fw/*.bin, consumed via include_bytes!().
"""

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SRC = ROOT / "LR1302_HAL" / "sx1302_hal" / "libloragw" / "src"
OUT = ROOT / "lr1302" / "fw"

# .var file -> output name
FIRMWARES = {
    "agc_fw_sx1250.var": "agc_fw_sx1250.bin",
    "arb_fw.var": "arb_fw.bin",
}

EXPECTED_SIZE = 8192  # MCU_FW_SIZE in loragw_sx1302.c


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    for src_name, out_name in FIRMWARES.items():
        text = (SRC / src_name).read_text(encoding="utf-8", errors="replace")
        data = bytes(int(x, 16) for x in re.findall(r"0x([0-9A-Fa-f]{2})", text))
        if len(data) != EXPECTED_SIZE:
            raise SystemExit(
                f"{src_name}: got {len(data)} bytes, expected {EXPECTED_SIZE}"
            )
        (OUT / out_name).write_bytes(data)
        print(f"wrote {OUT / out_name} ({len(data)} bytes)")


if __name__ == "__main__":
    sys.exit(main())
