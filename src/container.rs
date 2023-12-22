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
    let mut container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        error!("Error while creating container: {:?}", e);
        return Err(e);
    }

    debug!("Finished, cleaning & exit");
    container.clean_exit()
}
