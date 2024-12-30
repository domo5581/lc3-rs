#[allow(dead_code)]
enum Registers {
	R0 = 0,
	R1,
	R2,
	R3,
	R4,
	R5,
	R6,
	R7,
	PC,
	COND,
	COUNT
}

#[allow(dead_code)]
enum Opcodes {
	// https://www.cs.utexas.edu/~fussell/courses/cs310h/lectures/Lecture_10-310h.pdf
	ADD = 0, // add
	AND, // and
	NOT, // not (bitwise)
	LD, // load (pc relative)
	LDI, // load indirect
	LDR, // load base + offset
	LEA, // load immediate
	ST, // store pc relative
	STR, // store base + offset
	STI, // store indirect
	BR, // branch
	JSR, // jump register
	JMP, // jump
	RTI, // return from interrupt (unused)
	TRAP, // trap
}

#[allow(dead_code)]
enum ConditionFlags {
	POS = 1 << 0,
	ZERO = 1 << 1,
	NEG = 1 << 2,
}

fn main() {
	let _memory:[u16; 1 << 16];
	let _registers:[u16; Registers::COUNT as usize];
	
}
