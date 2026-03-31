use color_eyre::eyre::{
    Ok, Result
};

use ratatui::{
    crossterm::{
        event::{
            self, Event, KeyCode, KeyModifiers},
    },
};
use crate::app::{MenuItem ,AppState};

pub enum AppEvent {
    Quit,
    Navigate(MenuItem),
    Select,
    None,
}

pub fn handle_events(app: &AppState) -> Result<AppEvent> {
    if let Event::Key(key) = event::read()? {
        return Ok(match key.code {
            KeyCode::Esc => AppEvent::Quit,
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => AppEvent::Quit,
            KeyCode::Tab | KeyCode::Right | KeyCode::Down => {
                AppEvent::Navigate(app.selected_item.next())
            }
            KeyCode::Left | KeyCode::Up => {
                AppEvent::Navigate(app.selected_item.prev())
            }
            KeyCode::Enter => AppEvent::Select,
            _ => AppEvent::None,
        });
    }
    Ok(AppEvent::None)
}