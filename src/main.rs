#![allow(dead_code)]
#![allow(unused_variables)]

mod vm;
use crate::vm::vm::*;
use crossterm::terminal::{self, disable_raw_mode};
use ctrlc;
use std::{process, env};


fn setup_terminal() {
	terminal::enable_raw_mode().expect("could not enable raw mode");
	ctrlc::set_handler(|| {
		disable_raw_mode().ok();
		process::exit(130); // ctrl+C exit code
	}).expect("could not set ctrl+c handler")
}

fn main() {
	// setup terminal
	setup_terminal();
	// setup lc3 and start executing
	let mut lc3: VM = VM::new();
	let args: Vec<String> = env::args().collect();
	let path = args.get(1).expect("a file must be specified");
	let initial_pc = lc3.memory.read(path.to_string());
	lc3.registers.pc = initial_pc;
	lc3.execute();
	// restore term state
	crossterm::terminal::disable_raw_mode().ok();
}
