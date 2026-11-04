//! Design System for Vaultling GUI
//!
//! This module provides a complete design token system including:
//! - Color palette (light/dark themes)
//! - Typography scale
//! - Spacing system
//! - Border radius
//! - Shadows
//! - Animation durations

use eframe::egui;

/// Color palette with semantic roles for both light and dark themes.
#[derive(Clone, Copy, Debug)]
pub struct Palette {
    // Base surfaces
    pub bg: egui::Color32,
    pub surface: egui::Color32,
    pub surface_hi: egui::Color32,
    pub line: egui::Color32,

    // Brand / accent
    pub accent: egui::Color32,
    pub on_accent: egui::Color32,

    // Text
    pub text: egui::Color32,
    pub muted: egui::Color32,

    // Semantic states
    pub danger: egui::Color32,
    pub warn: egui::Color32,
    pub ok: egui::Color32,
}

/// Dark theme palette
pub const DARK: Palette = Palette {
    bg: egui::Color32::from_rgb(0x11, 0x13, 0x18),
    surface: egui::Color32::from_rgb(0x1a, 0x1d, 0x24),
    surface_hi: egui::Color32::from_rgb(0x25, 0x29, 0x33),
    line: egui::Color32::from_rgb(0x2a, 0x2f, 0x3a),
    accent: egui::Color32::from_rgb(0xe8, 0xb3, 0x4a),
    on_accent: egui::Color32::from_rgb(0x1a, 0x14, 0x05),
    text: egui::Color32::from_rgb(0xe8, 0xea, 0xef),
    muted: egui::Color32::from_rgb(0x8c, 0x93, 0xa4),
    danger: egui::Color32::from_rgb(0xff, 0x6b, 0x6b),
    warn: egui::Color32::from_rgb(0xf0, 0xb4, 0x5f),
    ok: egui::Color32::from_rgb(0x5d, 0xd6, 0x8e),
};

/// Light theme palette
pub const LIGHT: Palette = Palette {
    bg: egui::Color32::from_rgb(0xf6, 0xf7, 0xf9),
    surface: egui::Color32::from_rgb(0xff, 0xff, 0xff),
    surface_hi: egui::Color32::from_rgb(0xe9, 0xec, 0xf1),
    line: egui::Color32::from_rgb(0xd8, 0xdd, 0xe5),
    accent: egui::Color32::from_rgb(0xb5, 0x7c, 0x10),
    on_accent: egui::Color32::from_rgb(0xff, 0xff, 0xff),
    text: egui::Color32::from_rgb(0x1a, 0x1d, 0x24),
    muted: egui::Color32::from_rgb(0x5f, 0x67, 0x76),
    danger: egui::Color32::from_rgb(0xc0, 0x28, 0x28),
    warn: egui::Color32::from_rgb(0xa8, 0x86, 0x0a),
    ok: egui::Color32::from_rgb(0x1a, 0x7f, 0x4b),
};

thread_local! {
    static PALETTE: std::cell::Cell<Palette> = const { std::cell::Cell::new(DARK) };
}

/// Get the current palette.
pub fn palette() -> Palette {
    PALETTE.with(|c| c.get())
}

/// Set the active palette (light or dark).
pub fn set_palette(light: bool) {
    PALETTE.with(|c| c.set(if light { LIGHT } else { DARK }));
}

/// Typography scale with semantic names.
pub mod typography {
    use eframe::egui;

    /// Display / hero text (32px)
    pub const DISPLAY: f32 = 32.0;
    /// Large heading (26px)
    pub const H1: f32 = 26.0;
    /// Medium heading (21px)
    pub const H2: f32 = 21.0;
    /// Small heading (18px)
    pub const H3: f32 = 18.0;
    /// Title / large body (17px)
    pub const TITLE: f32 = 17.0;
    /// Body large (14px)
    pub const BODY_LG: f32 = 14.0;
    /// Body default (13.5px)
    pub const BODY: f32 = 13.5;
    /// Body small (13px)
    pub const BODY_SM: f32 = 13.0;
    /// Caption / secondary (12.5px)
    pub const CAPTION: f32 = 12.5;
    /// Small caption (12px)
    pub const CAPTION_SM: f32 = 12.0;
    /// Tiny text (11.5px)
    pub const SMALL: f32 = 11.5;
    /// Micro text (11px)
    pub const MICRO: f32 = 11.0;
    /// Tiny micro (10.5px)
    pub const MICRO_SM: f32 = 10.5;

    /// Font weight: regular
    pub fn regular() -> egui::FontId {
        egui::FontId::proportional(BODY)
    }

    /// Font weight: strong/bold
    pub fn strong(size: f32) -> egui::FontId {
        egui::FontId::new(size, egui::FontFamily::Proportional)
    }

    /// Font weight: monospace
    pub fn mono(size: f32) -> egui::FontId {
        egui::FontId::new(size, egui::FontFamily::Monospace)
    }
}

/// Spacing system based on a 4px base unit.
pub mod spacing {
    /// Base unit (4px)
    pub const UNIT: f32 = 4.0;

    /// 0.5x unit (2px)
    pub const XS: f32 = UNIT * 0.5;
    /// 1x unit (4px)
    pub const SM: f32 = UNIT;
    /// 1.5x unit (6px)
    pub const MD: f32 = UNIT * 1.5;
    /// 2x unit (8px)
    pub const LG: f32 = UNIT * 2.0;
    /// 3x unit (12px)
    pub const XL: f32 = UNIT * 3.0;
    /// 4x unit (16px)
    pub const XXL: f32 = UNIT * 4.0;
    /// 6x unit (24px)
    pub const XXXL: f32 = UNIT * 6.0;
    /// 8x unit (32px)
    pub const HUGE: f32 = UNIT * 8.0;
    /// 10x unit (40px)
    pub const MASSIVE: f32 = UNIT * 10.0;
    /// 15x unit (60px)
    pub const GIANT: f32 = UNIT * 15.0;
}

/// Border radius scale.
pub mod radius {
    use eframe::egui;

    /// None (0px)
    pub const NONE: f32 = 0.0;
    /// Extra small (2px)
    pub const XS: f32 = 2.0;
    /// Small (4px)
    pub const SM: f32 = 4.0;
    /// Medium (6px)
    pub const MD: f32 = 6.0;
    /// Large (8px)
    pub const LG: f32 = 8.0;
    /// Extra large (10px)
    pub const XL: f32 = 10.0;
    /// Round (14px)
    pub const ROUND: f32 = 14.0;
    /// Pill / fully rounded (9999px)
    pub const PILL: f32 = 9999.0;

    /// Convert to egui::Rounding
    pub fn to_rounding(r: f32) -> egui::Rounding {
        egui::Rounding::same(r)
    }
}

/// Shadow definitions.
pub mod shadow {
    use eframe::egui;

    /// No shadow
    pub const NONE: (f32, egui::Color32) = (0.0, egui::Color32::TRANSPARENT);

    /// Subtle shadow for cards (2px blur, 10% opacity)
    pub const SM: (f32, egui::Color32) = (2.0, egui::Color32::from_black_alpha(25));

    /// Medium shadow for elevated elements (8px blur, 15% opacity)
    pub const MD: (f32, egui::Color32) = (8.0, egui::Color32::from_black_alpha(38));

    /// Large shadow for modals/dropdowns (16px blur, 20% opacity)
    pub const LG: (f32, egui::Color32) = (16.0, egui::Color32::from_black_alpha(51));

    /// Extra large shadow for toasts/overlays (24px blur, 25% opacity)
    pub const XL: (f32, egui::Color32) = (24.0, egui::Color32::from_black_alpha(64));

    /// Apply shadow to a rect on a painter
    pub fn paint(
        painter: &egui::Painter,
        rect: egui::Rect,
        rounding: egui::Rounding,
        level: (f32, egui::Color32),
    ) {
        let (blur, color) = level;
        if blur > 0.0 {
            painter.rect_filled(
                rect.expand(blur),
                rounding,
                color,
            );
        }
    }
}

/// Animation durations.
pub mod animation {
    /// Instant (0ms)
    pub const INSTANT: f32 = 0.0;
    /// Fast (100ms)
    pub const FAST: f32 = 0.1;
    /// Normal (200ms)
    pub const NORMAL: f32 = 0.2;
    /// Slow (300ms)
    pub const SLOW: f32 = 0.3;
    /// Very slow (500ms)
    pub const VERY_SLOW: f32 = 0.5;
}

/// Z-index layers for proper stacking.
pub mod z_index {
    /// Base content
    pub const BASE: i32 = 0;
    /// Dropdown menus
    pub const DROPDOWN: i32 = 100;
    /// Tooltips
    pub const TOOLTIP: i32 = 200;
    /// Modal overlay
    pub const MODAL: i32 = 300;
    /// Toast notifications
    pub const TOAST: i32 = 400;
    /// Context menus
    pub const CONTEXT_MENU: i32 = 500;
}

/// Breakpoints for responsive design.
pub mod breakpoint {
    /// Mobile (< 600px)
    pub const MOBILE: f32 = 600.0;
    /// Tablet (600px - 900px)
    pub const TABLET: f32 = 900.0;
    /// Desktop (900px - 1200px)
    pub const DESKTOP: f32 = 1200.0;
    /// Wide (> 1200px)
    pub const WIDE: f32 = 1200.0;
}

/// Icon sizes.
pub mod icon {
    /// Tiny (12px)
    pub const XS: f32 = 12.0;
    /// Small (16px)
    pub const SM: f32 = 16.0;
    /// Medium (20px)
    pub const MD: f32 = 20.0;
    /// Large (24px)
    pub const LG: f32 = 24.0;
    /// Extra large (32px)
    pub const XL: f32 = 32.0;
}

/// Component-specific tokens.
pub mod component {
    use super::{radius, spacing};

    /// Button heights
    pub mod button {
        pub const SM: f32 = 28.0;
        pub const MD: f32 = 34.0;
        pub const LG: f32 = 40.0;
    }

    /// Input heights
    pub mod input {
        pub const SM: f32 = 28.0;
        pub const MD: f32 = 34.0;
        pub const LG: f32 = 40.0;
    }

    /// Card padding
    pub mod card {
        pub const PADDING: f32 = super::spacing::LG;
        pub const GAP: f32 = super::spacing::MD;
    }

    /// List item
    pub mod list_item {
        pub const HEIGHT: f32 = 56.0;
        pub const PADDING_H: f32 = super::spacing::LG;
        pub const PADDING_V: f32 = super::spacing::SM;
    }

    /// Modal
    pub mod modal {
        pub const MAX_WIDTH: f32 = 560.0;
        pub const PADDING: f32 = super::spacing::XXL;
        pub const RADIUS: f32 = super::radius::LG;
    }

    /// Tooltip
    pub mod tooltip {
        pub const MAX_WIDTH: f32 = 280.0;
        pub const PADDING: f32 = super::spacing::SM;
        pub const GAP: f32 = super::spacing::XS;
    }
}

/// Helper trait for applying design tokens to egui widgets.
pub trait DesignTokens {
    /// Apply the current palette's background color.
    fn with_bg(self) -> Self;
    /// Apply the current palette's surface color.
    fn with_surface(self) -> Self;
    /// Apply the current palette's text color.
    fn with_text(self) -> Self;
    /// Apply the current palette's muted text color.
    fn with_muted(self) -> Self;
    /// Apply the current palette's accent color.
    fn with_accent(self) -> Self;
    /// Apply the current palette's danger color.
    fn with_danger(self) -> Self;
    /// Apply the current palette's warning color.
    fn with_warn(self) -> Self;
    /// Apply the current palette's success color.
    fn with_ok(self) -> Self;
}

impl DesignTokens for egui::Color32 {
    fn with_bg(self) -> Self {
        palette().bg
    }
    fn with_surface(self) -> Self {
        palette().surface
    }
    fn with_text(self) -> Self {
        palette().text
    }
    fn with_muted(self) -> Self {
        palette().muted
    }
    fn with_accent(self) -> Self {
        palette().accent
    }
    fn with_danger(self) -> Self {
        palette().danger
    }
    fn with_warn(self) -> Self {
        palette().warn
    }
    fn with_ok(self) -> Self {
        palette().ok
    }
}

/// Helper to create styled text with design system tokens.
pub fn styled_text(text: impl Into<String>, size: f32, color: egui::Color32) -> egui::RichText {
    egui::RichText::new(text).size(size).color(color)
}

/// Helper to create strong (bold) styled text.
pub fn strong_text(text: impl Into<String>, size: f32, color: egui::Color32) -> egui::RichText {
    egui::RichText::new(text).size(size).strong().color(color)
}

/// Helper to create monospace styled text.
pub fn mono_text(text: impl Into<String>, size: f32, color: egui::Color32) -> egui::RichText {
    egui::RichText::new(text).size(size).monospace().color(color)
}