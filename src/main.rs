#![allow(dead_code)]

mod vm;
use mem;

fn main() {
	let lc3 = vm::VM::new();
	// let mut lc3 = VM::new();
	// let args: Vec<String> = env::args().collect();
	// let path = args.get(1).expect("a file must be specified");
	// lc3.memory.read(path.to_string());
	// lc3.execute();
}
