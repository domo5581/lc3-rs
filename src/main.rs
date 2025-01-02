#![allow(dead_code)]
#![allow(unused_variables)]
mod vm;

fn main() {
	let lc3 = vm::VM::new();
	// let mut lc3 = VM::new();
	// let args: Vec<String> = env::args().collect();
	// let path = args.get(1).expect("a file must be specified");
	// lc3.memory.read(path.to_string());
	// lc3.execute();
}
