#!/usr/bin/env bash
set -euo pipefail

###############################################################################
# scripts/deploy.sh — Deploy all BDCX contracts to testnet or mainnet
#
# Prerequisites:
#   - cargo (Rust toolchain)
#   - soroban CLI (install: cargo install soroban-cli --features opt)
#   - funded identity on target network
#
# Usage:
#   ./scripts/deploy.sh [testnet|mainnet] [identity]
#     network   (default: testnet)
#     identity  (default: admin)
#
# Examples:
#   ./scripts/deploy.sh                     # testnet, admin identity
#   ./scripts/deploy.sh mainnet             # mainnet, admin identity
#   ./scripts/deploy.sh testnet deployer    # testnet, deployer identity
###############################################################################

NETWORK="${1:-testnet}"
ADMIN="${2:-admin}"
RPC_URL=""

case "$NETWORK" in
    testnet)
        RPC_URL="https://soroban-testnet.stellar.org:443"
        NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
        ;;
    mainnet)
        RPC_URL="https://soroban.stellar.org:443"
        NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
        ;;
    local)
        RPC_URL="http://localhost:8000/soroban/rpc"
        NETWORK_PASSPHRASE="Standalone Network ; February 2017"
        ;;
    *)
        echo "ERROR: Unknown network '$NETWORK'. Use: testnet, mainnet, or local"
        exit 1
        ;;
esac

echo "==> BDCX — Deploy to $NETWORK"
echo "    Identity:  $ADMIN"
echo "    RPC URL:   $RPC_URL"
echo ""

# ── 1. Check prerequisites ──────────────────────────────────────────────────
command -v cargo >/dev/null 2>&1 || { echo "ERROR: cargo not found"; exit 1; }
command -v soroban >/dev/null 2>&1 || { echo "ERROR: soroban CLI not found"; exit 1; }

# ── 2. Add network if needed ────────────────────────────────────────────────
if ! soroban network ls 2>/dev/null | grep -q "$NETWORK"; then
    echo "==> Adding network '$NETWORK'..."
    soroban network add \
        --rpc-url "$RPC_URL" \
        --network-passphrase "$NETWORK_PASSPHRASE" \
        "$NETWORK"
fi

# ── 3. Verify identity exists and is funded ─────────────────────────────────
soroban keys address "$ADMIN" >/dev/null 2>&1 || {
    echo "ERROR: Identity '$ADMIN' not found. Generate with: soroban keys generate $ADMIN"
    exit 1
}
ADMIN_ADDR=$(soroban keys address "$ADMIN")
echo "    Admin address: $ADMIN_ADDR"

echo ""
echo "==> Checking balance..."
BALANCE=$(soroban keys balance "$ADMIN" --network "$NETWORK" 2>/dev/null || echo "0")
echo "    Balance: $BALANCE"

# ── 4. Build WASM ──────────────────────────────────────────────────────────
echo ""
echo "==> Building contracts (release)..."
cargo build --release 2>&1 | sed 's/^/    /'

# ── 5. Deploy & initialize ─────────────────────────────────────────────────
echo ""
echo "==> Deploying contracts..."

deploy() {
    local name="$1" wasm="$2"
    echo "    Deploying $name..."
    soroban contract deploy \
        --wasm "$wasm" \
        --source "$ADMIN" \
        --network "$NETWORK" 2>&1 | sed 's/^/      /'
}

invoke() {
    local id="$1" fn="$2"
    shift 2
    soroban contract invoke \
        --id "$id" \
        --fn "$fn" \
        --source "$ADMIN" \
        --network "$NETWORK" \
        "$@" 2>&1 | sed 's/^/      /'
}

BDC_TOKEN_ID=$(deploy "bdc-token"   "target/wasm32-unknown-unknown/release/bdc_token.wasm")
MRV_ORACLE_ID=$(deploy "mrv-oracle" "target/wasm32-unknown-unknown/release/mrv_oracle.wasm")
APPROVAL_GOV_ID=$(deploy "approval-gov" "target/wasm32-unknown-unknown/release/approval_gov.wasm")
RETIREMENT_ID=$(deploy "retirement"  "target/wasm32-unknown-unknown/release/retirement.wasm")
MARKETPLACE_ID=$(deploy "marketplace" "target/wasm32-unknown-unknown/release/marketplace.wasm")

echo ""
echo "    ┌──────────────────────┬──────────────────────────────────────┐"
echo "    │ Contract             │ Contract ID                          │"
echo "    ├──────────────────────┼──────────────────────────────────────┘"
printf "    │ %-20s │ %-36s\n" "bdc-token"    "$BDC_TOKEN_ID"
printf "    │ %-20s │ %-36s\n" "mrv-oracle"   "$MRV_ORACLE_ID"
printf "    │ %-20s │ %-36s\n" "approval-gov" "$APPROVAL_GOV_ID"
printf "    │ %-20s │ %-36s\n" "retirement"   "$RETIREMENT_ID"
printf "    │ %-20s │ %-36s\n" "marketplace"  "$MARKETPLACE_ID"
echo "    └──────────────────────┴──────────────────────────────────────"

echo ""
echo "==> Initializing contracts..."

invoke "$BDC_TOKEN_ID" initialize --arg "$ADMIN_ADDR"
invoke "$MRV_ORACLE_ID" initialize --arg "$ADMIN_ADDR"
invoke "$MRV_ORACLE_ID" set_bdc_token --arg "$BDC_TOKEN_ID"

invoke "$APPROVAL_GOV_ID" initialize --arg "$ADMIN_ADDR" --arg '{"u32":6}' --arg "$(printf '%llu' $((7*24*60*60)))"
invoke "$APPROVAL_GOV_ID" set_bdc_token --arg "$BDC_TOKEN_ID"
invoke "$APPROVAL_GOV_ID" set_mrv_oracle --arg "$MRV_ORACLE_ID"
invoke "$MRV_ORACLE_ID" set_approval_gov --arg "$APPROVAL_GOV_ID"

invoke "$BDC_TOKEN_ID" authorize_minter --arg "$APPROVAL_GOV_ID"

invoke "$RETIREMENT_ID" initialize --arg "$ADMIN_ADDR"
invoke "$RETIREMENT_ID" set_bdc_token --arg "$BDC_TOKEN_ID"

# Marketplace requires a yUSDC token address — set placeholder
USDC_PLACEHOLDER="CA3D5K7FVP4K7J4K7FVP4K7J4K7FVP4K7J4K7FVP4K7J4"
echo ""
echo "    NOTE: Marketplace init requires a yUSDC token address."
echo "    Update with:"
echo "      invoke $MARKETPLACE_ID initialize --arg \"$ADMIN_ADDR\""
echo "      invoke $MARKETPLACE_ID set_bdc_token --arg \"$BDC_TOKEN_ID\""
echo "      invoke $MARKETPLACE_ID set_usdc_token --arg \"<YUSDC_ID>\""
echo "      invoke $MARKETPLACE_ID set_fee_vault --arg \"<VAULT_ADDR>\""

# ── 6. Print export block ──────────────────────────────────────────────────
echo ""
echo "==> Deploy complete!"
echo ""
echo "    Export for use with other scripts:"
echo ""
echo "    export BDC_TOKEN_ID=$BDC_TOKEN_ID"
echo "    export MRV_ORACLE_ID=$MRV_ORACLE_ID"
echo "    export APPROVAL_GOV_ID=$APPROVAL_GOV_ID"
echo "    export RETIREMENT_ID=$RETIREMENT_ID"
echo "    export MARKETPLACE_ID=$MARKETPLACE_ID"
echo "    export ADMIN=$ADMIN_ADDR"
echo "    export NETWORK=$NETWORK"
echo ""

# Write .env for convenience
cat > .env <<EOF
BDC_TOKEN_ID=$BDC_TOKEN_ID
MRV_ORACLE_ID=$MRV_ORACLE_ID
APPROVAL_GOV_ID=$APPROVAL_GOV_ID
RETIREMENT_ID=$RETIREMENT_ID
MARKETPLACE_ID=$MARKETPLACE_ID
ADMIN=$ADMIN_ADDR
NETWORK=$NETWORK
EOF
echo "    Written .env file."
