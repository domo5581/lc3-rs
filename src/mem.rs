#![allow(dead_code)]

pub const MEM_SIZE: usize = u16::MAX as usize;

pub struct Memory {
	data: [u16; MEM_SIZE],
}

impl Memory {
	pub fn new() -> Memory {
	  Memory {
			data: [0; MEM_SIZE],
		}  
	}
	pub fn set(&mut self, addr: u16, value: u16) {
		self.data[addr as usize] = value;
	}

	pub fn get(&mut self, addr: u16) {
		self.data[addr as usize];
	}
}
