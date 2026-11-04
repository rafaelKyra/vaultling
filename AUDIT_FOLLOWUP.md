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
| 8 | At the 760 px minimum width the centre list collapsed to about 150 px because the side panes kept the width remembered from a larger window. | `src/gui/layout.rs` | Medium (UI unusable when narrow) | In a window too small for the configured widths the side panes are pinned to the shrunk widths from `pane_widths`; the sidebar floor is 175 px. | Unit tests plus a real run resized to 760x600: list and detail both readable. |
| 9 | Scroll areas inside forms (trash, history) were squeezed to about one row because `form()` laid its card out in a horizontal layout. | `form()` in `vaultling-gui.rs` | Medium (trash list unusable) | `form()` places the card in an explicit rectangle. | Real run: trash showed all entries; Restore moved one back ("Entry restored"). |
| 10 | CSV export then import silently lost data: 36 entries exported, 30 imported with no mention of the 6 skipped (entries without a password), and `export.rs` claimed a re-import "without loss" although the category and category-specific fields are not in the CSV. Re-importing into the same vault also duplicated entries. | `src/import.rs` (`parse`), `src/export.rs` doc comment, `import_csv` in `vaultling-gui.rs` | Medium (data loss unnoticed) | New `parse_counting` and `without_duplicates` (same name ignoring case, username and password); the status line now says "N skipped (no password), M already in the vault". The decrypted comparison copies are zeroized after use. The export doc comment states the real limits and points to the encrypted backup. | Tests `counts_the_rows_it_skips` and `duplicates_of_the_vault_and_of_the_file_are_dropped`; real run re-importing the export: "Imported 0 entries from Chrome, 6 skipped (no password), 30 already in the vault", vault stays at 36. |
| 11 | KeePass import was untested and lossy: it dropped notes, ignored folders, had no duplicate detection and reported only counts. | `import_kdbx` in `vaultling-gui.rs` | Low-Medium | Parsing moved to `import::parse_kdbx` (title, user, password, URL, notes, TOTP from `otp`, enclosing sub-folder name as tag); duplicates skipped; skipped/duplicate counts shown; wrong password and bad file give the same vague message. | Tests `reads_a_synthetic_kdbx`, `a_wrong_kdbx_password_gives_the_vague_error` against `tests/fixtures/synthetic.kdbx` (synthetic, built with pykeepass; the password `kdbx-test-pass` is in the tests); real run: "Imported 4 entries, 1 skipped (no title or password)", 36 to 40 entries. |
| 12 | The CSV export had no category column, so every import landed under "Website" and entries without a password were dropped. | `src/export.rs`, `src/import.rs` | Medium (round trip lost the category) | Export adds a `kind` column; import maps it with `Kind::from_label`, recognises the file as "Vaultling" and keeps passwordless entries when the column is present. Category-specific fields (card number, host...) are still not in the CSV. | Tests `the_category_survives_a_round_trip_and_passwordless_entries_are_kept`, `vaultling_export_round_trips_the_category`. Not driven in the GUI. |
| 5 | `/`, Ctrl+C and the new shortcuts fired while typing in a text field (Ctrl+C overwrote the user's selection with the entry password). | `update`, keyboard block | Medium (secret to clipboard unintentionally) | Shortcuts are ignored when `wants_keyboard_input()`. | Code path only; no automated GUI input test. |

Reviewed, no change needed: clipboard overwrite after 20 s runs independently
of lock state; idle lock (5 min) and lock on minimise are present; unlock and
create clear passphrase buffers on use; the KDF and crypto were not touched.

## Verification run

- `cargo check --locked --all-targets`: OK (one pre-existing `unused_mut` warning in `examples/seed_demo.rs`).
- `cargo test --locked --all-targets`: 131 lib tests + 5 gui tests pass, 0 failed.
- `cargo clippy --locked --all-targets`: warnings remain, all pre-existing and outside the changed code (main.rs, storage, totp, kind, key_lifecycle, import, circuit_breaker, and three in `vaultling-gui.rs` lines not touched here). No warnings in the new code.
- Visual check: done under Xvfb after installing `libxkbcommon-x11-0` in the sandbox (environment blocker, not a code fault). Only the unlocked browse view in two themes at 980x820. Minimum size (620x620), keyboard flows and mouse interaction were not exercised.

## Implemented from the plan in this branch

Responsive pane widths, four sort orders, keyboard navigation and a `?`
overlay, multi-select with bulk favourite and trash, statistics panel,
high-contrast palettes (WCAG AAA checked), opt-in two-factor codes in the list,
a three-step first-run walkthrough, a reduce-motion setting, accessible names
for the custom-drawn controls (checked against the AccessKit tree in unit
tests), KeePass import with notes, folders as tags and duplicate detection,
and CSV import/export that keeps the category.

## Verification

- `cargo check --locked --all-targets`: OK (one pre-existing `unused_mut` warning in `examples/seed_demo.rs`).
- `cargo test --locked --all-targets`: 132 lib tests + 8 gui tests pass, 0 failed.
- `cargo clippy --locked --all-targets`: warnings remain, all pre-existing and outside the changed code, except none new from this branch (checked by listing the files and lines clippy reports).
- Real interaction under Xvfb (xdotool, synthetic vaults): unlock, arrow navigation, `?` overlay, Ctrl+L, Ctrl+C and the 20 s clipboard overwrite, typing guard in the search box, Select mode with bulk favourite/trash, trash list and Restore, editor with Generate and Add, 760x600 window, CSV export (mode 0600) and import with duplicates, KeePass import (wrong password and correct password), first-run walkthrough steps 1 to 3.
- Not exercised: a real screen reader (only the AccessKit tree in unit tests), release-build timing, Wayland, the walkthrough's "Add an entry" button, CSV round trip into an empty vault through the GUI.

## Remaining

- CSV carries the category but not category-specific fields (card number, host...); the encrypted backup is the lossless copy.
- KDBX import reads and decrypts on the UI thread; in a debug build the Argon2 step froze the window for several seconds. Release timing not measured.
- Not implemented: import preview/column mapping, custom-fields editor beyond the existing category fields, related entries, skeleton/loading states.
- Pre-existing clippy warnings remain in other modules.
