use errors::exit_with_return_code;

mod capabilities;
mod child;
mod cli;
mod config;
mod container;
mod errors;
mod hostname;
mod ipc;
mod mount;
mod namespaces;
mod resources;
mod syscalls;
mod utils;
fn main() {
    match cli::parse_args() {
        Ok(args) => exit_with_return_code(container::start(args)),
        Err(e) => exit_with_return_code(Err(e)),
    }
}
