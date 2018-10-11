// lex80 CPU Simulator - Instructions Unit
// lxndio, 2018-10-10

use memory::*;
use registers::*;

pub fn run_instruction(instruction: [u8; 8], memory: &mut Memory, registers: &mut Registers) -> bool {
	match instruction[0] {
		// 0. .. .. ..
		0 => match instruction[1] {
			// 01 .. .. ..
			1 => match instruction[2] {
				// 01 0x aa aa - ld X, A
				0 => {
					let a: usize = (instruction[4] as usize * 0x1000)
								 + (instruction[5] as usize * 0x100)
								 + (instruction[6] as usize * 0x10)
								 + (instruction[7] as usize);
					let v: u8 = memory.get(a).unwrap();
					registers.set(instruction[3] as usize, v);
				},

				_ => {
					match instruction[3] {
						// 01 x0 0v 00 - ld X, V
						0 => {
							let v: u8 = instruction[5];
							registers.set(instruction[2] as usize, v);

							return true;
						},

						// 01 xy 00 00 - ld X, Y
						_ => {
							let y: u8 = registers.get(instruction[3] as usize).unwrap();
							registers.set(instruction[2] as usize, y);

							return true;
						},
					}
				},
			},

			// 02 .. .. ..
			2 => match instruction[2] {
				// 02 0. .. ..
				0 => match instruction[3] {
					// 02 00 xy 00 - add X, Y
					0 => {
						let x: u8 = registers.get(instruction[4] as usize).unwrap();
						let y: u8 = registers.get(instruction[5] as usize).unwrap();
						registers.set(instruction[4] as usize, x+y);

						if x+y > 15 {
							// Todo set C flag
							unimplemented!();
						}
					},

					_ => panic!("Error 103"),
				},

				_ => panic!("Error 102"),
			},

			_ => panic!("Error 101"),
		},
		_ => panic!("Error 100"),
	}

	return false;
}

pub fn instruction_to_array(memory: &Memory, address: usize) -> [u8; 8] {
	let mut res: [u8; 8] = [0; 8];

	for i in 0..7 {
		res[i] = memory.get(address + (i*8)).unwrap();
	}

	return res;
}