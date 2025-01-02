#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vm::VM;

enum Opcode {
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
	NOOP,
}

fn get_opcode(instr: u16) -> Opcode {
	match instr >> 12 {
		0 => Opcode::BR,
    1 => Opcode::ADD,
    2 => Opcode::LD,
    3 => Opcode::ST,
    4 => Opcode::JSR,
    5 => Opcode::AND,
    6 => Opcode::LDR,
    7 => Opcode::STR,
    8 => Opcode::RTI,
    9 => Opcode::NOT,
    10 => Opcode::LDI,
    11 => Opcode::STI,
    12 => Opcode::JMP,
    13 => Opcode::RES,
    14 => Opcode::LEA,
		15 => Opcode::TRAP,
		_ => Opcode::NOOP, 
	}
}

pub fn execute_opcode(vm: &mut VM) {
	let instr = vm.memory.get(vm.registers.pc);
	let opcode = get_opcode(instr);
	match opcode {
    Opcode::ADD => noop(instr, vm),
    Opcode::AND => noop(instr, vm),
    Opcode::NOT => noop(instr, vm),
    Opcode::BR => noop(instr, vm),
    Opcode::JMP => noop(instr, vm),
  	Opcode::JSR => noop(instr, vm),
    Opcode::LD => noop(instr, vm),
    Opcode::LDI => noop(instr, vm),
    Opcode::LDR => noop(instr, vm),
    Opcode::LEA => noop(instr, vm),
    Opcode::ST => noop(instr, vm),
    Opcode::STI => noop(instr, vm),
    Opcode::STR => noop(instr, vm),
    Opcode::TRAP => noop(instr, vm),
    _ => {}
  }
}

fn noop(instr: u16, vm: &mut VM) {}


