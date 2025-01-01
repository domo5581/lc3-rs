#![allow(dead_code)]

use std::{fs, io::{self, Read}};

pub const MEM_SIZE: usize = u16::MAX as usize;

pub struct Memory {
	pub data: [u16; MEM_SIZE],
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

	pub fn read(&mut self, path: String) {
		// don't need byteorder crate!!
		let file = fs::File::open(path).expect("could not open file");
		let mut reader = io::BufReader::new(file);
		let mut buffer: [u8;2] = [0, 2];
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
	}
}
