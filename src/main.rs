use clap::Parser;
use tokio::runtime::Runtime;
use track_creator::{config::Config, logger, tasks};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value = "config.toml")]
    config: String,
}

fn main() {
    logger::init_tracing();

    let cli = Cli::parse();
    let config = Config::from_file(&cli.config);

    let rt = Runtime::new().unwrap();
    rt.block_on(tasks::start_scheduler(config));
}
