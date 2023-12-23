use nix::{unistd::Pid, sched::{CloneFlags, clone}, sys::signal::Signal};
use tracing::info;

use crate::{config::ContainerOpts, errors::Errcode};


fn child(config: ContainerOpts) -> isize {
    info!("Starting container with command {} and args {:?}", 
    config.path.to_str().unwrap(), config.argv);

    0
}

const STACK_SIZE: usize = 1024* 1024;
fn generate_child_process(config: ContainerOpts) -> Result<Pid, Errcode> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();

    let pid ;
    
    unsafe {
        pid = clone(
            Box::new(|| child(config.clone())), 
            &mut tmp_stack,
            flags, 
            Some(Signal::SIGCHLD as i32))?;
    }
    
    Ok(pid)
}