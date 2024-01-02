use nix::{
    sched::{clone, CloneFlags},
    sys::signal::Signal,
    unistd::Pid,
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

    0
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
    set_mount_point(&config.mount_dir)?;
    userns(config.fd, config.uid)?;
    setcapabilities()?;
    setsyscalls()?;
    Ok(())
}
