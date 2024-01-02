use crate::errors::Errcode;
use nix::unistd::sethostname;
use rand::{seq::SliceRandom, Rng};
use tracing::debug;

const HOSTNAME_NAMES: [&str; 8] = [
    "cat", "world", "coffee", "girl", "man", "book", "pinguin", "moon",
];
const HOSTNAME_ADJ: [&str; 16] = [
    "blue",
    "red",
    "green",
    "yellow",
    "big",
    "small",
    "tall",
    "thin",
    "round",
    "square",
    "triangular",
    "weird",
    "noisy",
    "silent",
    "soft",
    "irregular",
];

pub fn generate_hostname() -> Result<String, Errcode> {
    let mut rng = rand::thread_rng();
    let num: u8 = rng.gen();
    let name = HOSTNAME_NAMES.choose(&mut rng).ok_or(Errcode::RngError)?;
    let adj = HOSTNAME_ADJ.choose(&mut rng).ok_or(Errcode::RngError)?;
    Ok(format!("{adj}-{name}-{num}"))
}

pub fn set_container_hostname(hostname: &String) -> Result<(), Errcode> {
    sethostname(hostname).map_err(|e| {
        debug!("Container hostname set failed, err:{:?}", e);
        e
    })?;
    debug!("Container hostname is now {hostname}");
    Ok(())
}
