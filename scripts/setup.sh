#!/usr/bin/env bash
set -euo pipefail

###############################################################################
# scripts/setup.sh — Deploy all BDCX contracts to local Soroban sandbox
#
# Prerequisites:
#   - cargo (Rust toolchain)
#   - soroban CLI (install: cargo install soroban-cli --features opt)
#   - local sandbox running (soroban network start)
#
# Usage:
#   ./scripts/setup.sh [--no-build]
###############################################################################

echo "==> BDCX — Local Sandbox Setup"
echo ""

# ── 1. Check prerequisites ──────────────────────────────────────────────────
echo "==> Checking prerequisites..."
command -v cargo >/dev/null 2>&1 || { echo "ERROR: cargo not found"; exit 1; }
command -v soroban >/dev/null 2>&1 || { echo "ERROR: soroban CLI not found. Install: cargo install soroban-cli --features opt"; exit 1; }
echo "    cargo:   $(cargo --version | head -1)"
echo "    soroban: $(soroban --version 2>/dev/null || echo 'found')"

# ── 2. Build contracts ─────────────────────────────────────────────────────
if [[ "${1:-}" != "--no-build" ]]; then
    echo ""
    echo "==> Building all contracts (release)..."
    cargo build --release 2>&1 | sed 's/^/    /'
    echo "    Build complete."
else
    echo ""
    echo "==> Skipping build (--no-build flag detected)"
fi

# ── 3. Configure identity / source account ─────────────────────────────────
ADMIN="${ADMIN:-admin}"
echo ""
echo "==> Using identity: $ADMIN"
soroban keys address "$ADMIN" >/dev/null 2>&1 || {
    echo "    Creating new identity '$ADMIN'..."
    soroban keys generate "$ADMIN"
}
ADMIN_ADDR=$(soroban keys address "$ADMIN")
echo "    Admin address: $ADMIN_ADDR"

# ── 4. Deploy contracts ────────────────────────────────────────────────────
echo ""
echo "==> Deploying contracts to local sandbox..."

deploy() {
    local name="$1" wasm="$2"
    echo "    Deploying $name..."
    soroban contract deploy \
        --wasm "$wasm" \
        --source "$ADMIN" \
        --network local 2>&1 | sed 's/^/      /'
}

BDC_TOKEN_ID=$(deploy "bdc-token"   "target/wasm32-unknown-unknown/release/bdc_token.wasm")
MRV_ORACLE_ID=$(deploy "mrv-oracle" "target/wasm32-unknown-unknown/release/mrv_oracle.wasm")
APPROVAL_GOV_ID=$(deploy "approval-gov" "target/wasm32-unknown-unknown/release/approval_gov.wasm")
RETIREMENT_ID=$(deploy "retirement"  "target/wasm32-unknown-unknown/release/retirement.wasm")
MARKETPLACE_ID=$(deploy "marketplace" "target/wasm32-unknown-unknown/release/marketplace.wasm")

echo ""
echo "    ┌──────────────────────┬──────────────────────────────────────┐"
echo "    │ Contract             │ Contract ID                          │"
echo "    ├──────────────────────┼──────────────────────────────────────┤"
printf "    │ %-20s │ %-36s │\n" "bdc-token"    "$BDC_TOKEN_ID"
printf "    │ %-20s │ %-36s │\n" "mrv-oracle"   "$MRV_ORACLE_ID"
printf "    │ %-20s │ %-36s │\n" "approval-gov" "$APPROVAL_GOV_ID"
printf "    │ %-20s │ %-36s │\n" "retirement"   "$RETIREMENT_ID"
printf "    │ %-20s │ %-36s │\n" "marketplace"  "$MARKETPLACE_ID"
echo "    └──────────────────────┴──────────────────────────────────────┘"

# ── 5. Initialize contracts ────────────────────────────────────────────────
echo ""
echo "==> Initializing contracts..."

invoke() {
    local id="$1" fn="$2"
    shift 2
    soroban contract invoke \
        --id "$id" \
        --fn "$fn" \
        --source "$ADMIN" \
        --network local \
        "$@" 2>&1 | sed 's/^/      /'
}

echo "    Initializing bdc-token..."
invoke "$BDC_TOKEN_ID" initialize --arg "$ADMIN_ADDR"

echo "    Initializing mrv-oracle..."
invoke "$MRV_ORACLE_ID" initialize --arg "$ADMIN_ADDR"
invoke "$MRV_ORACLE_ID" set_bdc_token --arg "$BDC_TOKEN_ID"

echo "    Initializing approval-gov..."
invoke "$APPROVAL_GOV_ID" initialize --arg "$ADMIN_ADDR" --arg '{"u32":6}' --arg "$(printf '%llu' $((7*24*60*60)))"

echo "    Linking approval-gov <-> mrv-oracle..."
invoke "$APPROVAL_GOV_ID" set_bdc_token --arg "$BDC_TOKEN_ID"
invoke "$APPROVAL_GOV_ID" set_mrv_oracle --arg "$MRV_ORACLE_ID"
invoke "$MRV_ORACLE_ID" set_approval_gov --arg "$APPROVAL_GOV_ID"

echo "    Authorizing approval-gov as minter on bdc-token..."
invoke "$BDC_TOKEN_ID" authorize_minter --arg "$APPROVAL_GOV_ID"

echo "    Initializing retirement..."
invoke "$RETIREMENT_ID" initialize --arg "$ADMIN_ADDR"
invoke "$RETIREMENT_ID" set_bdc_token --arg "$BDC_TOKEN_ID"

# ── 6. Print summary ───────────────────────────────────────────────────────
echo ""
echo "==> Setup complete!"
echo ""
echo "    Export these for use with other scripts:"
echo ""
echo "    export BDC_TOKEN_ID=$BDC_TOKEN_ID"
echo "    export MRV_ORACLE_ID=$MRV_ORACLE_ID"
echo "    export APPROVAL_GOV_ID=$APPROVAL_GOV_ID"
echo "    export RETIREMENT_ID=$RETIREMENT_ID"
echo "    export MARKETPLACE_ID=$MARKETPLACE_ID"
echo "    export ADMIN=$ADMIN_ADDR"
echo ""

# Optionally write to .env
if [[ -n "${WRITE_ENV:-}" ]]; then
    cat > .env <<EOF
BDC_TOKEN_ID=$BDC_TOKEN_ID
MRV_ORACLE_ID=$MRV_ORACLE_ID
APPROVAL_GOV_ID=$APPROVAL_GOV_ID
RETIREMENT_ID=$RETIREMENT_ID
MARKETPLACE_ID=$MARKETPLACE_ID
ADMIN=$ADMIN_ADDR
NETWORK=local
EOF
    echo "    Written .env file."
fi
