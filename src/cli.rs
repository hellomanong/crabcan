use std::path::PathBuf;

use clap::Parser;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::util::SubscriberInitExt;

use crate::errors::Errcode;

#[derive(Debug, Parser)]
#[command(author, version, about = "A simple container in Rust", long_about=None)]
pub struct Args {
    /// Activate debug mode
    #[arg(short, long)]
    debug: bool,
    /// Command to execute inside the container
    #[arg(short, long)]
    pub command: String,
    /// user ID to create inside the container
    #[arg(short, long)]
    pub uid: u32,
    /// Directory to mount as root of the container
    #[arg(short, long="mount", value_parser=clap::value_parser!(std::path::PathBuf))]
    pub mount_dir: PathBuf,
}

pub fn parse_args() -> Result<Args, Errcode> {
    let args = Args::parse();

    if args.debug {
        setup_log(LevelFilter::DEBUG);
    } else {
        setup_log(LevelFilter::INFO);
    }
    info!("{args:?}");

    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(Errcode::ArgumentInvalid("mount".into()));
    }

    Ok(args)
}

pub fn setup_log(filter: impl Into<LevelFilter>) {
    tracing_subscriber::fmt()
        .with_max_level(filter.into())
        .finish()
        .init();
}
