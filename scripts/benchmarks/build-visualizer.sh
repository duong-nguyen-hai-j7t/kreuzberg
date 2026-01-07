#!/usr/bin/env bash
set -euo pipefail

# Build the benchmark visualizer for deployment

# ============================================================================
# PRE-FLIGHT CHECKS
# ============================================================================

# Check for required commands
check_command() {
  local cmd="$1"
  if ! command -v "$cmd" &>/dev/null; then
    echo "ERROR: Required command '$cmd' not found in PATH" >&2
    echo "Please install $cmd and ensure it's available in your PATH" >&2
    exit 1
  fi
}

check_command "pnpm"
check_command "node"

# ============================================================================
# CALCULATE REPO ROOT AND NAVIGATE
# ============================================================================

# Get the absolute path to the directory containing this script
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Navigate up to repo root (script is in scripts/benchmarks/)
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

# Navigate to the visualizer directory
VISUALIZER_DIR="$REPO_ROOT/tools/benchmark-visualizer"

if [[ ! -d "$VISUALIZER_DIR" ]]; then
  echo "ERROR: Visualizer directory not found at: $VISUALIZER_DIR" >&2
  exit 1
fi

cd "$VISUALIZER_DIR"

echo "Building in: $VISUALIZER_DIR"

# ============================================================================
# BUILD PROCESS
# ============================================================================

echo "Installing dependencies..."
if ! pnpm install --frozen-lockfile; then
  echo "ERROR: Failed to install dependencies" >&2
  exit 1
fi

echo "Building visualizer..."
if ! pnpm run build; then
  echo "ERROR: Build process failed" >&2
  exit 1
fi

# ============================================================================
# POST-BUILD VALIDATION
# ============================================================================

DIST_DIR="$VISUALIZER_DIR/dist"
DIST_INDEX="$DIST_DIR/index.html"

if [[ ! -f "$DIST_INDEX" ]]; then
  echo "ERROR: Post-build validation failed - dist/index.html not found at: $DIST_INDEX" >&2
  exit 1
fi

echo "Validating build artifacts..."
if [[ ! -d "$DIST_DIR" ]]; then
  echo "ERROR: dist directory not found at: $DIST_DIR" >&2
  exit 1
fi

# Count build artifacts
ARTIFACT_COUNT=$(find "$DIST_DIR" -type f | wc -l)
if [[ $ARTIFACT_COUNT -lt 1 ]]; then
  echo "ERROR: No build artifacts found in $DIST_DIR" >&2
  exit 1
fi

echo "Build validation successful: found $ARTIFACT_COUNT artifacts"
echo "Build complete: $DIST_DIR/"
