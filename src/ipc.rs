use std::os::fd::RawFd;

use nix::sys::socket::{recv, send, MsgFlags};
use tracing::debug;

use crate::errors::Errcode;

pub fn send_bool(fd: RawFd, data: bool) -> Result<(), Errcode> {
    let data = [data.into()];
    let _num = send(fd, &data, MsgFlags::empty())?;
    // debug!("send num:{num}");
    Ok(())
}

pub fn recv_bool(fd: RawFd) -> Result<bool, Errcode> {
    let mut data: [u8; 1] = [0];

    let res = recv(fd, &mut data, MsgFlags::empty()).map_or_else(
        |e| {
            debug!("Recv err: {e}");
            Err(e)
        },
        |_| Ok(data[0] == 1),
    )?;

    Ok(res)
}
