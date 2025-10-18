//! Custom widgets for PHOSPHOROS dashboard

/// Re-export commonly used iced widgets
pub use iced::widget::*;

/// Create a card widget
pub fn card<'a, Message>(content: impl Into<iced::Element<'a, Message>>) -> iced::Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .padding(16)
        .style(|_theme: &iced::Theme| crate::theme::container_styles::card())
        .into()
}

/// Create a panel widget
pub fn panel<'a, Message>(content: impl Into<iced::Element<'a, Message>>) -> iced::Element<'a, Message>
where
    Message: 'a,
{
    container(content)
        .padding(20)
        .style(|_theme: &iced::Theme| crate::theme::container_styles::panel())
        .into()
}
