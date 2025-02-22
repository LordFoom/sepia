use clap::{Args, Parser};
#[derive(Debug, Parser)]
#[command(version = "0.2", about = "Screenshotting app to record  my days")]
pub struct AppArgs {
    #[arg(short, long)]
    pub verbose: bool,
    ///Time in seconds between screenshots
    #[arg(short, long)]
    pub time: Option<u64>,
}
