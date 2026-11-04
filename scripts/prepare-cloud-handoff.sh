#!/usr/bin/env bash
set -euo pipefail
repo_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd -- "$repo_dir"
output_dir="$(mktemp -d /tmp/vaultling-cloud-XXXXXXXX)"
git diff --binary HEAD -- src/bin/vaultling-gui.rs src/lib.rs > "$output_dir/local-changes.patch"
mkdir -p "$output_dir/src/gui"
cp -- src/gui/mod.rs src/gui/layout.rs src/gui/design_system.rs "$output_dir/src/gui/"
cp -- IMPROVEMENT_PLAN.md CLOUD_CONTINUE.md "$output_dir/"
git rev-parse HEAD > "$output_dir/BASE_COMMIT.txt"
tar -czf "$output_dir.tar.gz" -C "$output_dir" .
printf 'Arhivă pentru cloud: %s.tar.gz\nInstrucțiuni: CLOUD_CONTINUE.md din arhivă\n' "$output_dir"
