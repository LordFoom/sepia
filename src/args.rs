use clap::{Args, Parser};
#[derive(Debug, Parser)]
#[command(version = "0.1", about = "Screenshotting app to record  my days")]
pub struct AppArgs {
    ///Time in seconds between screenshots
    #[arg(short, long)]
    time: u8,
}
