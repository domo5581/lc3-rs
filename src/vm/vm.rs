#![allow(dead_code)]
#![allow(unused)]

use crate::vm::mem;
use crate::vm::isa;

const PC_START: u16 = 0x300;

pub struct Registers {
	r0: u16,
	r1: u16,
	r2: u16,
	r3: u16,
	r4: u16,
	r5: u16,
	r6: u16,
	r7: u16,
	pub pc: u16,
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

	pub fn return_register(&mut self, num: u16) -> &mut u16 {
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
	
	pub fn get_register(&mut self, num: u16) -> u16 {
		*self.return_register(num)
	}

	pub fn set_registers(&mut self, num : u16, value: u16) {
		*self.return_register(num) = value;
	}

	pub fn update_cond(&mut self, num : u16) {
		let value = self.get_register(num) as i16;
		if value > 0 {
			self.cond = ConditionFlags::POS as u16;
		} else if value == 0 {
			self.cond = ConditionFlags::ZERO as u16;
		} else {
		  self.cond = ConditionFlags::NEG as u16
		}
	}
	
	pub fn update_reg_and_cond(&mut self, num : u16, value: u16) {
		self.set_registers(num, value);
		self.update_cond(num);
	}
}


pub struct VM {
	 pub memory: mem::Memory,
	 pub registers: Registers,
	 pub running: bool, // is the vm running or not?
}

impl VM {
	 pub fn new() -> VM {
		VM{
			memory: mem::Memory::new(),
			registers: Registers::new(),
			running: false,
		}
	}

	pub fn state(&mut self, state: bool) {
		self.running = state;
	}

	pub fn execute(&mut self) {
		self.running = true;
		while (self.running) {
			if self.registers.pc as usize >= mem::MEM_SIZE - 1 {
				self.running = false;
				print!("{}", "vm has read past memory size")
			} else {
				isa::execute_opcode(self); // executes the opcode at the vm's pc
				self.registers.pc += 1;
			}
		}
	}
}

