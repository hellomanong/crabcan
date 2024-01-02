use libc::TIOCSTI;
use nix::{sched::CloneFlags, sys::stat::Mode};
use syscallz::{Action, Cmp, Comparator, Context, Syscall};
use tracing::{debug, error};

use crate::errors::Errcode;

pub fn setsyscalls() -> Result<(), Errcode> {
    debug!("");

    let mut ctx = Context::init_with_action(Action::Allow).map_err(|e| {
        error!("Init with allow action err: {e}");
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

    let s_isuid: u64 = Mode::S_ISUID.bits().into();
    let s_isgid: u64 = Mode::S_ISGID.bits().into();
    let clone_new_user: u64 = CloneFlags::CLONE_NEWUSER.bits() as u64;
    let syscalls_refuse_ifcomp = [
        (Syscall::chmod, 1, s_isuid),
        (Syscall::chmod, 1, s_isgid),
        (Syscall::fchmod, 1, s_isuid),
        (Syscall::fchmod, 1, s_isgid),
        (Syscall::fchmodat, 2, s_isuid),
        (Syscall::fchmodat, 2, s_isgid),
        (Syscall::unshare, 0, clone_new_user),
        (Syscall::clone, 0, clone_new_user),
        (Syscall::ioctl, 1, TIOCSTI),
    ];

    for (sc, ind, biteq) in syscalls_refuse_ifcomp.iter() {
        refuse_if_comp(&mut ctx, *ind, sc, *biteq)?;
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

fn refuse_if_comp(ctx: &mut Context, ind: u32, sc: &Syscall, biteq: u64) -> Result<(), Errcode> {
    ctx.set_rule_for_syscall(
        Action::Errno(EPERM),
        *sc,
        &[Comparator::new(ind, Cmp::MaskedEq, biteq, Some(biteq))],
    )
    .map_err(|e| {
        error!("Set rule action for syscall err: {e}");
        e
    })?;
    Ok(())
}
