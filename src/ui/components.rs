use ratatui::{
    style::{
        Color, Modifier, Style
    },
    text::{
        Span
    },
};

pub fn button_span(label: &'static str, selected: bool) -> Span<'static> {
    if selected {
        Span::styled(
            label,
            Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(
            label,
            Style::default()
                .fg(Color::LightCyan)
                .add_modifier(Modifier::DIM),
        )
    }
}