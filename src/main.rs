#![allow(dead_code)]
#![allow(unused_variables)]

mod vm;
use std::env;
use crate::vm::vm::*;


fn main() {
	let mut lc3: VM = VM::new();
	let args: Vec<String> = env::args().collect();
	let path = args.get(1).expect("a file must be specified");
	let initial_pc = lc3.memory.read_into(path.to_string());
	lc3.registers.pc = initial_pc;
	lc3.execute();
	// restore term state
	println!();
}
