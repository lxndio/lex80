// lex80 CPU Simulator - Instructions Unit
// lxndio, 2018-10-10

use memory::*;
use registers::*;

pub fn run_instruction(instruction: [u8; 4], memory: &mut Memory, registers: &mut Registers, verbose: bool) -> bool {

	if verbose {
		print!("Instruction: ");
		for i in instruction.iter() {
			print!("{:x?}{:x?} ", get_upper_hex(*i), get_lower_hex(*i));
		}
		println!();
	}

	match instruction[0] {
		// 01 .. .. ..
		1 => match get_upper_hex(instruction[1]) {
			// 01 0x aa aa - ld X, A
			0 => {
				let x: u8 = get_lower_hex(instruction[1]);
				let a: usize = ((instruction[2] as usize) << 8) + (instruction[3] as usize);
				let v: u8 = memory.get(a).unwrap();
				registers.set_8bit(x as usize, v);
			},

			_ => match get_lower_hex(instruction[1]) {
				// 01 x0 vv 00 - ld X, V
				0 => {
					let x: u8 = get_upper_hex(instruction[1]);
					let v: u8 = instruction[2];
					registers.set_8bit(x as usize, v);

					return true;
				},

				// 01 xy 00 00 - ld X, Y
				_ => {
					let x: u8 = get_upper_hex(instruction[1]);
					let y: u8 = get_lower_hex(instruction[1]);
					let v: u8 = registers.get_8bit(y as usize).unwrap();
					registers.set_8bit(x as usize, v);

					return true;
				},
			}
		},

		// 02 .. .. ..
		2 => match instruction[1] {
			// 02 00 xy 00 - add X, Y
			0 => {
				let x_v: u8 = registers.get_8bit(get_upper_hex(instruction[2]) as usize).unwrap();
				let y_v: u8 = registers.get_8bit(get_lower_hex(instruction[2]) as usize).unwrap();
				let mut r: u8 = 0;
				
				// Check if overflow would occur
				if (x_v as u16) + (y_v as u16) > 255 {
					// TODO set overflow flag
				} else {
					r = x_v + y_v;
				}

				registers.set_8bit(get_upper_hex(instruction[2]) as usize, r);

				return true;
			},

			_ => panic!("Error 101"),
		},

		// 0A .. .. ..
		10 => match get_upper_hex(instruction[1]) {
			// 0A 0. .. ..
			0 => match get_lower_hex(instruction[1]) {
				// 0A 00 00 00 - halt
				0 => return false,

				// 0A 0x 00 00 - dbg X
				_ => {
					let x: u8 = get_lower_hex(instruction[1]);
					let v: u8 = registers.get_8bit(x as usize).unwrap();
					println!("{:?}", v);

					return true;
				},
			},

			_ => panic!("Error 102"),
		},

		_ => panic!("Error 100"),
	}

	return false;
}

pub fn instruction_to_array(memory: &Memory, address: usize) -> [u8; 4] {
	let mut res: [u8; 4] = [0; 4];

	for i in 0..3 {
		res[i] = memory.get(address + (i*8)).unwrap();
	}

	return res;
}

#[inline]
fn get_lower_hex(value: u8) -> u8 { value & 0b0000_1111 }

#[inline]
fn get_upper_hex(value: u8) -> u8 { value >> 4 }