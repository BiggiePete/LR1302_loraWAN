# Build the C side of the conformance difftest.
#
#   powershell -File tools/difftest/build.ps1
#
# Produces tools/difftest/difftest_c.exe from the *unmodified* Semtech HAL
# sources plus harness.c / stubs.c in this directory.

$ErrorActionPreference = "Stop"

$root     = Resolve-Path (Join-Path $PSScriptRoot "..\..")
$difftest = Join-Path $root "tools\difftest"
$hal      = Join-Path $root "LR1302_HAL\sx1302_hal"
$lib      = Join-Path $hal "libloragw"
$objdir   = Join-Path $difftest "obj"

if (-not (Test-Path $objdir)) { New-Item -ItemType Directory -Path $objdir | Out-Null }

$CC = "gcc"
if (Test-Path "C:\MinGW\bin\gcc.exe") { $CC = "C:\MinGW\bin\gcc.exe" }

$cflags = @(
    "-std=c99", "-O1", "-Wall",
    # MinGW's runtime has neither CLOCK_MONOTONIC nor clock_nanosleep();
    # stubs.c provides the latter, build.ps1 the former (see stubs.c).
    "-DCLOCK_MONOTONIC=1",
    "-I$lib\inc", "-I$lib\src", "-I$lib\.",
    "-I$(Join-Path $hal 'libtools\inc')",
    "-I$difftest\cinclude"
)

# Sources compiled directly (harness.c #includes loragw_hal.c itself so that
# its file-static merge helpers and the global lgw_context stay reachable).
$sources = @(
    "$difftest\harness.c",
    "$difftest\stubs.c",
    "$lib\src\loragw_sx1302.c",
    "$lib\src\loragw_sx1302_timestamp.c",
    "$lib\src\loragw_sx1302_rx.c",
    "$lib\src\loragw_aux.c"
)

$objs = @()
foreach ($src in $sources) {
    $name = [IO.Path]::GetFileNameWithoutExtension($src) + ".o"
    $obj  = Join-Path $objdir $name
    & $CC @cflags -c -o $obj $src
    if ($LASTEXITCODE -ne 0) { throw "compile failed: $src" }
    $objs += $obj
}

$out = Join-Path $difftest "difftest_c.exe"
& $CC @objs -o $out -lm
if ($LASTEXITCODE -ne 0) { throw "link failed" }

Write-Output "built $out"
