# Audit follow-up

Base: `codex/continue-gui-audit` (9e21691). No separate OpenCode report was
available, so findings come from `CLOUD_CONTINUE.md` and a read of the changed
code. Synthetic data only.

## Findings

| # | Finding | Evidence | Severity | Remedy | Verification |
|---|---|---|---|---|---|
| 1 | `ThreePaneLayout::show` took `(sidebar, detail, center)` while `show_three_pane` passed `(sidebar, center, detail)`, so the center closure rendered in the right pane and the detail closure in the center. | `src/gui/layout.rs` (`show`, `show_three_pane`) | Medium (functional/UI) | `show` now takes `(sidebar, center, detail)`. | Tests `closures_render_in_their_own_slots` and `every_pane_runs_once_at_small_and_large_sizes`; the first fails when the old swap is reintroduced. Screenshots (Xvfb, dark and light, 980x820) show list in the center and detail on the right. |
| 2 | `lock()` left decrypted data in memory: password history rows (old passwords in clear), the trash list, and `new_pass`/`new_pass_again` and path fields; entry rows were only `clear()`ed, not overwritten. | `src/bin/vaultling-gui.rs`, `lock`; `history_rows` filled from `password_history` | Medium (secret lifetime) | `lock()` calls `wipe_secrets` (zeroizes row and history text before drop) and clears trash, tag filter, new-passphrase and path fields. | Unit test `wipe_secrets_empties_rows_and_history`. This is best effort: copies in egui text buffers and the allocator are not reachable (already stated in the file header). |

| 3 | Only the browsing list was drawn: when `browsing` was false (lock screen, vault creation, editor, import/export, settings, trash, help) no central panel was shown, so those screens rendered blank. | `src/bin/vaultling-gui.rs`, `update`, `if browsing { show_three_pane(..) }` had no `else` | High (app unusable outside the list) | Added the `else` branch with a full-width `CentralPanel`. | Screenshots of lock screen, editor and statistics panels under Xvfb. |
| 4 | List card overflowed the centre pane (clipped at its right edge). | list row labels were untruncated | Low (UI) | Row labels truncate, row fills the available width. | Screenshot, dark and high-contrast. |
| 6 | The Ctrl+C shortcut never worked: egui-winit converts it to `Event::Copy` and emits no key press, so the handler waiting for `Key::C` never fired. | `egui-winit-0.29/src/lib.rs:763`; GUI keyboard block | Medium (documented feature broken) | Handler also accepts `Event::Copy`. | Real run under Xvfb: clipboard held the entry's synthetic password after Ctrl+C, and a single space 20 s later. |
| 7 | Forms (editor, settings, import, stats...) did not scroll, so the Save/Add button of the editor was out of reach in a normal-height window. | `form()` in `vaultling-gui.rs` | High (cannot add entries) | `form()` wraps its content in a vertical `ScrollArea`. | Real run: filled the editor, scrolled with the wheel, clicked Add; entry count went 36 to 37. |
| 5 | `/`, Ctrl+C and the new shortcuts fired while typing in a text field (Ctrl+C overwrote the user's selection with the entry password). | `update`, keyboard block | Medium (secret to clipboard unintentionally) | Shortcuts are ignored when `wants_keyboard_input()`. | Code path only; no automated GUI input test. |

Reviewed, no change needed: clipboard overwrite after 20 s runs independently
of lock state; idle lock (5 min) and lock on minimise are present; unlock and
create clear passphrase buffers on use; the KDF and crypto were not touched.

## Verification run

- `cargo check --locked --all-targets`: OK (one pre-existing `unused_mut` warning in `examples/seed_demo.rs`).
- `cargo test --locked --all-targets`: 125 lib tests + 4 gui tests pass, 0 failed.
- `cargo clippy --locked --all-targets`: warnings remain, all pre-existing and outside the changed code (main.rs, storage, totp, kind, key_lifecycle, import, circuit_breaker, and three in `vaultling-gui.rs` lines not touched here). No warnings in the new code.
- Visual check: done under Xvfb after installing `libxkbcommon-x11-0` in the sandbox (environment blocker, not a code fault). Only the unlocked browse view in two themes at 980x820. Minimum size (620x620), keyboard flows and mouse interaction were not exercised.

## Implemented from the plan in this branch

Responsive pane widths (`pane_widths`, window minimum raised to 760x600), list
sort by name/newest/edited/category, keyboard navigation (arrows, j/k) and a
`?` shortcuts overlay, multi-select with bulk favourite and move-to-trash
(recoverable), vault statistics panel, high-contrast palettes (WCAG AAA
checked by unit test) with a Settings toggle.

## Remaining

- Interaction was driven for real under Xvfb with xdotool (synthetic vault): unlock by typing, arrow navigation, `?` overlay, Ctrl+L lock, Ctrl+C copy and 20 s clipboard overwrite, typing `a/b?c` in the search box without triggering shortcuts, Select mode with bulk move to trash (36 to 34), editor with Generate and Add (36 to 37). Not driven: the 760 px minimum width, import/export flows, favourites in bulk, restore from trash.
- Added: a three-step first-run hint in the empty-vault state (not a wizard) and an opt-in Settings switch to show live two-factor codes in list rows (off by default, reset on lock).
- Not implemented: full onboarding wizard, import preview/mapping, custom-fields editor beyond the existing category fields, related entries, skeleton/loading states, reduced-motion switch, screen-reader labels (limited by the egui version in use).
- Pre-existing clippy warnings remain in other modules.
