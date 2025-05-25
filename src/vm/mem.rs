use core::panic;
use std::{fs, io::{self, Read}, time::Duration};
use crossterm::event::{poll, read, Event, KeyEvent, KeyCode, KeyEventKind};

pub const MEM_SIZE: usize = u16::MAX as usize;

pub enum KeyboardMappedReg {
	Kbsr = 0xFE00,
	Kbdr = 0xFE02,
}

pub struct Memory {
	pub data: [u16; MEM_SIZE],
	keyboard_ready: bool,
	keyboard_char: u16,
}

impl Memory {
	pub fn new() -> Memory {
	  Memory {
			data: [0; MEM_SIZE], // empty array of zeros
			keyboard_ready: false,
			keyboard_char: 0,
		}  
	}
	pub fn set(&mut self, addr: u16, value: u16) {
		if addr as usize >= MEM_SIZE {
			panic!("attemped to write to/past max memory address!");
		}
		self.data[addr as usize] = value
	}

	fn check_keyboard(&mut self) {
		if !self.keyboard_ready {
			if poll(Duration::from_millis(0)).unwrap() {
      			if let Ok(Event::Key(key_event)) = read() {
        			if key_event.kind == crossterm::event::KeyEventKind::Press {
						match key_event.code {
							KeyCode::Char(c) => {
								self.keyboard_char = c as u16;
								self.keyboard_ready = true;
							},
							KeyCode::Backspace => {
								self.keyboard_char = 0x08;
								self.keyboard_ready = true;
							},
							_ => {}
						}
					}
				}
			}
		}
	}

	pub fn get(&mut self, addr: u16) -> u16 {
		match addr {
			0xFE00 => {
				self.check_keyboard();
				if self.keyboard_ready {
					0x8000
				} else {
					0x0000
				}
			},
			0xFE02 => {
				self.check_keyboard();
				if self.keyboard_ready {
					let ch = self.keyboard_char;
					self.keyboard_ready = false;
					ch
				} else {
					0
				}
			},
			_ => self.data[addr as usize]
		}
	}
	
	pub fn read(&mut self, path: String) -> u16 {
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
