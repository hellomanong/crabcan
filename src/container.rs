use std::os::fd::RawFd;

use nix::{
    sys::{utsname::uname, wait::waitpid},
    unistd::{close, Pid},
};
use tracing::{debug, error};

use crate::{
    child::generate_child_process,
    cli::Args,
    config::ContainerOpts,
    errors::Errcode,
    namespaces::handle_child_uid_map,
    resources::{clean_cgroups, restrict_resouces},
};

pub struct Container {
    sockets: (RawFd, RawFd),
    config: ContainerOpts,
    child_pid: Option<Pid>,
}

impl Container {
    pub fn new(args: Args) -> Result<Self, Errcode> {
        let (config, sockets) = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;
        Ok(Self {
            sockets,
            config,
            child_pid: None,
        })
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        let pid = generate_child_process(self.config.clone())?;
        restrict_resouces(&self.config.hostname, pid)?;
        handle_child_uid_map(pid, self.sockets.0)?;

        self.child_pid = Some(pid);
        debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        debug!("cleaning container");
        close(self.sockets.0)?;
        debug!("close write socket success");
        close(self.sockets.1)?;
        debug!("close read socket success");

        // wsl 中不知为啥无法使用cgroup2，暂时先不清理了
        // clean_cgroups(&self.config.hostname)?;
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

    debug!("Container child PID: {:?}", container.child_pid);
    wait_child(container.child_pid)?;

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

pub fn wait_child(pid: Option<Pid>) -> Result<(), Errcode> {
    if let Some(child_pid) = pid {
        debug!("Waiting for child (pid: {child_pid} to finish");
        if let Err(e) = waitpid(child_pid, None) {
            error!("Error while waiting for pid to finish: {e:?}");
            return Err(e.into());
        }
    }

    Ok(())
}
