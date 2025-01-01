#![allow(dead_code)]

mod mem;
use core::panic;
// use std::env;
// use std::fs;
// use std::io;
// use std::io::Read;
use std::{env, u16};
use mem::Memory;

const PC_START: u16 = 0x300;

enum Opcodes {
    BR = 0,
    ADD,
    LD,
    ST,
    JSR,
    AND,
    LDR,
    STR,
    RTI,
    NOT,
    LDI,
    STI,
    JMP,
    RES,
    LEA,
    TRAP,
}

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

enum ConditionFlags {
	// condition flags can only be set as nzp with a 1 in each position
	POS = 1 << 0,
	ZERO = 1 << 1,
	NEG = 1 << 2,
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
			pc: PC_START,
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
			8 => &mut self.pc,
			9 => &mut self.cond,
			_ => panic!("not a register!"),
		}
	}
	
	fn get_register(&mut self, num: u16) -> u16 {
		*self.return_register(num)
	}

	fn set_registers(&mut self, num : u16, value: u16) {
		*self.return_register(num) = value;
	}

	fn update_cond(&mut self, num : u16) {
		if *self.return_register(num) > 0 {
			self.cond = ConditionFlags::POS as u16;
		} else if *self.return_register(num) == 0 {
			self.cond = ConditionFlags::ZERO as u16;
		}	else {
		  self.cond = ConditionFlags::NEG as u16
		}
	}
	
	fn update_reg_and_cond(&mut self, num : u16, value: u16) {
		self.set_registers(num, value);
		self.update_cond(num);
	}
}

enum State {
  Stopped = 0,
	Running,
}

struct VM {
	memory: Memory,
	registers: Registers,
	state: State,
}

impl VM {
	fn new() -> VM {
		VM{
		memory: Memory::new(),
		registers: Registers::new(),
		state: State::Stopped,
		}
	}
}



fn main() {
	let mut lc3 = VM::new();
	let args: Vec<String> = env::args().collect();
	let path = args.get(1).expect("a file must be specified");
	lc3.memory.read(path.to_string());
	for addr in 0x3000..0x3007 {
		println!("the instruction is {:#b} at address {:#x}", lc3.memory.data[addr], addr);
	}
}
