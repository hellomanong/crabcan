use std::{fs::File, io::Write, os::fd::RawFd};

use nix::{
    sched::{unshare, CloneFlags},
    unistd::{setgroups, setresgid, setresuid, Gid, Pid, Uid},
};
use tracing::{debug, info};

use crate::{
    errors::Errcode,
    ipc::{recv_bool, send_bool},
};

pub fn userns(fd: RawFd, uid: u32) -> Result<(), Errcode> {
    debug!("Setting up user namespace with UID {}", uid);
    let has_userns = match unshare(CloneFlags::CLONE_NEWUSER) {
        Ok(_) => true,
        Err(_) => false,
    };
    send_bool(fd, has_userns)?;
    recv_bool(fd)?;
    if has_userns {
        info!("User namespaces set up");
    } else {
        info!("User namespaces not supported, continuing...");
    }
    // Switch UID / GID with the one provided by the user

    debug!("Switching to uid: {} / gid: {}...", uid, uid);
    let gid = Gid::from_raw(uid);
    let uid = Uid::from_raw(uid);

    setgroups(&[gid])?;
    setresgid(gid, gid, gid)?;
    setresuid(uid, uid, uid)?;

    Ok(())
}

pub fn handle_child_uid_map(pid: Pid, fd: RawFd) -> Result<(), Errcode> {
    if recv_bool(fd)? {
        let mut uid_map = File::create(format!("/proc/{}/uid_map", pid.as_raw()))?;
        uid_map.write_all(b"0 10000 2000")?;

        let mut gid_map = File::create(format!("/proc/{}/gid_map", pid.as_raw()))?;
        gid_map.write_all(b"0 10000 2000")?;
    } else {
        info!("No user namespace set up from child process");
    }

    debug!("Child UID/GID map done, sending signal to child to continue...");
    send_bool(fd, false)
}
