#!/bin/bash
set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Set library paths for the native extension
export DYLD_LIBRARY_PATH="$REPO_ROOT/target/release:${DYLD_LIBRARY_PATH:-}"
export LD_LIBRARY_PATH="$REPO_ROOT/target/release:${LD_LIBRARY_PATH:-}"

echo "Running Kreuzberg R test suite..."
echo "Repository root: $REPO_ROOT"
echo "Script directory: $SCRIPT_DIR"
echo ""

cd "$SCRIPT_DIR"
Rscript main_test.R
