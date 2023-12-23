use clap::Error;
use nix::sys::utsname::uname;
use tracing::{debug, error};

use crate::{cli::Args, config::ContainerOpts, errors::Errcode};

pub struct Container {
    config: ContainerOpts,
}

impl Container {
    pub fn new(args: Args) -> Result<Self, Errcode> {
        let config = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;
        Ok(Self { config })
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        debug!("cleaning container");
        Ok(())
    }
}

pub fn start(args: Args) -> Result<(), Errcode> {

    debug!("----start----");
    check_linux_version()?;

    let mut container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        error!("Error while creating container: {:?}", e);
        return Err(e);
    }

    debug!("Finished, cleaning & exit");
    container.clean_exit()
}

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;
pub fn check_linux_version() -> Result<(), Errcode> {
    let uname = uname().expect("Get linux uname err:");
    
    let uname_str = uname.release().to_str().expect("Get linux release err:"); 
    debug!("Linux release: {:?}", uname_str);
    let version = scan_fmt::scan_fmt!(uname_str, "{f}.{}", f32).expect("Get linux version err:");
    if version < MINIMAL_KERNEL_VERSION {
        return Err(Errcode::NotSupported(0));
    }

    if uname.machine() != "x86_64" {
        return Err(Errcode::NotSupported(1));
    }

    Ok(())
}
