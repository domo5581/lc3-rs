#![allow(dead_code)]
#![allow(unused_variables)]

use crate::vm::vm::VM;

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
    Opcode::ADD => add(instr, vm),
    Opcode::AND => and(instr, vm),
    Opcode::NOT => noop(instr, vm),
    Opcode::BR => br(instr, vm),
    Opcode::JMP => jmp(instr, vm),
  	Opcode::JSR => noop(instr, vm),
    Opcode::LD => noop(instr, vm),
    Opcode::LDI => noop(instr, vm),
    Opcode::LDR => noop(instr, vm),
    Opcode::LEA => noop(instr, vm),
    Opcode::ST => noop(instr, vm),
    Opcode::STI => noop(instr, vm),
    Opcode::STR => noop(instr, vm),
    Opcode::TRAP => noop(instr, vm),
    Opcode::RTI => unimplemented!("RTI - privilege violation"),
    Opcode::RES => unimplemented!("RES - illegal instruction"),
    _ => {}
  }
}

fn sext(val: u16, bits: u8) -> u16 {
  // sign extension
  if (val >> (bits - 1)) & 1 == 1 {
    val | (0xffff << bits)
  } else {
    val
  }
}

fn noop(instr: u16, vm: &mut VM) {}

fn add(instr: u16, vm: &mut VM) {
  let dr = (instr >> 9) & 0b111;
  let sr1 = (instr >> 6) & 0b111;
  if (instr >> 5 & 1) == 0 {
    let sr2 = instr & 0b111;
    let res = vm.registers.get_register(sr1).wrapping_add(vm.registers.get_register(sr2));
    vm.registers.update_reg_and_cond(dr, res);
  } else {
    let imm5 = instr & 0b11111;
    let res = vm.registers.get_register(sr1).wrapping_add(sext(imm5, 5));
    vm.registers.update_reg_and_cond(dr, res);
  }
}

fn and(instr: u16, vm: &mut VM) {
  let dr = (instr >> 9) & 0b111;
  let sr1 = (instr >> 6) & 0b111;
  if (instr >> 5 & 1) == 0 {
    let sr2: u16 = instr & 0b111;
    let res: u16 = vm.registers.get_register(sr1) & vm.registers.get_register(sr2);
    vm.registers.update_reg_and_cond(dr, res);
  } else {
    let imm5: u16 = instr & 0b11111;
    let res: u16 = vm.registers.get_register(sr1) & sext(imm5, 5);
    vm.registers.update_reg_and_cond(dr, res);
  }
}

fn br(instr: u16, vm: &mut VM) {
  // branch
  let sext_pcoffset9: u16 = sext(instr & 0b111111111, 9);
  let instr_cond: u16 = (instr >> 9) & 0b111;
  if (instr_cond & vm.registers.pc != 0) || (instr_cond == 0b111) {
    vm.registers.pc = vm.registers.pc.wrapping_add(sext_pcoffset9);
  }
}

fn jmp(instr: u16, vm: &mut VM) {
  // ret is a special form of jmp where it jut loads the 7th register into pc
  let reg: u16 = (instr >> 6) & 0b111;
  vm.registers.pc = vm.registers.get_register(reg);
}

fn jsr(instr: u16, vm: &mut VM) {
  // jump to subrouting
  // jsr includes it's bigger brother jsr
  vm.registers.set_registers(7, vm.registers.pc);
  if (instr >> 10) & 1 == 0 {
    let baser: u16 = (instr >> 6) & 0b111;
    vm.registers.pc = vm.registers.get_register(baser);
  } else {
    let pcoffset11: u16 = sext(instr & 0b1111111111, 11);
    vm.registers.pc = vm.registers.pc.wrapping_add(pcoffset11);
  }
}

fn ld(instr: u16, vm: &mut VM) {
  let dr: u16 = (instr >> 9) & 0b111;
  let pcoffset9: u16 = instr & 0b11111111;
  let addr: u16 = vm.registers.pc.wrapping_add(sext(pcoffset9, 9));
  vm.registers.update_reg_and_cond(dr, vm.memory.get(addr));
}

fn ldi(instr: u16, vm: &mut VM) {
  // load indirect -> take a value of an address in memory and then set the register with that address value in memory
  let dr: u16 = (instr >> 9) & 0b111;
  let pcoffset9: u16 = instr & 0b11111111;
  let addr: u16 = vm.memory.get(vm.registers.pc.wrapping_add(sext(pcoffset9, 9)));
  vm.registers.update_reg_and_cond(dr, vm.memory.get(addr));
}

fn ldr(instr: u16, vm: &mut VM) {
  // load base and offset
  let soffset6: u16 = sext(instr & 0b111111, 6);
  let br: u16 = instr >> 6 & 0b111;
  let dr: u16 = (instr >> 9) & 0b111;
  let addr: u16 = vm.registers.get_register(br).wrapping_add(soffset6);
  vm.registers.update_reg_and_cond(dr, vm.memory.get(addr));
}

fn lea(instr: u16, vm: &mut VM) {
  let spcoffset9: u16 = sext(instr & 0b111111111, 9);
  let dr: u16 = (instr >> 9) & 0b111;
  // load effective address
  vm.registers.set_registers(dr, vm.registers.pc.wrapping_add(spcoffset9));
}

fn not(instr: u16, vm: &mut VM) {
  let dr: u16 = (instr >> 9) & 0b111;
  let sr: u16 = (instr >> 6) & 0b111;
  let val: u16 = !vm.registers.get_register(sr);
  vm.registers.update_reg_and_cond(dr, val);
}

fn st(instr: u16, vm: &mut VM) {
  // store
  let spcoffset9: u16 = sext(instr & 0b111111111, 9);
  let sr: u16 = instr >> 9 & 0b111;
  vm.memory.set(vm.registers.pc.wrapping_add(spcoffset9), sr);
}

fn sti(instr: u16, vm: &mut VM) {
  // store indirect
  let spcoffset9: u16 = sext(instr & 0b111111111, 9);
  let sr: u16 = instr >> 9 & 0b111;
  let addr: u16 = vm.memory.get(vm.registers.pc.wrapping_add(spcoffset9));
  vm.memory.set(addr, vm.registers.get_register(sr));
}

fn str(instr: u16, vm: &mut VM) {
  let sr: u16 = instr >> 9 & 0b111;
  let baser: u16 = instr >> 6 & 0b111;
  let offset6: u16 = instr & 0b111111;
  vm.memory.set(baser + sext(offset6, 6), vm.registers.get_register(sr));
}







