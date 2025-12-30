//! Premium custom widgets for PHOSPHOROS dashboard

use crate::theme::{colors, container_styles};
use iced::widget::{button, column, container, text, horizontal_space, vertical_space, Button};
use iced::{Element, Length};

/// Create a premium card widget with elevation
pub fn card<'a, Message>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .padding(20)
        .style(|_theme: &iced::Theme| container_styles::card())
        .into()
}

/// Create a premium panel widget
pub fn panel<'a, Message>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .padding(24)
        .style(|_theme: &iced::Theme| container_styles::panel())
        .into()
}

/// Create a premium status card with colored border
pub fn status_card<'a, Message>(
    status: StatusType,
    content: impl Into<Element<'a, Message>>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let style = match status {
        StatusType::Success => container_styles::status_success(),
        StatusType::Warning => container_styles::status_warning(),
        StatusType::Error => container_styles::status_error(),
        StatusType::Info => container_styles::status_info(),
    };
    
    container(content)
        .padding(20)
        .style(move |_theme: &iced::Theme| style)
        .into()
}

/// Status type for status cards
#[derive(Debug, Clone, Copy)]
pub enum StatusType {
    Success,
    Warning,
    Error,
    Info,
}

/// Create a premium header with title and optional actions
pub fn header<'a, Message>(
    title: &'a str,
    subtitle: Option<&'a str>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let mut content = column![
        text(title).size(32),
    ].spacing(4);
    
    if let Some(sub) = subtitle {
        content = content.push(
            text(sub).size(14)
        );
    }
    
    content.into()
}

/// Create a premium badge
pub fn badge<'a, Message>(
    label: &'a str,
    badge_type: BadgeType,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let bg_color = match badge_type {
        BadgeType::Primary => colors::PRIMARY,
        BadgeType::Success => colors::SUCCESS,
        BadgeType::Warning => colors::WARNING,
        BadgeType::Error => colors::ERROR,
        BadgeType::Info => colors::INFO,
        BadgeType::Neutral => colors::BORDER_HOVER,
    };
    
    container(
        text(label).size(12)
    )
    .padding([4, 12])
    .style(move |_theme: &iced::Theme| {
        container::Style {
            background: Some(bg_color.into()),
            border: iced::Border {
                radius: 12.0.into(),
                ..Default::default()
            },
            ..Default::default()
        }
    })
    .into()
}

/// Badge type for different semantic meanings
#[derive(Debug, Clone, Copy)]
pub enum BadgeType {
    Primary,
    Success,
    Warning,
    Error,
    Info,
    Neutral,
}

/// Create a metric card showing a key statistic
pub fn metric_card<'a, Message>(
    label: &'a str,
    value: &'a str,
    icon: &'a str,
    trend: Option<MetricTrend>,
) -> Element<'a, Message>
where
    Message: 'a,
{
    let trend_element = if let Some(t) = trend {
        let trend_text = match t {
            MetricTrend::Up(v) => format!("▲ {}", v),
            MetricTrend::Down(v) => format!("▼ {}", v),
            MetricTrend::Neutral => "—".to_string(),
        };
        
        Some(text(trend_text).size(12))
    } else {
        None
    };
    
    let mut content = column![
        text(icon).size(24),
        vertical_space().height(8),
        text(value).size(28),
        vertical_space().height(4),
        text(label).size(12),
    ].spacing(2);
    
    if let Some(trend_elem) = trend_element {
        content = content.push(vertical_space().height(4));
        content = content.push(trend_elem);
    }
    
    card(content)
}

/// Metric trend indicator
#[derive(Debug, Clone)]
pub enum MetricTrend {
    Up(String),
    Down(String),
    Neutral,
}

/// Create a premium divider
pub fn divider<'a, Message>() -> Element<'a, Message>
where
    Message: 'a,
{
    container(horizontal_space())
        .height(1)
        .width(Length::Fill)
        .style(|_theme: &iced::Theme| {
            container::Style {
                background: Some(colors::BORDER.into()),
                ..Default::default()
            }
        })
        .into()
}

/// Create a section header
pub fn section_header<'a, Message>(title: &'a str) -> Element<'a, Message>
where
    Message: 'a,
{
    column![
        text(title).size(18),
        vertical_space().height(2),
        divider(),
    ]
    .spacing(8)
    .into()
}

/// Create a glass morphism overlay container
pub fn glass_container<'a, Message>(content: impl Into<Element<'a, Message>>) -> Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .padding(24)
        .style(|_theme: &iced::Theme| container_styles::glass())
        .into()
}

/// Premium button styles
pub mod button_styles {
    use crate::theme::colors;
    use iced::widget::button;
    use iced::{Border, Color, Shadow, Vector};

    /// Primary button style
    pub fn primary() -> button::Style {
        button::Style {
            background: Some(colors::PRIMARY.into()),
            text_color: colors::TEXT,
            border: Border {
                color: colors::PRIMARY_BRIGHT,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.25, 0.55, 0.95, 0.3),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    }

    /// Primary button hover state
    pub fn primary_hover() -> button::Style {
        button::Style {
            background: Some(colors::PRIMARY_BRIGHT.into()),
            text_color: colors::TEXT,
            border: Border {
                color: colors::PRIMARY_BRIGHT,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.25, 0.55, 0.95, 0.5),
                offset: Vector::new(0.0, 4.0),
                blur_radius: 12.0,
            },
        }
    }

    /// Secondary button style
    pub fn secondary() -> button::Style {
        button::Style {
            background: Some(colors::CARD_BG.into()),
            text_color: colors::TEXT,
            border: Border {
                color: colors::BORDER_HOVER,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: colors::SHADOW_SOFT,
                offset: Vector::new(0.0, 1.0),
                blur_radius: 4.0,
            },
        }
    }

    /// Secondary button hover state
    pub fn secondary_hover() -> button::Style {
        button::Style {
            background: Some(colors::HOVER.into()),
            text_color: colors::TEXT,
            border: Border {
                color: colors::BORDER_ACTIVE,
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: colors::SHADOW_MEDIUM,
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    }

    /// Success button style
    pub fn success() -> button::Style {
        button::Style {
            background: Some(colors::SUCCESS.into()),
            text_color: colors::TEXT,
            border: Border {
                color: Color::from_rgba(0.18, 0.72, 0.55, 0.5),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.18, 0.72, 0.55, 0.3),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    }

    /// Danger button style
    pub fn danger() -> button::Style {
        button::Style {
            background: Some(colors::ERROR.into()),
            text_color: colors::TEXT,
            border: Border {
                color: Color::from_rgba(0.93, 0.28, 0.35, 0.5),
                width: 1.0,
                radius: 8.0.into(),
            },
            shadow: Shadow {
                color: Color::from_rgba(0.93, 0.28, 0.35, 0.3),
                offset: Vector::new(0.0, 2.0),
                blur_radius: 8.0,
            },
        }
    }
}

/// Create a premium primary button
pub fn primary_button<'a, Message>(label: &'a str) -> Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(text(label).size(14))
        .padding([12, 24])
        .style(|_theme: &iced::Theme, status| {
            match status {
                button::Status::Hovered => button_styles::primary_hover(),
                _ => button_styles::primary(),
            }
        })
}

/// Create a premium secondary button
pub fn secondary_button<'a, Message>(label: &'a str) -> Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(text(label).size(14))
        .padding([12, 24])
        .style(|_theme: &iced::Theme, status| {
            match status {
                button::Status::Hovered => button_styles::secondary_hover(),
                _ => button_styles::secondary(),
            }
        })
}

/// Create a premium success button
pub fn success_button<'a, Message>(label: &'a str) -> Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(text(label).size(14))
        .padding([12, 24])
        .style(|_theme: &iced::Theme, _status| {
            button_styles::success()
        })
}

/// Create a premium danger button
pub fn danger_button<'a, Message>(label: &'a str) -> Button<'a, Message>
where
    Message: Clone + 'a,
{
    button(text(label).size(14))
        .padding([12, 24])
        .style(|_theme: &iced::Theme, _status| {
            button_styles::danger()
        })
}
