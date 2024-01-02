use syscallz::{Action, Context, Syscall};
use tracing::{debug, error};

use crate::errors::Errcode;

pub fn setsyscalls() -> Result<(), Errcode> {
    debug!("");

    let mut ctx = Context::init_with_action(Action::Allow).map_err(|e| {
        error!("Init with action err: {e}");
        e
    })?;

    let syscalls_refused = [
        Syscall::keyctl,
        Syscall::add_key,
        Syscall::request_key,
        Syscall::mbind,
        Syscall::migrate_pages,
        Syscall::move_pages,
        Syscall::set_mempolicy,
        Syscall::userfaultfd,
        Syscall::perf_event_open,
    ];

    for sc in syscalls_refused.iter() {
        refuse_syscall(&mut ctx, sc)?
    }

    ctx.load()?;
    Ok(())
}

const EPERM: u16 = 1;
fn refuse_syscall(ctx: &mut Context, sc: &Syscall) -> Result<(), Errcode> {
    ctx.set_action_for_syscall(Action::Errno(EPERM), *sc)
        .map_err(|e| {
            error!("Set action for syscall err: {e}");
            e
        })?;

    Ok(())
}
