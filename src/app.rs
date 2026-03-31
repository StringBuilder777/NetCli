#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MenuItem{
    Cli,
    Ui,
    Cal
}
impl MenuItem{
    pub fn next(self) -> Self{
        match self {
            MenuItem::Cli => MenuItem::Ui,
            MenuItem::Ui => MenuItem::Cal,
            MenuItem::Cal => MenuItem::Cli
        }
    }
    pub fn prev(self) -> Self  {
        match self {
            MenuItem::Cli => MenuItem::Cal,
            MenuItem::Ui => MenuItem::Cli,
            MenuItem::Cal => MenuItem::Ui
        }
    }
}
pub struct AppState {
    pub selected_item: MenuItem,
}
impl AppState {
    pub fn new() -> AppState {
        Self {
            selected_item: MenuItem::Ui,
        }
    }
}