use std::os::fd::RawFd;

use bytes::BytesMut;
use nix::sys::socket::{send, MsgFlags, recv};
use tracing::{info, debug};

use crate::errors::Errcode;


pub fn send_bool(fd: RawFd, data: bool) -> Result<(), Errcode> {
    let data = [data.into()];
    let _num = send(fd, &data, MsgFlags::empty())?;
    // debug!("send num:{num}");
    Ok(())
}

pub fn recv_data(fd: RawFd) -> Result<bool, Errcode> {
    let mut data: [u8; 1] = [0];

    let _num = recv(fd, &mut data, MsgFlags::empty())?;
    // debug!("recv num:{num}");
    Ok(data[0] == 1)
}