use std::io::{Result};
use reqwest::Client;

pub struct ProfileInfo {
    pub name: String,
    pub token: String,
}

pub struct Message {
    pub author: String,
    pub text: Option<String>,
    pub media: Option<String>,
    pub timestamp: u64
}

pub struct Channel {
    pub name: String,
    pub messages: Vec<Message> 
}

pub struct Space {
    pub name: String,
    pub channels: Vec<Channel>
}

pub struct App {
    pub running: bool,
    pub connected: bool,
    pub profile: Option<ProfileInfo>,
    pub current_space_index: Option<usize>,
    pub spaces: Vec<Space>,
    pub client: Client
}

const DEBUG: bool = false;
const ENV_VAR_SLACK: &str = "SLACK_TUI_TOKEN";

pub fn init_test_data() -> App
{
    App {
        running: true,
        connected: true,
        profile: Some(ProfileInfo {
            name: String::from("Test Name"),
            token: String::from("Test Token")
        }),
        spaces: vec![ 
            Space {
                name: String::from("Space 1"),
                channels: vec![
                    Channel {
                        name: String::from("Channel 1"),
                        messages: Vec::new()
                    },
                    Channel {
                        name: String::from("Channel 2"),
                        messages: Vec::new()
                    }
                ] 
            }
        ],
        current_space_index: Some(0),
        client: Client::new()
    }
}

pub fn init_real_data() -> App
{
    let profile: Option<ProfileInfo> = match std::env::var(ENV_VAR_SLACK)
    {
        Ok(val) => Some(ProfileInfo {
            name: String::from(""),
            token: val
        }),
        Err(e) => None
    };
    App {
        running: true,
        connected: false,
        spaces: Vec::new(),
        current_space_index: None,
        profile: profile,
        client: Client::new()
    }
}

pub fn init() -> App
{
    match DEBUG {
        true => init_test_data(),
        false => init_real_data()
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
