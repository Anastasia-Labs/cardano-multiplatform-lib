#!/usr/bin/env bash
#
# The clippy gate — the single definition of it. CI invokes this script and passes no lint
# flags of its own. If you find yourself writing `--allow clippy::…` in the workflow, it
# belongs here instead. (Pattern adopted from upstream dcSpark's clippy.sh.)
#
# The toolchain is pinned in rust-toolchain.toml, so this list is measured against exactly
# one clippy version and cannot rot when stable moves.
#
# Two groups below:
#  1. Deliberate architecture (kept in upstream's gate too) — not debt.
#  2. Pre-existing debt in the 6.2.0-era tree, measured on toolchain 1.96.1 (2026-08-05).
#     Upstream fixed these in their cddl-codegen emitter after this fork's tree diverged;
#     they disappear on the next upstream sync/regen. Do not add new code that trips them —
#     the allow keeps the OLD sites building, it is not a license for new ones.
#
# Usage:  ./clippy.sh                # the gate, exactly as CI runs it
#         ./clippy.sh -p cml-chain   # extra cargo-clippy args are forwarded
set -euo pipefail
cd "$(dirname "${BASH_SOURCE[0]}")"

LINTS=(
  --deny clippy::all

  # --- group 1: deliberate architecture (same allows as upstream dcSpark) ---
  # Large error enums returned by ~35 builder functions; boxing them changes every
  # builder signature.
  --allow clippy::result_large_err
  # Hand-written era enums with inherently uneven variants; boxing changes public types.
  --allow clippy::large_enum_variant
  # Cosmetic: blank line after doc comments at a few sites.
  --allow clippy::empty_line_after_doc_comments

  # --- group 2: pre-existing at 6.2.0, fixed upstream post-divergence; drop on next sync ---
  --allow clippy::useless_conversion
  --allow clippy::derivable_impls
  --allow clippy::manual_repeat_n
  --allow clippy::double_ended_iterator_last
  --allow clippy::single_match
  --allow clippy::collapsible_match
  --allow clippy::clone_on_copy
  --allow clippy::unnecessary_unwrap
)

exec cargo clippy --workspace --all-features --all-targets "$@" -- "${LINTS[@]}"
