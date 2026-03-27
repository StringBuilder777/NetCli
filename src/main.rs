use color_eyre::eyre::{
    Ok, Result
};
use ratatui::{
    widgets::{
        Paragraph, Block, Borders
    },
    crossterm::{
        event::{
            self, Event, KeyCode},
    },
    layout::{
        Alignment, Constraint, Direction, Layout
    },
    style::{
        Color, Modifier, Style
    },
    text::{
        Line, Span
    },
    Frame,DefaultTerminal,
};

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum MenuItem{
    Cli,
    Ui
}

impl MenuItem{
    fn next(self) -> Self{
        match self {
            MenuItem::Cli => MenuItem::Ui,
            MenuItem::Ui => MenuItem::Cli
        }
    }
}

struct AppState {
    selected_item: MenuItem,
}

impl AppState {
    fn new() -> AppState {
        Self {
            selected_item: MenuItem::Ui,
        }
    }
}

fn main() -> Result<()>{
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();
    result

}

fn run(mut _terminal: DefaultTerminal) -> Result<()> {
    let mut app = AppState::new();
    loop {
        //rendering
        _terminal.draw(|f| render(f, &app))?;
        //Input handling
        if let Event::Key(key) = event::read()?{
            match key.code {
                KeyCode::Esc => {
                    break;
                }
                KeyCode::Tab | KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down => {
                    app.selected_item = app.selected_item.next();
                }
                KeyCode::Enter => {
                    match app.selected_item {
                        MenuItem::Ui => {}
                        MenuItem::Cli => {}
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
fn render(frame: &mut Frame, app: &AppState) {
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

    let buttons = Line::from(vec![
        Span::raw("         "), btn_cli,
        Span::raw("   "), btn_ui,
    ]);

    let hint = Line::from(Span::styled(
        "▲▼ ◀▶ navegar  |  Enter seleccionar  |  Esc salir",
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
        Span::styled(" NetCli | ESC Salir", Style::default().bg(Color::Blue).fg(Color::White)),
    ]));

    frame.render_widget(status, status_chunks[1]);
}

fn button_span(label: &'static str, selected: bool) -> Span<'static> {
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
