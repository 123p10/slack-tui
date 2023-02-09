use std::io::{Result};
use std::time::Duration;
use tui::{
    backend::{CrosstermBackend},
    Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, List, ListItem},
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
const MAX_SPACE_NAME: usize = 25;
const MAX_CHANNEL_NAME: usize = 25;

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
    match app_inst.connected
    {
        true => render_connected(f, app_inst),
        false => render_disconnected(f, app_inst)
    }
}

fn render_disconnected(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app_inst: &mut app::App)
{
    
    let block = Block::default().borders(Borders::ALL).title("slack-tui");
    let txt = Paragraph::new("Disconnected").block(block);
    f.render_widget(txt, f.size());
}

fn render_connected(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, app_inst: &mut app::App)
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
    let block = Block::default().title("Spaces").borders(Borders::ALL);
    let mut list_items = Vec::with_capacity(app_inst.spaces.len());
    for space in app_inst.spaces.iter() {
        if space.name.len() <= MAX_SPACE_NAME
        {
            let name = String::from(&space.name);
            let item = ListItem::new(name);
            list_items.push(item);
        }
    }
    let list = List::new(list_items).block(block);
    f.render_widget(list, area);
}
fn render_profile(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, area: Rect, app_inst: &mut app::App)
{
    let block = Block::default().title("Profile").borders(Borders::ALL);
    match &app_inst.profile
    {
        Some(profile) => {    
            let txt = Paragraph::new(profile.name.clone()).block(block);
            f.render_widget(txt, area);
        }
        None => {
            f.render_widget(block, area);
        }
    }
}

fn render_channel_list(f: &mut Frame<CrosstermBackend<std::io::Stdout>>, area: Rect, app_inst: &mut app::App)
{ 
    let block = Block::default().title("Channels").borders(Borders::ALL);
    let space_index = app_inst.current_space_index;
    match space_index {
        Some(idx) => {
            let mut list_items = Vec::with_capacity(app_inst.spaces[idx].channels.len());
            for channel in app_inst.spaces[idx].channels.iter() {
                if channel.name.len() <= MAX_CHANNEL_NAME
                {
//                    let name = String::from(&channel.name);
                    let item = ListItem::new(channel.name.clone());
                    list_items.push(item);
                }
            }
            let list = List::new(list_items).block(block).highlight_symbol(">>");
            f.render_widget(list, area);
        },
        None => ()
    }
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
   render_channel_list(f, chunks[0], app_inst);
   let m_block = Block::default().title("Text").borders(Borders::ALL);
   f.render_widget(m_block, chunks[1]);
}
