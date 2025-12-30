#!/usr/bin/env bash
set -euo pipefail

pkg_dir="crates/kreuzberg-wasm/pkg"

echo "Checking for generated type definitions..."

pkg_json="$pkg_dir/package.json"
if [ ! -f "$pkg_json" ]; then
  echo "ERROR: WASM package.json not found: $pkg_json"
  exit 1
fi

types_file="$(python3 -c 'import json; import sys; print(json.load(open(sys.argv[1]))["types"])' "$pkg_json")"
if [ -z "$types_file" ]; then
  echo "ERROR: Missing \"types\" field in $pkg_json"
  exit 1
fi

required_files=("$pkg_dir/$types_file")

for file in "${required_files[@]}"; do
  if [ ! -f "$file" ]; then
    echo "ERROR: Type definition file not found: $file"
    exit 1
  fi
  echo "Found: $file"
done

echo ""
echo "Type definitions validation passed!"
