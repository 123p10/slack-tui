use std::io::{Result};
use std::time::Duration;
use tui::{
    backend::{CrosstermBackend},
    Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph},
    text::Text,
    Frame
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
    terminal.draw(|f| ui(f, app_inst))?;
    Ok(())
}

fn ui(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app_inst: &mut app::App)
{
   let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(10),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(f.size());
    render_tabs(f, chunks[0], app_inst);
    render_main_space(f, chunks[1], app_inst);
}

fn render_tabs(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, area: Rect, app_inst: &mut app::App)
{
   let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(30),
                Constraint::Percentage(40),
                Constraint::Percentage(30),
            ].as_ref()
        )
        .split(area);

    let block = Block::default().borders(Borders::ALL);
    f.render_widget(block, area);
    render_space_info(f, chunks[0], app_inst);
    render_spaces(f, chunks[1], app_inst);
    render_profile(f, chunks[2], app_inst);
}

fn render_space_info(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, area: Rect, app_inst: &mut app::App)
{
    let space_info = Block::default().title("Space Info").borders(Borders::ALL);
    f.render_widget(space_info, area);
}
fn render_spaces(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, area: Rect, app_inst: &mut app::App)
{
    let spaces = Block::default().title("Spaces").borders(Borders::ALL);
    f.render_widget(spaces, area);
}
fn render_profile(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, area: Rect, app_inst: &mut app::App)
{
    let block = Block::default().title("Profile").borders(Borders::ALL);
    let txt = Paragraph::new(app_inst.profile.name.clone()).block(block);
    f.render_widget(txt, area);
}

fn render_main_space(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, area: Rect, app_inst: &mut app::App)
{
   let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(80),
            ].as_ref()
        )
        .split(area);
   let l_block = Block::default().title("Channels").borders(Borders::ALL);
   f.render_widget(l_block, chunks[0]);
   let m_block = Block::default().title("Text").borders(Borders::ALL);
   f.render_widget(m_block, chunks[1]);
}
