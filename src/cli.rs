use std::{
    os::fd::{IntoRawFd, RawFd},
    path::PathBuf,
};

use clap::Parser;
use nix::sys::socket::{socketpair, AddressFamily, SockFlag, SockType};
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

pub fn generate_socketpair() -> Result<(RawFd, RawFd), Errcode> {
    let sp = socketpair(
        AddressFamily::Unix,
        SockType::SeqPacket,
        None,
        SockFlag::SOCK_CLOEXEC,
    )?;

    Ok((sp.0.into_raw_fd(), sp.1.into_raw_fd()))
}
