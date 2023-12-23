use std::{ffi::CString, os::fd::RawFd, path::PathBuf};

use crate::{cli::generate_socketpair, errors::Errcode};

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub fd: RawFd,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socketpair()?;

        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|v| CString::new(v).expect("Cannot read arg"))
            .collect();

        let path = argv[0].clone();
        Ok((
            Self {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.0.clone(),
            },
            sockets
        ))
    }
}
