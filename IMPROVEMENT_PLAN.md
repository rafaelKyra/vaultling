# Vaultling GUI Improvement Plan

## Overview
This plan outlines improvements to the Vaultling GUI focusing on:
- **Design & Graphics**: Modern visual design system, consistent theming, better icons
- **Usability & UX**: Intuitive navigation, better workflows, keyboard shortcuts
- **New Local Features**: Password generator, strength meter, categories, tags, import/export improvements
- **Accessibility**: Keyboard navigation, screen reader support, high contrast

## Current State Analysis

### Strengths
- Solid security architecture (mlock, zeroization, circuit breaker)
- Clean Rust codebase with good separation of concerns
- Functional CLI and GUI entry points
- Comprehensive test suite (119 tests passing)

### Areas for Improvement

#### 1. Visual Design System
- **Colors**: Hardcoded colors throughout, no consistent theme
- **Typography**: Default system fonts, no hierarchy
- **Spacing**: Inconsistent margins/padding
- **Icons**: Custom enum with unused variants, no visual icons rendered
- **Dark/Light mode**: Basic support but not polished

#### 2. Layout & Navigation
- Single-pane layout with tabs
- No persistent sidebar for categories/tags
- Header lacks clear actions
- Search/filter not prominent
- No responsive behavior

#### 3. Entry List
- Basic table-like display
- No visual distinction between entry types
- Limited sorting/filtering
- No inline actions (copy, edit, delete)
- TOTP codes not shown in list

#### 4. Entry Detail/Edit Forms
- Modal dialogs but cramped
- No password strength indicator
- No password generator
- Fields not grouped logically
- TOTP setup not user-friendly

#### 5. Missing Features (Local Only)
- Password generator with customizable options
- Password strength meter (visual)
- Categories/tags management UI
- Bulk operations (select multiple, delete, export)
- Keyboard shortcuts
- Onboarding/first-run experience
- Help/tooltips system
- Vault statistics dashboard

## Status (re-validated against the code, 2026-11-04)

Legend: done / partial / missing. Evidence is in `src/bin/vaultling-gui.rs` unless noted.

| Item | Status | Evidence |
|---|---|---|
| Design tokens, light/dark palette | done | `src/gui/design_system.rs`, `palette()`/`set_palette` |
| Hardcoded colours replaced | partial | `apply_theme` still sets some literal `Color32` values |
| Three-pane layout | done (fixed) | `src/gui/layout.rs`; slot-order bug fixed, tests added |
| Responsive breakpoints | done | `pane_widths` in `src/gui/layout.rs`, min window 760x600 |
| Keyboard shortcuts | done | arrows/j/k, `?` overlay, typing-safe guards |
| Password generator | done | `generate_password` |
| Strength meter | done | `strength`, `meter` |
| Entry types / categories (16 kinds) | done | `vaultling::kind` |
| Tags, favourites, sort | done | name, newest, edited, category |
| Multi-select / bulk actions | done | Select mode: favourite, move to trash |
| TOTP in detail with countdown | done | detail pane; optional in list rows (Settings, off by default) |
| Password history | done | `history_rows` |
| Stats dashboard, high contrast | done | `stats_view`, `HC_*` palettes |
| Onboarding wizard | partial | three-step hint in the empty-vault state; no wizard |

Implementation order: finish visual foundation and navigation (rows marked
partial in the first half of the table) before any new feature.

## Implementation Phases

### Phase 1: Design System Foundation (Week 1)
1. Create `design_system.rs` module with:
   - Color palette (light/dark themes)
   - Typography scale
   - Spacing system
   - Border radius, shadows
   - Animation durations
2. Replace all hardcoded colors with design tokens
3. Add proper icon rendering (using egui's built-in or custom SVG)

### Phase 2: Layout & Navigation (Week 1-2)
1. Implement three-pane layout:
   - Left: Sidebar (Categories, Tags, Vaults)
   - Center: Entry list with search/filter toolbar
   - Right: Entry detail/preview
2. Add responsive breakpoints
3. Implement keyboard navigation (Tab, arrows, Enter, Escape)

### Phase 3: Entry List Enhancement (Week 2)
1. Card-based layout with visual hierarchy
2. Entry type badges (Website, Credit Card, Note, etc.)
3. Inline action buttons (copy user, copy pass, show TOTP)
4. Multi-select with bulk actions
5. Sort by: name, updated, created, category
6. Filter by: category, tag, type, favorites

### Phase 4: Entry Forms & Detail (Week 2-3)
1. Redesign add/edit modal with:
   - Logical field groups
   - Password strength meter (real-time)
   - Password generator button
   - TOTP setup with QR code
   - Custom fields support
2. Detail view with:
   - Large copy buttons
   - TOTP code display with countdown
   - History/versions
   - Related entries

### Phase 5: New Local Features (Week 3)
1. Password Generator dialog
2. Categories/Tags management
3. Vault Statistics dashboard
4. Import/Export improvements (preview, mapping)
5. Keyboard shortcuts overlay (press `?`)
6. Onboarding wizard (first run)

### Phase 6: Polish & Accessibility (Week 3-4)
1. High contrast mode
2. Reduced motion support
3. Screen reader labels
4. Focus indicators
5. Tooltips for all actions
6. Empty states with illustrations
7. Loading/skeleton states

## Technical Approach

### Files to Modify
- `src/bin/vaultling-gui.rs` - Main GUI (will be split into modules)
- New: `src/gui/design_system.rs`
- New: `src/gui/layout.rs`
- New: `src/gui/components/` - Reusable UI components
- New: `src/gui/views/` - View components
- New: `src/gui/password_generator.rs`
- New: `src/gui/onboarding.rs`

### Architecture Principles
- Keep security-first: no secrets in UI state longer than needed
- Use existing vaultling library - no new dependencies for crypto
- Local-only features only (no network calls)
- Maintain backward compatibility with existing vault format

## Success Criteria
- [x] All existing tests still pass (now 121 lib + 1 gui)
- [ ] New GUI builds without warnings
- [ ] Visual consistency across all views
- [ ] Keyboard navigation works for all major flows
- [ ] Password generator produces secure passwords
- [ ] Categories/tags fully functional
- [ ] Onboarding completes in < 2 minutes
- [ ] No regressions in CLI functionality

## Risk Mitigation
- **Scope creep**: Strict phase gates, no new features after Phase 5
- **Breaking changes**: Feature flags for new UI, old UI as fallback
- **Performance**: Virtual scrolling for large vaults, lazy loading
- **Security**: Audit all new code paths for secret handling