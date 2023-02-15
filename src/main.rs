use std::io::{Result};
use std::time::{Duration, Instant};
use tokio;
mod app;
mod logging;
mod terminal;
mod web_api;
const TICK_RATE_MS: u64 = 50;
const DEFAULT_TOKEN: &str = "xoxc-433343262982-759574558630-746415958595-60bdeaec9788d11832b04530e641f09f09586221b85c35b1cfa0aef8c1ab1810";

#[tokio::main]
async fn main() -> Result<()> 
{
    logging::setup_logging()?;
    let mut term = terminal::setup_tui().expect("Failed to setup terminal");

    let mut last_t = Instant::now();
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    log::info!("Initializing App");
    let mut app = app::init(); 
    log::info!("Starting Main Loop");
    web_api::get_profile_info(&mut app.client, String::from(DEFAULT_TOKEN)).await;
    loop {
        terminal::tick(&mut term, &mut app)?;
        if last_t.elapsed() >= tick_rate {
            app::tick()?;
            last_t = Instant::now();
        }
        if !app.running {
            break;
        }
    };

    terminal::destroy_tui(&mut term).expect("Failed to destruct terminal");

    log::info!("Ending Program");
    Ok(())
}
