mod mem;
use std::env;
use std::fs;
use std::io;
use std::io::Read;

fn main() {
	let args: Vec<String> = env::args().collect();
	let	path = args.get(1).expect("enter a file please!");
	let file = fs::File::open(path).expect("couldnt open file");
	let mut reader = io::BufReader::new(file);
	let mut values = Vec::new();
	let mut buffer:[u8;2] = [0;2];
	while reader.read_exact(&mut buffer).is_ok() {
		let instruction = u16::from_be_bytes(buffer);
		values.push(instruction);
	}
	println!("data: {:?}\n", values);
	for data in values{
		println!("hex data: {:#06X}", data);
	}
}
