use core::panic;
use std::{fs, io::{self, Read}};

pub const MEM_SIZE: usize = u16::MAX as usize;

pub enum MemMapReg {
	Kbsr = 0xFE00,
	Kbdr = 0xFE02,
}

pub struct Memory {
	pub data: [u16; MEM_SIZE],
}

impl Memory {
	pub fn new() -> Memory {
	  Memory {
			data: [0; MEM_SIZE], // empty array of zeros
		}  
	}
	pub fn set(&mut self, addr: u16, value: u16) {
		if addr as usize >= MEM_SIZE {
			panic!("attemped to write to/past max memory address!");
		}
		self.data[addr as usize] = value
	}

	fn handle_keyboard(&mut self) {
		let mut buffer = [0; 1];
    	std::io::stdin().read_exact(&mut buffer).unwrap();
    	if buffer[0] != 0 {
        	self.set(MemMapReg::Kbsr as u16, 1 << 15);
        	self.set(MemMapReg::Kbdr as u16, buffer[0] as u16);
    	} else {
        	self.set(MemMapReg::Kbsr as u16, 0)
    	}
	}

	pub fn get(&mut self, addr: u16) -> u16 {
		if addr == MemMapReg::Kbsr as u16 {
			self.handle_keyboard();
		}
		self.data[addr as usize]
	}
	
	pub fn read_into(&mut self, path: String) -> u16 {
		// don't need byteorder crate!!
		let file = fs::File::open(path).expect("could not open file");
		let mut reader = io::BufReader::new(file);
		let mut buffer: [u8;2] = [0; 2];
		// code to read the base address (first word of the obj file)
		let _ = reader.read_exact(&mut buffer).expect("could not read file");
		let base_addr = u16::from_be_bytes(buffer);
		let mut addr = base_addr;
		loop {
		  match reader.read_exact(&mut buffer) {
		    Ok(()) => {
				let instruction = u16::from_be_bytes(buffer);
				self.set(addr, instruction as u16);
				addr += 1;
			}
			Err(e) => {
				if e.kind() == std::io::ErrorKind::UnexpectedEof {
					break;
				} else {
					panic!("could not read instruction {:?}", e);
				}
			}
		  }
		}
		base_addr
	}
}
