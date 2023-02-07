use std::io::{Result};
pub struct App {
    pub running: bool,
}

pub fn tick() -> Result<()>
{
    Ok(())
}

fn quit(app: &mut App)
{
    app.running = false;
}

pub fn on_key(app: &mut App, c: char)
{
    match c {
        'q' => quit(app),
        _ => () 
    }
}
