extern crate strum;
#[macro_use]
extern crate strum_macros;

extern crate num_derive;

extern crate clap;
use clap::{Arg, App};

mod util;
mod interpreter;
mod vm;

fn main() {
	let matches = App::new("arm-vm")
						.version("1.0")
						.author("Kyle Horne <me@kyhorne.com>")
						.about("Virtualization of a 32-bit ARM-like processor with native execution.")
						.arg(Arg::with_name("repl")
							.short("r")
							.long("repl")
							.help("Runs the virtual machine in read-eval-print-loop (REPL) mode")
							.takes_value(false))
						.get_matches();
 	if matches.is_present("repl") {
		vm::Processor::new().repl();
	}
}
