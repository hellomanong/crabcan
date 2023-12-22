use container::Container;
use errors::exit_with_return_code;

mod cli;
mod errors;
mod config;
mod container;
fn main() {
    match cli::parse_args() {
        Ok(args) => exit_with_return_code(container::start(args)),
        Err(e) => exit_with_return_code(Err(e)),
    }
}
