#!/usr/bin/env pwsh
# Build McLoad release artifacts for the documented target matrix.
# Usage: pwsh -File scripts/build-release-multi.ps1
# Env / params:
#   -Strict     — treat skipped targets as failure
#   -UseCross   — prefer `cross build` when available
#   -DistDir    — output root (default: dist/release)

[CmdletBinding()]
param(
    [switch]$Strict,
    [switch]$UseCross,
    [string]$DistDir = "dist/release"
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

if ($env:STRICT -eq "1") { $Strict = $true }
if ($env:USE_CROSS -eq "1") { $UseCross = $true }
if ($env:DIST_DIR) { $DistDir = $env:DIST_DIR }

$HostTriple = (rustc -vV | Select-String '^host:').ToString().Split(':', 2)[1].Trim()

$Targets = @(
    "x86_64-unknown-linux-gnu",
    "x86_64-pc-windows-msvc",
    "aarch64-apple-darwin"
)

New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

function Test-Cmd([string]$Name) {
    return [bool](Get-Command $Name -ErrorAction SilentlyContinue)
}

function Resolve-WindowsTarget {
    if ($HostTriple -match 'windows-msvc$') { return "x86_64-pc-windows-msvc" }
    if ($HostTriple -match 'windows-gnu$') { return "x86_64-pc-windows-gnu" }
    if ((Test-Cmd "x86_64-w64-mingw32-gcc") -or (Test-Cmd "x86_64-w64-mingw32-g++")) {
        return "x86_64-pc-windows-gnu"
    }
    return "x86_64-pc-windows-msvc"
}

function Get-BinName([string]$Target) {
    if ($Target -match 'windows') { return "mcload.exe" }
    return "mcload"
}

function Test-CanAttempt([string]$Target) {
    switch -Regex ($Target) {
        '^aarch64-apple-darwin$' {
            if ($HostTriple -match 'apple-darwin$') { return $true }
            if ($UseCross -and (Test-Cmd "cross")) { return $true }
            Write-Host "skip: $Target needs a macOS host (or -UseCross with a configured cross toolchain)"
            return $false
        }
        '^x86_64-pc-windows-msvc$' {
            if ($HostTriple -match 'windows-msvc$') { return $true }
            Write-Host "skip: $Target needs Windows MSVC; use windows-gnu when MinGW is available"
            return $false
        }
        '^x86_64-pc-windows-gnu$' {
            if ($HostTriple -match 'windows') { return $true }
            if ((Test-Cmd "x86_64-w64-mingw32-gcc") -or (Test-Cmd "x86_64-w64-mingw32-g++")) { return $true }
            if ($UseCross -and (Test-Cmd "cross")) { return $true }
            Write-Host "skip: $Target needs MinGW or cross"
            return $false
        }
        default { return $true }
    }
}

function Build-One([string]$Target) {
    Write-Host ""
    Write-Host "=== $Target ==="

    if (-not (Test-CanAttempt $Target)) {
        return "SKIP"
    }

    rustup target add $Target | Out-Null

    $builder = "cargo"
    if ($UseCross -and (Test-Cmd "cross") -and ($Target -ne $HostTriple)) {
        $builder = "cross"
    }

    Write-Host "building with: $builder build --release --target $Target"
    & $builder build --release --target $Target
    if ($LASTEXITCODE -ne 0) {
        Write-Host "FAIL: build failed for $Target"
        return "FAIL"
    }

    $bin = Get-BinName $Target
    $src = Join-Path "target" $Target "release" $bin
    if (-not (Test-Path -LiteralPath $src)) {
        Write-Host "FAIL: missing artifact $src"
        return "FAIL"
    }

    $destDir = Join-Path $DistDir $Target
    New-Item -ItemType Directory -Force -Path $destDir | Out-Null
    Copy-Item -Force $src (Join-Path $destDir $bin)
    Write-Host "artifact: $(Join-Path $destDir $bin)"
    return "OK"
}

$Effective = foreach ($t in $Targets) {
    if ($t -eq "x86_64-pc-windows-msvc") { Resolve-WindowsTarget } else { $t }
}
$Declared = [System.Collections.Generic.List[string]]::new()
foreach ($t in $Effective) {
    if (-not $Declared.Contains($t)) { [void]$Declared.Add($t) }
}

$ok = 0; $failed = 0; $skipped = 0
$summary = @()

foreach ($target in $Declared) {
    $status = Build-One $target
    switch ($status) {
        "OK"   { $ok++; $summary += "OK      $target" }
        "SKIP" { $skipped++; $summary += "SKIP    $target" }
        default { $failed++; $summary += "FAIL    $target" }
    }
}

Write-Host ""
Write-Host "=== release multi-target summary ==="
$summary | ForEach-Object { Write-Host $_ }
Write-Host "ok=$ok failed=$failed skipped=$skipped dist=$DistDir"

if ($failed -gt 0) { exit 1 }
if ($Strict -and $skipped -gt 0) {
    Write-Error "STRICT: skipped targets count as failure"
    exit 1
}
exit 0
