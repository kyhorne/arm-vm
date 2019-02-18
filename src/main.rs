extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use] extern crate log;
extern crate env_logger;

mod util;
mod interpreter;

fn main() {
    interpreter::repl();
}
