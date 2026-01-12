use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Enable Matrix-style display mode
    #[arg(long)]
    pub matrix: bool,

    /// Skip Unicode support check and start immediately
    #[arg(long, short)]
    pub quick: bool,

    /// Run in daemon mode (background process)
    #[arg(long, short)]
    pub daemon: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Args::parse()
    }
}
