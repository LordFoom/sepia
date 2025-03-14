use clap::Parser;
#[derive(Debug, Parser, Clone)]
#[command(version = "0.2", about = "Screenshotting app to record  my days")]
pub struct AppArgs {
    ///Time in seconds between screenshots
    #[arg(short, long)]
    pub time: Option<u64>,
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub dir: Option<String>,
    ///Only take screenshots if the screen has changed,
    #[arg(short, long)]
    pub motion_triggered: bool,
    ///Sensitivity of motion triggered, only trigger if 'difference' greater than this. Default is
    #[arg(short, long, requires = "motion_triggered")]
    pub sensitivity: Option<u32>,
}
