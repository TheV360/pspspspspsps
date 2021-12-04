use std::convert::TryFrom;
use std::io::Read;

const MINIMUM_LETTERS: usize = 3;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum CatInstruction {
	MoveRight,	// psp
	MoveLeft,	// psps
	Increment,	// pspsp
	Decrement,	// pspsps
	JumpForward,// pspspsp
	JumpBack,	// pspspsps
	Output,		// pspspspsp
	Input,		// pspspspsps
}
impl CatInstruction {
	pub fn pspspspsps_to_vec(code: &str) -> Vec<Self> {
		let mut instructions = Vec::new();
		
		for token in code.split_ascii_whitespace() {
			if let Ok(i) = CatInstruction::try_from(token) {
				instructions.push(i);
			}
		}
		
		instructions
	}
	pub fn pspspspsps_to_usize(s: &str) -> Result<usize, ()> {
		let mut result: usize = 0;
		let mut should_be_p_and_not_s = true;
		
		for c in s.chars() {
			if should_be_p_and_not_s && c == 'p'
			|| (!should_be_p_and_not_s) && c == 's' {
				result += 1;
				should_be_p_and_not_s = !should_be_p_and_not_s;
			} else {
				return Err(());
			}
		}
		
		Ok(result)
	}
}
impl TryFrom<usize> for CatInstruction {
	type Error = ();
	fn try_from(value: usize) -> Result<Self, Self::Error> {
		match value {
			0 => Ok(CatInstruction::MoveRight),
			1 => Ok(CatInstruction::MoveLeft),
			2 => Ok(CatInstruction::Increment),
			3 => Ok(CatInstruction::Decrement),
			4 => Ok(CatInstruction::JumpForward),
			5 => Ok(CatInstruction::JumpBack),
			6 => Ok(CatInstruction::Output),
			7 => Ok(CatInstruction::Input),
			_ => Err(()),
		}
	}
}
impl TryFrom<&str> for CatInstruction {
	type Error = ();
	fn try_from(value: &str) -> Result<Self, Self::Error> {
		if let Ok(result) = CatInstruction::pspspspsps_to_usize(value) {
			if result < MINIMUM_LETTERS {
				Err(())
			} else {
				CatInstruction::try_from(result - MINIMUM_LETTERS)
			}
		} else {
			Err(())
		}
	}
}

#[derive(Default)]
pub struct CatInterpreter {
	pub pointer: usize,
	pub pc: usize,
	pub tape: Vec<u8>,
	pub instructions: Vec<CatInstruction>,
	pub debug: bool,
}
impl CatInterpreter {
	pub fn new(tape_size: usize, instructions: Vec<CatInstruction>) -> Self {
		Self { pointer: 0, pc: 0, tape: vec![0u8; tape_size], instructions, debug: false }
	}
	pub fn step(&mut self) -> Result<(), &'static str> {
		if self.is_done() { return Err("Program is done.") }
		match self.instructions[self.pc] {
			CatInstruction::MoveRight => self.pointer = (self.pointer + 1) % self.tape.len(),
			CatInstruction::MoveLeft => if self.pointer > 0 { self.pointer = (self.pointer - 1) % self.tape.len() } else { self.pointer = self.tape.len() - 1 },
			CatInstruction::Increment => self.tape[self.pointer] = self.tape[self.pointer].wrapping_add(1),
			CatInstruction::Decrement => self.tape[self.pointer] = self.tape[self.pointer].wrapping_sub(1),
			CatInstruction::Output => print!("{}", if self.tape[self.pointer].is_ascii() { self.tape[self.pointer] as char } else { std::char::REPLACEMENT_CHARACTER }),
			CatInstruction::Input => self.tape[self.pointer] = std::io::stdin().lock().bytes().next().and_then(|r| {r.ok()}).unwrap(),
			CatInstruction::JumpForward => if self.tape[self.pointer] == 0 {
				let mut depth = 0usize;
				loop {
					self.pc += 1;
					if self.pc > self.instructions.len() {
						return Err("Couldn't find JumpBack instruction for JumpForward.");
					}
					if self.instructions[self.pc] == CatInstruction::JumpForward {
						depth += 1;
					}
					if self.instructions[self.pc] == CatInstruction::JumpBack {
						if depth == 0 { break } else { depth -= 1 }
					}
				}
			},
			CatInstruction::JumpBack => if self.tape[self.pointer] != 0 {
				let mut depth = 0usize;
				loop {
					if self.pc == 0 {
						return Err("Couldn't find JumpForward instruction for JumpBack.");
					}
					self.pc -= 1;
					if self.instructions[self.pc] == CatInstruction::JumpBack {
						depth += 1;
					}
					if self.instructions[self.pc] == CatInstruction::JumpForward {
						if depth == 0 { break } else { depth -= 1 }
					}
				}
			},
			// _ => return Err("hhhaha.. what the fuck"),
		}
		if self.debug {
			match self.instructions[self.pc] {
				CatInstruction::MoveRight => println!("moved right"),
				CatInstruction::MoveLeft => println!("moved left"),
				CatInstruction::Increment => println!("incremented"),
				CatInstruction::Decrement => println!("decremented"),
				_ => {},
			}
		}
		self.pc += 1;
		
		Ok(())
	}
	pub fn is_done(&self) -> bool {
		self.pc >= self.instructions.len()
	}
}
