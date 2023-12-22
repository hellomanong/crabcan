use std::{ffi::CString, path::PathBuf};

use crate::errors::Errcode;

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
}

impl ContainerOpts {
    pub fn new(command: String, uid: u32, mount_dir: PathBuf) -> Result<ContainerOpts, Errcode> {
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|v| CString::new(v).expect("Cannot read arg"))
            .collect();

        let path = argv[0].clone();
        Ok(Self{
            path,
            argv,
            uid,
            mount_dir,
        })
    }
}
