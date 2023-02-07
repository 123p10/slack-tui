use std::io::{Result};
use std::time::Duration;
use tui::{
    backend::{CrosstermBackend},
    Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use crate::app;

const TIMEOUT_MS: u64 = 250;

pub fn setup_tui() -> Result<Terminal<CrosstermBackend<std::io::Stdout>>>
{
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();    
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect("Failed to Initialize Terminal");
    
    
    Ok(terminal)
}
pub fn destroy_tui(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>) -> Result<()>
{
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn tick(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app_inst: &mut app::App) -> Result<()>
{
    let timeout = Duration::from_millis(TIMEOUT_MS);
    if crossterm::event::poll(timeout)? {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => app::on_key(app_inst, c),
                _ => {}
            }
        }
    }
    Ok(())
}

