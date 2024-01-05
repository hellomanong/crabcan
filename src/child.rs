use std::ffi::CString;

use nix::{
    sched::{clone, CloneFlags},
    sys::signal::Signal,
    unistd::{execve, Pid},
};
use tracing::{error, info};

use crate::{
    capabilities::setcapabilities, config::ContainerOpts, errors::Errcode,
    hostname::set_container_hostname, mount::set_mount_point, namespaces::userns,
    syscalls::setsyscalls,
};

fn child(config: ContainerOpts) -> isize {
    info!(
        "Starting container with command {} and args {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );

    match setup_container_configurations(&config) {
        Ok(_) => {}
        Err(e) => {
            error!("Set up container err: {e}");
            return -1;
        }
    }

    info!(
        "Starting container with command {} and args {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );

    match execve::<CString, CString>(&config.path, &config.argv, &[]) {
        Ok(_) => return 0,
        Err(e) => {
            error!("execve {:?} err: {e}", config.path);
            return 1;
        }
    }
}

const STACK_SIZE: usize = 1024 * 1024;
pub fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);

    let pid;

    unsafe {
        pid = clone(
            Box::new(|| child(config.clone())),
            &mut tmp_stack,
            flags,
            Some(Signal::SIGCHLD as i32),
        )?;
    }

    Ok(pid)
}

fn setup_container_configurations(config: &ContainerOpts) -> Result<(), Errcode> {
    set_container_hostname(&config.hostname)?;
    set_mount_point(&config.mount_dir, &config.addpaths)?;
    userns(config.fd, config.uid)?;
    setcapabilities()?;
    setsyscalls()?;
    Ok(())
}
