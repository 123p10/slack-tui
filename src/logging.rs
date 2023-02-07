use std::io::{Result};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
pub fn setup_logging() -> Result<()>
{
    let log_location = match home::home_dir(){
        Some(path) => path.join(".slack-tui.log"),
        None => panic!("Could not find home directory on system"),
    };
    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {l}: {m}\n")))
        .build(log_location).expect("Failed to setup log file, check permissions of directory");

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(log::LevelFilter::Info)).expect("Failed to initialize logger");

    log4rs::init_config(config).expect("Failed to initialize logger configuration");
    log::info!("Initialized Logging");
    Ok(())
}
