#![allow(dead_code)]
#![allow(unused_variables)]

mod vm;
use crate::vm::vm::*;
use crossterm::terminal::{self, disable_raw_mode};
use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use std::{process, env, time::Duration, thread};

fn setup_terminal() {
	terminal::enable_raw_mode().expect("could not enable raw mode");
	// create new thread for ctrlc - raw mode blocks ctrl c signals
	thread::spawn(|| {
		loop {
			if poll(Duration::from_millis(16)).unwrap() {
				if let Ok(Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers, .. })) = read() {
                    if modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                        disable_raw_mode().ok();
						println!();
                        process::exit(130);
					}
				}
			}
		}
	});
}

fn main() {
	// setup terminal
	setup_terminal();
	// setup lc3 and start executing
	let mut lc3: VM = VM::new();
	let args: Vec<String> = env::args().collect();
	let path = args.get(1).expect("a file must be specified");
	let initial_pc = lc3.memory.read(path.to_string());
	lc3.registers.pc = initial_pc;
	lc3.execute();
	// restore term state
	println!();
	crossterm::terminal::disable_raw_mode().ok();
}
