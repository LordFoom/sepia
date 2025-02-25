use clap::{Args, Parser};
#[derive(Debug, Parser)]
#[command(version = "0.2", about = "Screenshotting app to record  my days")]
pub struct AppArgs {
    ///Time in seconds between screenshots
    #[arg(short, long)]
    pub time: Option<u64>,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub dir: Option<String>,
    ///Only take screenshots if the screen has changed, even if "time" seconds have passed
    #[arg[short, long]]
    pub motion_triggered: bool,
}
