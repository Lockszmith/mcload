#!/usr/bin/env bash
# Build McLoad release artifacts for the documented target matrix.
# Usage: scripts/build-release-multi.sh
# Env:
#   STRICT=1     — treat skipped targets as failure (default: 0)
#   USE_CROSS=1  — prefer `cross build` over `cargo build` when `cross` is on PATH
#   DIST_DIR     — output root (default: dist/release)

set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

DIST_DIR="${DIST_DIR:-dist/release}"
STRICT="${STRICT:-0}"
USE_CROSS="${USE_CROSS:-0}"
HOST_TRIPLE="$(rustc -vV | awk -F': ' '/^host:/{print $2}')"

# Canonical matrix from README (Windows MSVC on Windows hosts; GNU when
# cross-building Windows from Linux/WSL with MinGW).
TARGETS=(
  "x86_64-unknown-linux-gnu"
  "x86_64-pc-windows-msvc"
  "aarch64-apple-darwin"
)

mkdir -p "$DIST_DIR"

have_cmd() { command -v "$1" >/dev/null 2>&1; }

resolve_windows_target() {
  local want="x86_64-pc-windows-msvc"
  case "$HOST_TRIPLE" in
    *-windows-msvc) echo "$want" ;;
    *-windows-gnu)  echo "x86_64-pc-windows-gnu" ;;
    *)
      if have_cmd x86_64-w64-mingw32-gcc || have_cmd x86_64-w64-mingw32-g++; then
        echo "x86_64-pc-windows-gnu"
      else
        echo "$want"
      fi
      ;;
  esac
}

bin_name_for() {
  case "$1" in
    *-windows-*) echo "mcload.exe" ;;
    *)           echo "mcload" ;;
  esac
}

can_attempt() {
  local target="$1"
  case "$target" in
    aarch64-apple-darwin)
      case "$HOST_TRIPLE" in
        *-apple-darwin) return 0 ;;
        *)
          if [[ "${USE_CROSS}" == "1" ]] && have_cmd cross; then
            return 0
          fi
          echo "skip: ${target} needs a macOS host (or USE_CROSS=1 with a configured cross toolchain)"
          return 1
          ;;
      esac
      ;;
    x86_64-pc-windows-msvc)
      case "$HOST_TRIPLE" in
        *-windows-msvc) return 0 ;;
        *)
          echo "skip: ${target} needs Windows MSVC; use windows-gnu on this host if MinGW is available"
          return 1
          ;;
      esac
      ;;
    x86_64-pc-windows-gnu)
      if have_cmd x86_64-w64-mingw32-gcc || have_cmd x86_64-w64-mingw32-g++ \
        || [[ "$HOST_TRIPLE" == *-windows-* ]]; then
        return 0
      fi
      if [[ "${USE_CROSS}" == "1" ]] && have_cmd cross; then
        return 0
      fi
      echo "skip: ${target} needs MinGW (x86_64-w64-mingw32-gcc) or cross"
      return 1
      ;;
    *)
      return 0
      ;;
  esac
}

build_one() {
  local target="$1"
  local bin
  bin="$(bin_name_for "$target")"

  echo ""
  echo "=== ${target} ==="

  if ! can_attempt "$target"; then
    return 2
  fi

  rustup target add "$target" >/dev/null

  local builder=(cargo)
  if [[ "${USE_CROSS}" == "1" ]] && have_cmd cross && [[ "$target" != "$HOST_TRIPLE" ]]; then
    builder=(cross)
  fi

  echo "building with: ${builder[*]} build --release --target ${target}"
  if ! "${builder[@]}" build --release --target "$target"; then
    echo "FAIL: build failed for ${target}" >&2
    return 1
  fi

  local src="target/${target}/release/${bin}"
  if [[ ! -f "$src" ]]; then
    echo "FAIL: missing artifact ${src}" >&2
    return 1
  fi

  local dest_dir="${DIST_DIR}/${target}"
  mkdir -p "$dest_dir"
  cp -f "$src" "${dest_dir}/${bin}"
  echo "artifact: ${dest_dir}/${bin}"
  return 0
}

# Expand the matrix: swap Windows MSVC → GNU when appropriate on this host.
EFFECTIVE=()
for t in "${TARGETS[@]}"; do
  if [[ "$t" == "x86_64-pc-windows-msvc" ]]; then
    EFFECTIVE+=("$(resolve_windows_target)")
  else
    EFFECTIVE+=("$t")
  fi
done

# De-dupe while preserving order
DECLARED=()
for t in "${EFFECTIVE[@]}"; do
  skip_dup=0
  for d in "${DECLARED[@]+"${DECLARED[@]}"}"; do
    [[ "$d" == "$t" ]] && skip_dup=1 && break
  done
  [[ "$skip_dup" -eq 1 ]] || DECLARED+=("$t")
done

ok=0
failed=0
skipped=0
declare -a SUMMARY=()

for target in "${DECLARED[@]}"; do
  set +e
  build_one "$target"
  rc=$?
  set -e
  case "$rc" in
    0) ok=$((ok + 1)); SUMMARY+=("OK      ${target}") ;;
    2) skipped=$((skipped + 1)); SUMMARY+=("SKIP    ${target}") ;;
    *) failed=$((failed + 1)); SUMMARY+=("FAIL    ${target}") ;;
  esac
done

echo ""
echo "=== release multi-target summary ==="
printf '%s\n' "${SUMMARY[@]}"
echo "ok=${ok} failed=${failed} skipped=${skipped} dist=${DIST_DIR}"

if [[ "$failed" -gt 0 ]]; then
  exit 1
fi
if [[ "$STRICT" == "1" && "$skipped" -gt 0 ]]; then
  echo "STRICT=1: skipped targets count as failure" >&2
  exit 1
fi
exit 0
