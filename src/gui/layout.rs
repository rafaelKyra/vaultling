use eframe::egui;

/// Three-pane layout with a fixed-width sidebar, flexible center, and a detail pane.
///
/// Sidebar width: 236px
/// Detail pane width: 376px
/// Center area fills the remaining space.
#[derive(Debug, Clone)]
pub struct ThreePaneLayout {
    /// Width of the sidebar in pixels
    pub sidebar_width: f32,
    /// Width of the detail pane in pixels
    pub detail_width: f32,
}

/// Narrowest the centre list may become before the side panes give way.
pub const MIN_CENTER: f32 = 260.0;

/// Pane widths for a window `total` pixels wide.
///
/// Returns `(sidebar, detail)`. At full size these are the configured widths;
/// in a narrow window both shrink proportionally so the centre list keeps at
/// least [`MIN_CENTER`] pixels, down to a floor below which a pane stops being
/// usable (the window's own minimum size keeps us above it).
pub fn pane_widths(total: f32, sidebar: f32, detail: f32) -> (f32, f32) {
    const SIDEBAR_MIN: f32 = 175.0;
    const DETAIL_MIN: f32 = 240.0;
    // Margins and separators the panels spend on themselves.
    const CHROME: f32 = 70.0;
    let room = (total - MIN_CENTER - CHROME).max(SIDEBAR_MIN + DETAIL_MIN);
    if sidebar + detail <= room {
        return (sidebar, detail);
    }
    let k = room / (sidebar + detail);
    let s = (sidebar * k).max(SIDEBAR_MIN);
    // The sidebar's floor may have taken more than its share; the detail pane
    // gives that back, down to its own floor.
    let d = (detail * k).min(room - s).max(DETAIL_MIN);
    (s, d)
}

impl ThreePaneLayout {
    /// Create a new three-pane layout with default widths.
    pub fn new() -> Self {
        Self {
            sidebar_width: 236.0,
            detail_width: 376.0,
        }
    }

    /// Set the sidebar width.
    pub fn sidebar_width(mut self, width: f32) -> Self {
        self.sidebar_width = width;
        self
    }

    /// Set the detail pane width.
    pub fn detail_width(mut self, width: f32) -> Self {
        self.detail_width = width;
        self
    }

    /// Show the three-pane layout with the given content closures.
    ///
    /// Closure order matches the on-screen order left to right: sidebar,
    /// center, detail. The closures are called sequentially (sidebar, detail,
    /// then center, because side panels must be laid out before the central
    /// panel), each receiving `&mut app` so they can mutably borrow the same state
    /// without borrow-checker conflicts.
    pub fn show<App, F1, F2, F3>(
        self,
        ctx: &egui::Context,
        app: &mut App,
        sidebar: F1,
        center: F2,
        detail: F3,
    ) where
        F1: FnOnce(&mut egui::Ui, &mut App),
        F2: FnOnce(&mut egui::Ui, &mut App),
        F3: FnOnce(&mut egui::Ui, &mut App),
    {
        let total = ctx.screen_rect().width();
        let (sw, dw) = pane_widths(total, self.sidebar_width, self.detail_width);
        // When the window cannot fit the configured widths the side panes are
        // pinned to the shrunk size: a width remembered from a larger window
        // would otherwise squeeze the centre list to nothing after a resize.
        let compact = sw < self.sidebar_width || dw < self.detail_width;

        let side = |id: &'static str, left: bool, w: f32, max: f32, min: f32| {
            let panel = if left {
                egui::SidePanel::left(id)
            } else {
                egui::SidePanel::right(id)
            };
            let panel = if compact {
                panel.resizable(false).exact_width(w)
            } else {
                panel.resizable(true).default_width(w).width_range(min..=max.max(min))
            };
            panel.frame(
                egui::Frame::none()
                    .fill(ctx.style().visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(16.0, 14.0)),
            )
        };

        side("three_pane_sidebar", true, sw, total * 0.4, 175.0).show(ctx, |ui| sidebar(ui, app));
        side("three_pane_detail", false, dw, total * 0.55, 240.0).show(ctx, |ui| detail(ui, app));

        // Center panel
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(ctx.style().visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(16.0, 14.0)),
            )
            .show(ctx, |ui| center(ui, app));
    }
}

impl Default for ThreePaneLayout {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function: show a three-pane layout with default widths.
pub fn show_three_pane<App, F1, F2, F3>(
    ctx: &egui::Context,
    app: &mut App,
    sidebar: F1,
    center: F2,
    detail: F3,
) where
    F1: FnOnce(&mut egui::Ui, &mut App),
    F2: FnOnce(&mut egui::Ui, &mut App),
    F3: FnOnce(&mut egui::Ui, &mut App),
{
    ThreePaneLayout::new().show(ctx, app, sidebar, center, detail);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(width: f32) -> Vec<&'static str> {
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(width, 600.0))),
            ..Default::default()
        };
        let mut calls: Vec<&'static str> = Vec::new();
        let _ = ctx.run(raw, |ctx| {
            show_three_pane(
                ctx,
                &mut calls,
                |_, c| c.push("sidebar"),
                |_, c| c.push("center"),
                |_, c| c.push("detail"),
            );
        });
        calls
    }

    #[test]
    fn closures_render_in_their_own_slots() {
        // Regression: the wrapper once passed `center` into the detail slot.
        let ctx = egui::Context::default();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1200.0, 700.0),
        )),
            ..Default::default()
        };
        let mut rects: std::collections::HashMap<&str, egui::Rect> = Default::default();
        // Two frames: egui settles panel sizes after the first pass.
        for _ in 0..2 {
            let _ = ctx.run(raw.clone(), |ctx| {
                show_three_pane(
                    ctx,
                    &mut rects,
                    |ui, r| {
                        r.insert("sidebar", ui.max_rect());
                    },
                    |ui, r| {
                        r.insert("center", ui.max_rect());
                    },
                    |ui, r| {
                        r.insert("detail", ui.max_rect());
                    },
                );
            });
        }
        assert!(rects["sidebar"].right() <= rects["center"].left() + 1.0);
        assert!(rects["center"].right() <= rects["detail"].left() + 1.0);
    }

    #[test]
    fn pane_widths_keep_the_centre_usable() {
        // Roomy window: configured widths untouched.
        assert_eq!(pane_widths(1600.0, 236.0, 376.0), (236.0, 376.0));
        // Narrow windows shrink the side panes, never below their floors.
        for total in [760.0_f32, 800.0, 980.0] {
            let (s, d) = pane_widths(total, 236.0, 376.0);
            assert!(s >= 175.0 && d >= 240.0, "{total}: {s} {d}");
            if total >= 760.0 {
                assert!(total - s - d - 70.0 >= MIN_CENTER - 1.0, "{total}: {s} {d}");
            }
        }
    }

    #[test]
    fn every_pane_runs_once_at_small_and_large_sizes() {
        for w in [480.0, 800.0, 1600.0] {
            let mut calls = run(w);
            calls.sort();
            assert_eq!(calls, ["center", "detail", "sidebar"], "width {w}");
        }
    }
}
