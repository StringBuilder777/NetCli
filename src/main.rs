mod app;
mod event;
mod ui;

use color_eyre::eyre::Result;
use ratatui::DefaultTerminal;
use app::AppState;
use event::{handle_events, AppEvent};

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    let mut app = AppState::new();
    loop {
        terminal.draw(|f| ui::welcome::render(f, &app))?;

        match handle_events(&app)? {
            AppEvent::Quit => break,
            AppEvent::Navigate(item) => app.selected_item = item,
            AppEvent::Select => {
                // aquí después decides qué pantalla cargar
            }
            AppEvent::None => {}
        }
    }
    Ok(())
}