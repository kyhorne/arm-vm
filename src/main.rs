extern crate strum;
#[macro_use]
extern crate strum_macros;

mod util;
mod interpreter;

fn main() {
    interpreter::repl();
}
