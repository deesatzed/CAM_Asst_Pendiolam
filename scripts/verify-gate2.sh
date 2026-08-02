#!/bin/sh
set -eu

repo_root=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
cd "$repo_root"

if [ "$#" -gt 0 ] && [ "$1" = "--help" ]; then
  printf '%s\n' 'Run the deterministic local Gate 2 Packet Studio proof.'
  exit 0
fi

export CARGO_NET_OFFLINE=true
cargo run -p common-reality-studio --example verify_gate2
