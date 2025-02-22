mod args;

use args::AppArgs;
use chrono::Utc;
use clap::Parser;
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use log::{debug, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};
use std::{
    io::{stdin, Read},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use xcap::Monitor;

fn normalized(filename: &str) -> String {
    filename
        .replace("|", "")
        .replace("\\", "")
        .replace(":", "")
        .replace("/", "")
}

fn init_logging(verbose: bool) -> Result<()> {
    color_eyre::install()?;
    // Configure the console appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
        .build();

    // Configure the file appender
    let file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
        .build("./sepia.log")
        .expect("Failed to create file appender");

    let level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };
    // Build the `log4rs` configuration
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("file")
                .build(level),
        )
        .expect("Failed to build log4rs configuration");

    // Initialize the logger
    log4rs::init_config(config).expect("Failed to initialize logging");
    Ok(())
}

fn main() -> Result<()> {
    let args = AppArgs::parse();
    init_logging(args.verbose)?;
    let num_seconds_between_screen_shots = if let Some(user_time) = args.time {
        user_time
    } else {
        1
    };
    debug!("Let's get image capturing time!");
    let start = Instant::now();
    let monitors = Monitor::all().unwrap();

    //thread listens for quit command
    let (tx_exit, rx_exit) = mpsc::channel();
    thread::spawn(move || {
        let mut stdin = stdin();
        let mut buf = [0; 1];
        while stdin.read_exact(&mut buf).is_ok() {
            let ch = buf[0] as char;
            if tx_exit.send(ch).is_err() {
                debug!("Error break in quit thread");
                break;
            }
            if ch == 'q' {
                debug!("We got  that Q!");
                break;
            }
        }
    });
    println!("Press {} to exit", "q".bold().yellow());
    //main loop that takes screenshots
    loop {
        if let Ok(ch) = rx_exit.try_recv() {
            if ch == 'q' {
                println!("{}....!!", "Quitting".red().bold());
                break;
            }
        }
        let now = Utc::now();
        for monitor in monitors.clone() {
            let now_monitor = format!("{}{}", monitor.name(), now.to_string());
            let image = monitor.capture_image().unwrap();
            image.save(format!("monitor-{}.png", normalized(&now_monitor)))?;
        }
        thread::sleep(Duration::from_secs(num_seconds_between_screen_shots));
    }

    println!("Elapsed time: {:?}", start.elapsed());
    Ok(())
}
