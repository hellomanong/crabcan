use std::{
    fs::read_dir,
    path::{Path, PathBuf},
    thread::sleep,
    time::{self, Duration},
};

use nix::{
    mount::{mount, umount2, MntFlags, MsFlags},
    sched::{unshare, CloneFlags},
    unistd::{chdir, pivot_root},
};
use tracing::{debug, error};
use tracing_subscriber::fmt::format;

use crate::{
    errors::Errcode,
    utils::{create_directory, delete_dir, random_string},
};

// 如果命名空间是使用clone(2) 创建的，则子进程命名空间的挂载列表是父进程挂载命名空间中挂载列表的副本
pub fn set_mount_point(mount_bind: &PathBuf) -> Result<(), Errcode> {
    // 使用MS_PRIVATE重新挂载
    let flags = vec![MsFlags::MS_PRIVATE, MsFlags::MS_REC];
    let root = PathBuf::from("/");
    mount_dir(None, &root, None, flags)?;

    // let dir = PathBuf::from("/");
    // let dir = read_dir(dir).map_err(|e| {
    //     error!("---------err:{e}-----------");
    //     e
    // })?;
    // for d in dir.into_iter() {
    //     debug!("---------------dir--{d:?}--------------------");
    // }

    // 挂载一个内存文件系统，这样创建的临时目录，在退出后自动删除
    let ftype = PathBuf::from("tmpfs");
    let tmp = PathBuf::from("/tmp");
    mount_dir(None, &tmp, Some(&ftype), vec![])?;

    // sleep(Duration::from_secs(300));

    let new_root = PathBuf::from(format!("/tmp/crabcan. {}", random_string(12)));
    create_directory(&new_root)?;
    mount_dir(
        Some(mount_bind),
        &new_root,
        None,
        vec![MsFlags::MS_PRIVATE, MsFlags::MS_BIND],
    )?;

    let old_root = format!("oldroot. {}", random_string(6));
    let put_root = new_root.join(PathBuf::from(old_root.clone()));
    create_directory(&put_root)?;
    pivot_root(&new_root, &put_root).map_err(|e| {
        error!("-------pivot new_root: {new_root:?}, put_root: {put_root:?}, err: {e}");
        e
    })?;

    debug!("Unmounting old root");
    let old_root = PathBuf::from(format!("/{old_root}"));
    chdir(&PathBuf::from("/")).map_err(|e| {
        error!("Chdir root dir err: {e}");
        e
    })?;

    unmount_dir(&old_root)?;
    delete_dir(&old_root)?;

    Ok(())
}

fn mount_dir(
    source: Option<&PathBuf>,
    target: &PathBuf,
    ftype: Option<&PathBuf>,
    in_flags: Vec<MsFlags>,
) -> Result<(), Errcode> {
    let mut flags = MsFlags::empty();
    for f in in_flags.into_iter() {
        flags.insert(f)
    }

    mount::<PathBuf, PathBuf, PathBuf, PathBuf>(source, target, ftype, flags, None).map_err(
        |e| {
            error!(
                "Mount source: {:?}, target: {:?}, err: {e}",
                source.unwrap(),
                target
            );
            e
        },
    )?;
    Ok(())
}

fn unmount_dir(path: &PathBuf) -> Result<(), Errcode> {
    umount2(path, MntFlags::MNT_DETACH).map_err(|e| {
        error!("Unmount {path:?} err: {e}");
        e
    })?;

    Ok(())
}
