use std::io::{Result};
use std::time::{Duration, Instant};
mod app;
mod logging;
mod terminal;
const TICK_RATE_MS: u64 = 50;

fn main() -> Result<()> 
{
    logging::setup_logging()?;
    let mut term = terminal::setup_tui().expect("Failed to setup terminal");

    let mut app = app::App {
        running: true
    };

    let mut last_t = Instant::now();
    let tick_rate = Duration::from_millis(TICK_RATE_MS);
    log::info!("Starting Main Loop");
    loop {
        terminal::tick(&mut term, &mut app)?;
        if last_t.elapsed() >= tick_rate {
            app::tick();
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
