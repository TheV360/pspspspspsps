mod lib;
use lib::*;

fn main() {
	let code = std::fs::read_to_string("hello.pspspsps").unwrap();
	let mut machine = CatInterpreter::new(2048, CatInstruction::pspspspsps_to_vec(&code));
	// machine.debug = true;
	let mut instr_left = 0xffffusize;
	while !machine.is_done() && instr_left > 0 {
		machine.step().unwrap();
		instr_left -= 1;
	}
	if instr_left == 0 {
		println!("ran out of instructions.");
	} else {
		println!("program completed.");
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn pspspspsps_to_usize_test() {
		let good_examples = "pspspsps pspsps pspsp psps pspspsps";
		let needed_results = [8, 6, 5, 4, 8];
		
		for (index, substring) in good_examples.split_ascii_whitespace().enumerate() {
			let result = CatInstruction::pspspspsps_to_usize(substring);
			
			assert!(result.is_ok());
			assert_eq!(result.unwrap(), needed_results[index]);
		}
		
		let bad_examples = "pspspspspsspsps pss piss pee poop ppsspspsppp pspspspps fuck i think i screwed it up";
		
		for substring in bad_examples.split_ascii_whitespace() {
			assert!(CatInstruction::pspspspsps_to_usize(substring).is_err());
		}
	}
	
	#[test]
	fn machine_basic_move_program() {
		let mut machine = CatInterpreter::new(4, vec![
			CatInstruction::MoveRight,
			CatInstruction::MoveLeft,
			CatInstruction::MoveRight,
		]);
		
		while !machine.is_done() {
			if let Err(e) = machine.step() {
				panic!("{}", e);
			}
		}
		
		assert_eq!(machine.pointer, 1);
	}
	
	#[test]
	fn machine_basic_add_program() {
		let mut machine = CatInterpreter::new(4, vec![
			CatInstruction::Increment, // [0] = 1
			CatInstruction::MoveRight,
			CatInstruction::Increment,
			CatInstruction::Increment, // [1] = 2
			CatInstruction::MoveLeft,
			CatInstruction::Decrement, // [0] = 0
			CatInstruction::MoveRight,
			CatInstruction::Decrement,
			CatInstruction::Increment, // [1] = 2
		]);
		
		machine.step().unwrap();
		assert_eq!(machine.tape[0], 1);
		machine.step().unwrap();
		
		machine.step().unwrap();
		assert_eq!(machine.tape[1], 1);
		machine.step().unwrap();
		assert_eq!(machine.tape[1], 2);
		machine.step().unwrap();
		
		machine.step().unwrap();
		assert_eq!(machine.tape[0], 0);
		machine.step().unwrap();
		
		machine.step().unwrap();
		assert_eq!(machine.tape[1], 1);
		machine.step().unwrap();
		assert_eq!(machine.tape[1], 2);
		
		assert_eq!(machine.pointer, 1);
	}
	
	#[test]
	fn machine_basic_output() {
		let mut machine = CatInterpreter::new(4, vec![CatInstruction::Increment; 0x42]);
		let last = machine.instructions.len() - 1;
		machine.instructions[last] = CatInstruction::Output;
		
		while !machine.is_done() {
			if let Err(e) = machine.step() {
				panic!("{}", e);
			}
		}
	}
}
