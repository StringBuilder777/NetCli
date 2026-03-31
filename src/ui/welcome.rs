use ratatui::{
    widgets::{Paragraph, Block, Borders},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    Frame,
};
use crate::app::{AppState, MenuItem};
use crate::ui::components::button_span;

const ASCII_NAME :&str = r"
 ██████   █████           █████      █████████  ████  ███
░░██████ ░░███           ░░███      ███░░░░░███░░███ ░░░
 ░███░███ ░███   ██████ ████████   ███     ░░░  ░███ ████
 ░███░░███░███  ███░░███░░░███░   ░███          ░███░░███
 ░███ ░░██████ ░███████   ░███    ░███          ░███ ░███
 ░███  ░░█████ ░███░░░    ░███    ░░███     ███ ░███ ░███
 █████  ░░█████░░██████   ░░█████  ░░█████████  ██████████
░░░░░    ░░░░░  ░░░░░░     ░░░░░    ░░░░░░░░░  ░░░░░░░░░░
";

pub fn render(frame: &mut Frame, app: &AppState) {
    let area = frame.area();

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(40),
            Constraint::Fill(1),
        ])
        .split(area);

    let horizontal_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Length(300),
            Constraint::Fill(1),
        ])
        .split(vertical_chunks[1]);

    let panel_area = horizontal_chunks[1];

    let name_lines: Vec<Line> = ASCII_NAME
        .lines()
        .map(|l| Line::from(Span::styled(l, Style::default().fg(Color::Blue))))
        .collect();

    let subtitle = Line::from(Span::styled(
        "NetCli ~ v.0.1.0",
        Style::default().add_modifier(Modifier::BOLD).fg(Color::White),
    ));

    let btn_cli = button_span("  ⌨  Línea de comandos  ", app.selected_item == MenuItem::Cli);
    let btn_ui  = button_span("  ▦  Interfaz visual    ", app.selected_item == MenuItem::Ui);
    let btn_cal  = button_span("  λ  subnet   ", app.selected_item == MenuItem::Cal);

    let buttons = Line::from(vec![
        Span::raw("     "), btn_cli,
        Span::raw("     "), btn_ui,
        Span::raw("     "), btn_cal,
    ]);

    let hint = Line::from(Span::styled(
        "▲ ▼ ◀ ▶ navegar  |  Enter seleccionar  |  Esc salir",
        Style::default().fg(Color::Gray),
    ));

    // ← aquí estaba el bug: name_lines nunca se agregaba
    let mut text: Vec<Line> = name_lines;
    text.push(Line::raw(""));
    text.push(subtitle);
    text.push(Line::raw(""));
    text.push(buttons);
    text.push(Line::raw(""));
    text.push(hint);

    let block = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::White))
                .title(Span::styled(
                    " NetCli ",
                    Style::default().fg(Color::Blue),
                )),
        )
        .alignment(Alignment::Center);

    frame.render_widget(block, panel_area);

    // Barra de estado
    let status_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Length(1)])
        .split(area);

    let status = Paragraph::new(Line::from(vec![
        Span::styled(" Ctrl+C | ESC : para Salir", Style::default().bg(Color::Blue).fg(Color::White)),
    ]));

    frame.render_widget(status, status_chunks[1]);
}
