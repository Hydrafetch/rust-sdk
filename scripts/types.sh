#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
SPEC="${HYDRAFETCH_SPEC:-https://api.hydrafetch.com/openapi.json}"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
if [[ "$SPEC" == http* ]]; then curl -fsS "$SPEC" > "$tmp/spec.json"; else cp "$SPEC" "$tmp/spec.json"; fi
python3 scripts/spec_to_schema.py < "$tmp/spec.json" > "$tmp/schema.json"
cargo typify "$tmp/schema.json" --no-builder -o "$tmp/generated.rs"
python3 scripts/check_options.py "$tmp/spec.json"
if [ "${1:-}" = check ]; then
  if ! cmp -s "$tmp/generated.rs" src/generated.rs; then
    echo "src/generated.rs no longer matches $SPEC. Run scripts/types.sh, review the change, and release." >&2
    exit 1
  fi
  echo "Types match the published API."
  exit 0
fi
cp "$tmp/generated.rs" src/generated.rs
python3 -c 'import hashlib,json,sys; print(hashlib.sha256(json.dumps(json.load(open(sys.argv[1])),separators=(",",":"),ensure_ascii=False).encode()).hexdigest())' "$tmp/spec.json" > openapi.sha256
echo "Wrote src/generated.rs and openapi.sha256 from $SPEC."
