use std::fs::{canonicalize, remove_dir};

use cgroups_rs::{cgroup_builder::CgroupBuilder, CgroupPid, MaxValue};
use nix::unistd::Pid;
use rlimit::{setrlimit, Resource};
use tracing::{debug, error, info};

use crate::errors::Errcode;

const KMEM_LIMIT: i64 = 1024 * 1024 * 1024;
const MEM_LIMIT: i64 = KMEM_LIMIT;
const MAX_PID: MaxValue = MaxValue::Value(64);
const NOFILE_RLIMIT: u64 = 64;

pub fn restrict_resouces(hostname: &String, pid: Pid) -> Result<(), Errcode> {
    debug!("Restricting resources for hostname {}", hostname);
    let h = cgroups_rs::hierarchies::auto();
    let cgs = CgroupBuilder::new(hostname)
        .cpu()
        .shares(256)
        .done()
        .memory()
        .kernel_memory_limit(KMEM_LIMIT)
        .memory_hard_limit(MEM_LIMIT)
        .done()
        .pid()
        .maximum_number_of_processes(MAX_PID)
        .done()
        .blkio()
        .weight(50)
        .done()
        .build(h)
        .unwrap();

    // info!("-------subsystems:{:?}-----", cgs.subsystems());

    let pid: u64 = pid.as_raw().try_into().unwrap();
    info!("----the pid is: {pid}----");
    cgs.add_task(CgroupPid::from(pid)).map_err(|e| {
        error!("Add task err: {e}");
        e
    })?;

    setrlimit(Resource::NOFILE, NOFILE_RLIMIT, NOFILE_RLIMIT).map_err(|e| {
        error!("Set rlimit err: {e}");
        e
    })?;

    Ok(())
}

pub fn clean_cgroups(hostname: &String) -> Result<(), Errcode> {
    debug!("Cleaning cgroups");
    let d = canonicalize(format!("/sys/fs/cgroup/{hostname}/")).map_err(|e| {
        error!("Error while canonicalize /sys/fs/cgroup/{hostname}, err: {e}");
        e
    })?;

    remove_dir(d).map_err(|e| {
        error!("Cgroups cleaning failed: {e}");
        e
    })?;

    Ok(())
}
