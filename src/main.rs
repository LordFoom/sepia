mod args;

use args::AppArgs;
use chrono::Utc;
use clap::Parser;
use color_eyre::{eyre::Result, owo_colors::OwoColorize};
use image::{DynamicImage, ImageReader};
use log::{debug, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    Config,
};

use std::{
    collections::HashMap,
    io::{stdin, Read},
    path::Path,
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

//TODO add 'sensitivity' flag to control pHash difference needed for save
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

    let monitors = Monitor::all().unwrap();
    //let us get the possible dir
    let storage_dir = get_storage_dir(args);

    //take an initial screenshot for comparison
    let mut baseline_images = take_screenshot(&monitors, &storage_dir)?;
    println!("Press {} to exit", "q".bold().yellow());
    //TODO replace this with an arg, also tune it, and don't forget to drink apple juice
    let sensitivity = 100;
    //main loop that takes screenshots
    loop {
        if let Ok(ch) = rx_exit.try_recv() {
            if ch == 'q' {
                println!("{}....!!", "Quitting".red().bold());
                break;
            }
        }
        let now = Utc::now();
        let mut new_screen_shots = take_screenshot(&monitors, &storage_dir)?;
        //now we compare the pHash of each image with the baseline
        let change_figure = difference_from_baseline(&baseline_images, &new_screen_shots)?;
        //if it has changed enough, we keep the newly created image, and replace the baseline to
        //the new image
        //if not we delete the new images and keep the old baseline images
        thread::sleep(Duration::from_secs(num_seconds_between_screen_shots));
    }

    println!("Elapsed time: {:?}", start.elapsed());
    Ok(())
}

///Determine requested storage dir and create if needed
fn get_storage_dir(args: AppArgs) -> String {
    if let Some(dir) = args.dir {
        //does the directory exist?
        let storage_dir = Path::new(&dir);
        if storage_dir.exists() {
            if storage_dir.is_file() {
                panic!(
                    "Woah! Cannot store pics in a file, must be a dir, this is a file: {}",
                    storage_dir.to_str().unwrap_or("UNKNOWN")
                )
            }
            let mut dir_path = storage_dir.to_str().unwrap_or("./").to_string();
            if !dir_path.ends_with('/') {
                dir_path.push('/');
            }
            dir_path
        } else {
            //no, then let us create it
            std::fs::create_dir(storage_dir).unwrap();
            storage_dir.to_str().unwrap_or("./").to_string()
        }
    } else {
        //current directory is the default
        "./".to_string()
    }
}

///We compare the images in the map and we return their pHash differences.
///Return a map of MonitorName->DifferenceScore
fn difference_from_baseline(
    baseline_images: &HashMap<String, String>,
    new_screen_shots: &HashMap<String, String>,
) -> Result<HashMap<String, String>> {
    todo!()
}

///Take a screenshot, return map of monitor name => screen shot path
fn take_screenshot(monitors: &Vec<Monitor>, storage_dir: &str) -> Result<HashMap<String, String>> {
    let now = Utc::now();
    let mut monitor_screenshots = HashMap::new();
    for monitor in monitors.clone() {
        let now_monitor_name = monitor.name().to_string();
        let now_monitor = format!("{}{}", now_monitor_name, now.to_string());
        let screen_shot = monitor.capture_image().unwrap();
        let path = format!("{}monitor-{}.png", storage_dir, normalized(&now_monitor));
        //monitor_screenshots.insert(path, )
        screen_shot.save(&path)?;
        //now we load the image we just created using 'image' lib
        //let just_saved_img = image::ImageReader::open(&path)?.decode()?;
        //monitor_screenshots.insert(now_monitor_name, just_saved_img);
    }
    //replace this with the actual hashmap
    Ok(monitor_screenshots)
}
