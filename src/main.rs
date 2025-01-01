mod mem;
use core::panic;
use std::env;
use std::fs;
use std::io;
use std::io::Read;
use std::u16;

struct Registers {
	r0: u16,
	r1: u16,
	r2: u16,
	r3: u16,
	r4: u16,
	r5: u16,
	r6: u16,
	r7: u16,
	pc: u16,
	cond: u16,
}

impl Registers {
  fn new() -> Registers {
    Registers {
			r0: 0,
			r1: 0,
			r2: 0,
			r3: 0,
			r4: 0,
			r5: 0,
			r6: 0,
			r7: 0,
			pc: 0x300,
			cond: 0,
		}
  }

	// s/o dogeystamp for this cool register return logic
	// most other vms just updated it with a &mut self

	fn return_register(&mut self, num: u16) -> &mut u16 {
		match num {
			0 => &mut self.r0,
			1 => &mut self.r1,
			2 => &mut self.r2,
			3 => &mut self.r3,
			4 => &mut self.r4,
			5 => &mut self.r5,
			6 => &mut self.r6,
			7 => &mut self.r7,
			_ => panic!("not a register!"),
		}
	}
	
	fn get_register(&mut self, num: u16) -> u16 {
		*self.return_register(num)
	}

	fn set_registers(&mut self, num : u16, value: u16) {
		*self.return_register(num) = value;
	}
}





fn main() {










	// let args: Vec<String> = env::args().collect();
	// let	path = args.get(1).expect("enter a file please!");
	// let file = fs::File::open(path).expect("couldnt open file");
	// let mut reader = io::BufReader::new(file);
	// let mut values = Vec::new();
	// let mut buffer:[u8;2] = [0;2];
	// while reader.read_exact(&mut buffer).is_ok() {
	// 	let instruction = u16::from_be_bytes(buffer);
	// 	values.push(instruction);
	// }
	// println!("data: {:?}\n", values);
	// for data in values{
	// 	println!("hex data: {:#06X}", data);
	// }
}
