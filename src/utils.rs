use rand::Rng;
use tracing::error;
pub fn random_string(n: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    let name: String = (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    name
}

use std::{
    fs::{create_dir_all, remove_dir},
    path::PathBuf,
};

use crate::errors::Errcode;
pub fn create_directory(path: &PathBuf) -> Result<(), Errcode> {
    create_dir_all(path).map_err(|e| {
        error!("Cannot create directory {}: {}", path.to_str().unwrap(), e);
        e
    })?;
    Ok(())
}

pub fn delete_dir(path: &PathBuf) -> Result<(), Errcode> {
    remove_dir(path.as_path()).map_err(|e| {
        error!("Cannot del directory {}: {}", path.to_str().unwrap(), e);
        e
    })?;
    Ok(())
}
