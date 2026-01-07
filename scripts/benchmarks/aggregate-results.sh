#!/usr/bin/env bash
set -euo pipefail

# Aggregate benchmark results from multiple directories
# Usage: ./aggregate-results.sh <input-dirs-csv> <output-dir>

INPUT_DIRS="${1:-benchmark-artifacts/*}"
OUTPUT_DIR="${2:-consolidated-output}"

# Validate input parameters
if [[ -z "${INPUT_DIRS}" ]]; then
  echo "error: INPUT_DIRS is empty or not provided" >&2
  echo "usage: $0 <input-dirs-csv> [output-dir]" >&2
  exit 1
fi

if [[ -z "${OUTPUT_DIR}" ]]; then
  echo "error: OUTPUT_DIR is empty or not provided" >&2
  exit 1
fi

# Validate output directory's parent exists and is writable
output_parent="$(dirname "${OUTPUT_DIR}")"
if [[ ! -d "${output_parent}" ]]; then
  echo "error: parent directory does not exist: ${output_parent}" >&2
  exit 1
fi

if [[ ! -w "${output_parent}" ]]; then
  echo "error: parent directory is not writable: ${output_parent}" >&2
  exit 1
fi

# Create output directory if it doesn't exist
if [[ ! -d "${OUTPUT_DIR}" ]]; then
  mkdir -p "${OUTPUT_DIR}" || {
    echo "error: failed to create output directory: ${OUTPUT_DIR}" >&2
    exit 1
  }
fi

echo "Aggregating benchmark results..."
echo "Input: ${INPUT_DIRS}"
echo "Output: ${OUTPUT_DIR}"

if ! cargo run --release --package benchmark-harness -- consolidate \
  --inputs "${INPUT_DIRS}" \
  --output "${OUTPUT_DIR}"; then
  echo "error: cargo consolidation command failed" >&2
  exit 1
fi

# Validate that output files were created
consolidated_file="${OUTPUT_DIR}/consolidated.json"
aggregated_file="${OUTPUT_DIR}/aggregated.json"

if [[ ! -f "${consolidated_file}" ]]; then
  echo "error: expected output file not created: ${consolidated_file}" >&2
  exit 1
fi

if [[ ! -f "${aggregated_file}" ]]; then
  echo "error: expected output file not created: ${aggregated_file}" >&2
  exit 1
fi

echo "Aggregation complete"
echo "  Consolidated: ${consolidated_file}"
echo "  Aggregated: ${aggregated_file}"
