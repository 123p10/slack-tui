use std::io::{Result};

pub struct ProfileInfo {
    pub name: String
}
pub struct App {
    pub running: bool,
    pub profile: ProfileInfo
}

pub fn init() -> App
{
    App {
        running: true,
        profile: ProfileInfo {
            name: String::from("Test Name")
        }
    }
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
