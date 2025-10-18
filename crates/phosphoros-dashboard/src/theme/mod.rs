//! Dark theme configuration following blueprint specifications

use iced::widget::container;
use iced::{application, color, Border, Color, Theme};

/// PHOSPHOROS custom theme
pub struct PhosphorosTheme {
    dark_mode: bool,
}

impl PhosphorosTheme {
    /// Create new theme
    pub fn new(dark_mode: bool) -> Self {
        Self { dark_mode }
    }

    /// Get iced Theme
    pub fn theme(&self) -> Theme {
        if self.dark_mode {
            Theme::TokyoNightStorm
        } else {
            Theme::GruvboxLight
        }
    }
}

/// Color palette for dark mode (as per blueprint)
pub mod colors {
    use iced::Color;

    /// Background color #1E1E1E
    pub const BACKGROUND: Color = Color::from_rgb(0.118, 0.118, 0.118);

    /// Text color #C0C0C0
    pub const TEXT: Color = Color::from_rgb(0.753, 0.753, 0.753);

    /// Primary accent color
    pub const PRIMARY: Color = Color::from_rgb(0.4, 0.6, 1.0);

    /// Success color (green)
    pub const SUCCESS: Color = Color::from_rgb(0.298, 0.686, 0.314);

    /// Warning color (orange)
    pub const WARNING: Color = Color::from_rgb(1.0, 0.596, 0.0);

    /// Error color (red)
    pub const ERROR: Color = Color::from_rgb(0.957, 0.263, 0.212);

    /// Info color (blue)
    pub const INFO: Color = Color::from_rgb(0.129, 0.588, 0.953);

    /// Panel background
    pub const PANEL_BG: Color = Color::from_rgb(0.157, 0.157, 0.157);

    /// Border color
    pub const BORDER: Color = Color::from_rgb(0.2, 0.2, 0.2);

    /// Hover color
    pub const HOVER: Color = Color::from_rgb(0.25, 0.25, 0.25);

    /// Active color
    pub const ACTIVE: Color = Color::from_rgb(0.3, 0.3, 0.3);

    /// Sidebar background
    pub const SIDEBAR_BG: Color = Color::from_rgb(0.098, 0.098, 0.098);
}

/// Container styles
pub mod container_styles {
    use super::colors;
    use iced::widget::container;
    use iced::{Border, Color, Theme};

    /// Panel container style
    pub fn panel() -> container::Style {
        container::Style {
            background: Some(colors::PANEL_BG.into()),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }

    /// Sidebar style
    pub fn sidebar() -> container::Style {
        container::Style {
            background: Some(colors::SIDEBAR_BG.into()),
            border: Border {
                color: colors::BORDER,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }

    /// Card style
    pub fn card() -> container::Style {
        container::Style {
            background: Some(colors::PANEL_BG.into()),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }
}
