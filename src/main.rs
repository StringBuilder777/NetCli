use color_eyre::eyre::{
    Ok, Result
};
use ratatui::{DefaultTerminal, crossterm::{
    event::{
        self, Event},
    terminal}, Frame};
use ratatui::widgets::{Paragraph, Widget};

fn main() -> Result<()>{
    println!("Hello, world!");
    color_eyre::install()?;

    let terminal = ratatui::init();
    let result = run(terminal);

    ratatui::restore();
    result

}

fn run(mut terminal: DefaultTerminal) -> Result<()> {
    loop {
        //rendering
        terminal.draw(render)?;
        //Input handling
        if let Event::Key(key) = event::read()?{
            match key.code {
                event::KeyCode::Esc => {
                    break;
                }
                _ => {}
            }
        }
    }
    Ok(())
}

fn  render(frame: &mut Frame){
    Paragraph::new("Hola chavo").render(frame.area(), frame.buffer_mut());
}
