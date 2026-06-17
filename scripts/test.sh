#!/usr/bin/env bash
set -euo pipefail

###############################################################################
# scripts/test.sh — Run all BDCX tests, lints, and formatting checks
#
# Usage:
#   ./scripts/test.sh                    # full suite
#   ./scripts/test.sh --unit             # unit tests only
#   ./scripts/test.sh --integration      # integration tests only
#   ./scripts/test.sh --lint             # clippy + fmt checks only
#   ./scripts/test.sh --doc              # doc build only
#   ./scripts/test.sh --quick            # unit + clippy (no doc)
###############################################################################

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

RUN_UNIT=false
RUN_INTEGRATION=false
RUN_LINT=false
RUN_DOC=false

case "${1:-all}" in
    all)
        RUN_UNIT=true
        RUN_INTEGRATION=true
        RUN_LINT=true
        RUN_DOC=true
        ;;
    --unit)
        RUN_UNIT=true
        ;;
    --integration)
        RUN_INTEGRATION=true
        ;;
    --lint)
        RUN_LINT=true
        ;;
    --doc)
        RUN_DOC=true
        ;;
    --quick)
        RUN_UNIT=true
        RUN_LINT=true
        ;;
    *)
        echo "Usage: $0 [--unit|--integration|--lint|--doc|--quick|all]"
        exit 1
        ;;
esac

EXIT_CODE=0

step() {
    local label="$1" cmd="$2"
    echo ""
    echo "═══════════════════════════════════════════════════════════════"
    echo "  $label"
    echo "═══════════════════════════════════════════════════════════════"
    if ! eval "$cmd"; then
        echo "  ✗ FAILED: $label"
        EXIT_CODE=1
    else
        echo "  ✓ PASSED: $label"
    fi
}

if $RUN_UNIT; then
    step "Unit Tests" "cargo test --all-features"
fi

if $RUN_INTEGRATION; then
    step "Integration Tests" "cargo test -p integration-tests"
fi

if $RUN_LINT; then
    step "Clippy"   "cargo clippy --all-targets -- -D warnings"
    step "Format"   "cargo fmt --check"
fi

if $RUN_DOC; then
    step "Documentation" "cargo doc --no-deps --document-private-items"
fi

echo ""
echo "═══════════════════════════════════════════════════════════════"
if [ "$EXIT_CODE" -eq 0 ]; then
    echo "  All checks passed."
else
    echo "  Some checks failed (exit code $EXIT_CODE)."
fi
echo "═══════════════════════════════════════════════════════════════"
exit "$EXIT_CODE"
