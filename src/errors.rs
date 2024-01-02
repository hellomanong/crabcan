use std::{io, process::exit};

use nix::errno;
use thiserror::Error;
use tracing::{debug, error};

#[derive(Debug, Error)]
pub enum Errcode {
    #[error("ArgumentInvalid: {0}")]
    ArgumentInvalid(String),
    #[error("The linux version NotSupported: return: {0}")]
    NotSupported(u8),
    #[error("The system error: {0}")]
    SystemError(#[from] errno::Errno),
    #[error("Get rand error")]
    RngError,
    #[error("Io err: {0}")]
    IoError(#[from] io::Error),
    #[error("Syscall err: {0}")]
    SyscallsError(#[from] syscallz::Error),
    #[error("Cgroup err: {0}")]
    CgroupError(#[from] cgroups_rs::error::Error),
}

impl Errcode {
    pub fn get_return_code(&self) -> i32 {
        1
    }
}

pub fn exit_with_return_code(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            debug!("Exit without any error, returning 0");
            exit(0);
        }
        Err(e) => {
            let code = e.get_return_code();
            error!("Exit with err:{e}, return:{code}");
            exit(code);
        }
    }
}

// impl Display for Errcode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::ArgumentInvalid(v) => write!(f, "ArgumentInvalid: {}", v),
//             _ => write!(f, "{:?}", self),
//         }
//     }
// }
