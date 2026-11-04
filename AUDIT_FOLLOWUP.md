# Audit follow-up

Base: `codex/continue-gui-audit` (9e21691). No separate OpenCode report was
available, so findings come from `CLOUD_CONTINUE.md` and a read of the changed
code. Synthetic data only.

## Findings

| # | Finding | Evidence | Severity | Remedy | Verification |
|---|---|---|---|---|---|
| 1 | `ThreePaneLayout::show` took `(sidebar, detail, center)` while `show_three_pane` passed `(sidebar, center, detail)`, so the center closure rendered in the right pane and the detail closure in the center. | `src/gui/layout.rs` (`show`, `show_three_pane`) | Medium (functional/UI) | `show` now takes `(sidebar, center, detail)`. | Tests `closures_render_in_their_own_slots` and `every_pane_runs_once_at_small_and_large_sizes`; the first fails when the old swap is reintroduced. Screenshots (Xvfb, dark and light, 980x820) show list in the center and detail on the right. |
| 2 | `lock()` left decrypted data in memory: password history rows (old passwords in clear), the trash list, and `new_pass`/`new_pass_again` and path fields; entry rows were only `clear()`ed, not overwritten. | `src/bin/vaultling-gui.rs`, `lock`; `history_rows` filled from `password_history` | Medium (secret lifetime) | `lock()` calls `wipe_secrets` (zeroizes row and history text before drop) and clears trash, tag filter, new-passphrase and path fields. | Unit test `wipe_secrets_empties_rows_and_history`. This is best effort: copies in egui text buffers and the allocator are not reachable (already stated in the file header). |

Reviewed, no change needed: clipboard overwrite after 20 s runs independently
of lock state; idle lock (5 min) and lock on minimise are present; unlock and
create clear passphrase buffers on use; the KDF and crypto were not touched.

## Verification run

- `cargo check --locked --all-targets`: OK (one pre-existing `unused_mut` warning in `examples/seed_demo.rs`).
- `cargo test --locked --all-targets`: 121 lib tests + 1 gui test pass, 0 failed.
- `cargo clippy --locked --all-targets`: warnings remain, all pre-existing and outside the changed code (main.rs, storage, totp, kind, key_lifecycle, import, circuit_breaker, and three in `vaultling-gui.rs` lines not touched here). No warnings in the new code.
- Visual check: done under Xvfb after installing `libxkbcommon-x11-0` in the sandbox (environment blocker, not a code fault). Only the unlocked browse view in two themes at 980x820. Minimum size (620x620), keyboard flows and mouse interaction were not exercised.

## Remaining

- The center list card is clipped at its right edge at 980 px width (visible in both screenshots). Not fixed here.
- No responsive collapse of panes at small widths; Phase 2 keyboard navigation, bulk actions, stats, onboarding and high contrast mode are unimplemented (see status table in `IMPROVEMENT_PLAN.md`).
- Ctrl+C copies the open entry's password even when a text field is focused.
