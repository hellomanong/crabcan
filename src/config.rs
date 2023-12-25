use std::{ffi::CString, os::unix::io::RawFd, path::PathBuf};

use tracing::debug;

use crate::{cli::generate_socketpair, errors::Errcode, hostname::generate_hostname};

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub fd: RawFd,
    pub hostname: String,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socketpair()?;
        debug!("Get socket pair: {}, {}", sockets.0, sockets.1);

        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|v| CString::new(v).expect("Cannot read arg"))
            .collect();

        let path = argv[0].clone();
        let hostname = generate_hostname()?;
        Ok((
            Self {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.1.clone(),
                hostname,
            },
            sockets,
        ))
    }
}
