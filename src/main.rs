use errors::exit_with_return_code;

mod child;
mod cli;
mod config;
mod container;
mod errors;
mod ipc;
fn main() {
    match cli::parse_args() {
        Ok(args) => exit_with_return_code(container::start(args)),
        Err(e) => exit_with_return_code(Err(e)),
    }
}
