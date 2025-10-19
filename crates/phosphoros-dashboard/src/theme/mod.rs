//! Dark theme configuration following blueprint specifications

use iced::widget::container;
use iced::{application, color, Border, Color, Theme};

/// PHOSPHOROS custom theme
#[derive(Debug)]
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

/// Premium color palette for high-end forensic suite
pub mod colors {
    use iced::Color;

    // === Core Background Colors ===
    /// Deep background color - professional dark base
    pub const BACKGROUND: Color = Color::from_rgb(0.08, 0.09, 0.11);
    
    /// Panel background - elevated surface
    pub const PANEL_BG: Color = Color::from_rgb(0.11, 0.12, 0.15);
    
    /// Card background - slightly elevated
    pub const CARD_BG: Color = Color::from_rgb(0.13, 0.14, 0.17);
    
    /// Sidebar background - distinctive side panel
    pub const SIDEBAR_BG: Color = Color::from_rgb(0.06, 0.07, 0.09);

    // === Text Colors ===
    /// Primary text - high contrast
    pub const TEXT: Color = Color::from_rgb(0.92, 0.93, 0.95);
    
    /// Secondary text - subdued
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.65, 0.67, 0.70);
    
    /// Tertiary text - very subdued
    pub const TEXT_TERTIARY: Color = Color::from_rgb(0.45, 0.47, 0.50);

    // === Premium Accent Colors ===
    /// Primary accent - sophisticated blue
    pub const PRIMARY: Color = Color::from_rgb(0.25, 0.55, 0.95);
    
    /// Primary bright - for highlights
    pub const PRIMARY_BRIGHT: Color = Color::from_rgb(0.35, 0.65, 1.0);
    
    /// Primary dark - for depth
    pub const PRIMARY_DARK: Color = Color::from_rgb(0.15, 0.45, 0.85);

    // === Status Colors ===
    /// Success - refined green
    pub const SUCCESS: Color = Color::from_rgb(0.18, 0.72, 0.55);
    
    /// Warning - premium amber
    pub const WARNING: Color = Color::from_rgb(0.95, 0.65, 0.18);
    
    /// Error - refined red
    pub const ERROR: Color = Color::from_rgb(0.93, 0.28, 0.35);
    
    /// Info - sophisticated cyan
    pub const INFO: Color = Color::from_rgb(0.22, 0.68, 0.88);

    // === Interactive States ===
    /// Border color - subtle
    pub const BORDER: Color = Color::from_rgb(0.18, 0.19, 0.22);
    
    /// Border hover - visible
    pub const BORDER_HOVER: Color = Color::from_rgb(0.28, 0.29, 0.32);
    
    /// Border active - prominent
    pub const BORDER_ACTIVE: Color = Color::from_rgb(0.35, 0.55, 0.95);
    
    /// Hover background
    pub const HOVER: Color = Color::from_rgb(0.16, 0.17, 0.20);
    
    /// Active background
    pub const ACTIVE: Color = Color::from_rgb(0.18, 0.19, 0.22);

    // === Gradient Colors ===
    /// Gradient start - premium effect
    pub const GRADIENT_START: Color = Color::from_rgb(0.15, 0.35, 0.75);
    
    /// Gradient end - premium effect  
    pub const GRADIENT_END: Color = Color::from_rgb(0.25, 0.15, 0.65);

    // === Resonance Spectrum Colors ===
    /// High resonance - gold/amber
    pub const RESONANCE_HIGH: Color = Color::from_rgb(0.95, 0.75, 0.25);
    
    /// Medium resonance - cyan
    pub const RESONANCE_MEDIUM: Color = Color::from_rgb(0.35, 0.75, 0.85);
    
    /// Low resonance - blue
    pub const RESONANCE_LOW: Color = Color::from_rgb(0.45, 0.55, 0.95);

    // === Shadow Colors (with transparency) ===
    /// Soft shadow
    pub const SHADOW_SOFT: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.15);
    
    /// Medium shadow
    pub const SHADOW_MEDIUM: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.25);
    
    /// Strong shadow
    pub const SHADOW_STRONG: Color = Color::from_rgba(0.0, 0.0, 0.0, 0.40);

    // === Glass Morphism ===
    /// Glass overlay
    pub const GLASS: Color = Color::from_rgba(0.15, 0.16, 0.19, 0.85);
    
    /// Glass border
    pub const GLASS_BORDER: Color = Color::from_rgba(0.40, 0.42, 0.45, 0.30);
}

/// Premium container styles
pub mod container_styles {
    use super::colors;
    use iced::widget::container;
    use iced::{Border, Color, Shadow, Theme, Vector};

    /// Premium panel container style
    pub fn panel() -> container::Style {
        container::Style {
            background: Some(colors::PANEL_BG.into()),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 12.0.into(),
            },
            shadow: Shadow {
                color: colors::SHADOW_SOFT,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    }

    /// Sidebar style with premium aesthetics
    pub fn sidebar() -> container::Style {
        container::Style {
            background: Some(colors::SIDEBAR_BG.into()),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 0.0.into(),
            },
            shadow: Shadow {
                color: colors::SHADOW_MEDIUM,
                offset: Vector::new(2.0, 0.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }
    }

    /// Premium card style with elevation
    pub fn card() -> container::Style {
        container::Style {
            background: Some(colors::CARD_BG.into()),
            border: Border {
                color: colors::BORDER,
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow {
                color: colors::SHADOW_SOFT,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
            ..Default::default()
        }
    }

    /// Premium card with hover state
    pub fn card_hover() -> container::Style {
        container::Style {
            background: Some(colors::CARD_BG.into()),
            border: Border {
                color: colors::BORDER_HOVER,
                width: 1.0,
                radius: 10.0.into(),
            },
            shadow: Shadow {
                color: colors::SHADOW_MEDIUM,
                offset: Vector::new(0.0, 4.0),
                blur_radius: 16.0,
            },
            ..Default::default()
        }
    }

    /// Glass morphism style for overlays
    pub fn glass() -> container::Style {
        container::Style {
            background: Some(colors::GLASS.into()),
            border: Border {
                color: colors::GLASS_BORDER,
                width: 1.0,
                radius: 16.0.into(),
            },
            shadow: Shadow {
                color: colors::SHADOW_STRONG,
                offset: Vector::new(0.0, 8.0),
                blur_radius: 24.0,
            },
            ..Default::default()
        }
    }

    /// Premium status card (success)
    pub fn status_success() -> container::Style {
        container::Style {
            background: Some(Color::from_rgba(0.18, 0.72, 0.55, 0.1).into()),
            border: Border {
                color: colors::SUCCESS,
                width: 1.5,
                radius: 10.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.18, 0.72, 0.55, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    }

    /// Premium status card (warning)
    pub fn status_warning() -> container::Style {
        container::Style {
            background: Some(Color::from_rgba(0.95, 0.65, 0.18, 0.1).into()),
            border: Border {
                color: colors::WARNING,
                width: 1.5,
                radius: 10.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.95, 0.65, 0.18, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    }

    /// Premium status card (error)
    pub fn status_error() -> container::Style {
        container::Style {
            background: Some(Color::from_rgba(0.93, 0.28, 0.35, 0.1).into()),
            border: Border {
                color: colors::ERROR,
                width: 1.5,
                radius: 10.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.93, 0.28, 0.35, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    }

    /// Premium status card (info)
    pub fn status_info() -> container::Style {
        container::Style {
            background: Some(Color::from_rgba(0.22, 0.68, 0.88, 0.1).into()),
            border: Border {
                color: colors::INFO,
                width: 1.5,
                radius: 10.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.22, 0.68, 0.88, 0.2),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 12.0,
            },
            ..Default::default()
        }
    }
}
