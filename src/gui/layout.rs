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
    /// The closures are called sequentially (sidebar, then detail, then center),
    /// each receiving `&mut app` so they can mutably borrow the same state
    /// without borrow-checker conflicts.
    pub fn show<App, F1, F2, F3>(
        self,
        ctx: &egui::Context,
        app: &mut App,
        sidebar: F1,
        detail: F3,
        center: F2,
    ) where
        F1: FnOnce(&mut egui::Ui, &mut App),
        F2: FnOnce(&mut egui::Ui, &mut App),
        F3: FnOnce(&mut egui::Ui, &mut App),
    {
        // Sidebar (left)
        egui::SidePanel::left("three_pane_sidebar")
            .resizable(true)
            .default_width(self.sidebar_width)
            .frame(
                egui::Frame::none()
                    .fill(ctx.style().visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(16.0, 14.0)),
            )
            .show(ctx, |ui| sidebar(ui, app));

        // Detail pane (right)
        egui::SidePanel::right("three_pane_detail")
            .resizable(true)
            .default_width(self.detail_width)
            .frame(
                egui::Frame::none()
                    .fill(ctx.style().visuals.panel_fill)
                    .inner_margin(egui::Margin::symmetric(16.0, 14.0)),
            )
            .show(ctx, |ui| detail(ui, app));

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
