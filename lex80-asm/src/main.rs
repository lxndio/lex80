mod assembler;
mod instruction;

use std::fs::File;
use std::io::{BufRead, BufReader, Result};

use assembler::*;
use instruction::*;

fn main() {
	let instructions: Vec<Instruction> = match generate_instructions() {
		Ok(i)	=> i,
		Err(e)	=> panic!("Error while generation instructions: {:?}", e),
	};

	let mut assembler: Assembler = Assembler::new("C:\\Users\\korni\\Dateiablage\\_TEMP\\l80tests\\test.l80".to_string(), instructions);

	assembler.assemble();
}

fn generate_instructions() -> Result<Vec<Instruction>> {
	let mut res: Vec<Instruction> = Vec::new();

	let file = File::open("C:\\Users\\korni\\Dateiablage\\_TEMP\\l80tests\\opcodes.db".to_string())?;

	for line in BufReader::new(file).lines() {
		let line_str = match line {
			Ok(l)	=> l,
			Err(e)	=> panic!("Error while parsing line. {:?}", e),
		};

		// Check if line has characters (if not, skip the line)
		if line_str.len() == 0 {
			continue;
		}

		println!("Adding opcode: {:?}", line_str);

		let line_parts: Vec<&str> = line_str.split_whitespace().collect::<Vec<&str>>();

		let mut parameters: Vec<Parameter> = Vec::new();
		for i in 1..3 {
			parameters.push(match line_parts[i] {
				"r" => Parameter::Register,
				"a" => Parameter::Address,
				"v" => Parameter::Value,
				"0" => Parameter::Null,
				_   => panic!("Found wrong parameter type in opcodes.db"),
			});
		}

		let mut opcode_parts: Vec<OpcodePart> = Vec::new();
		for i in 3..line_parts.len() {
			println!("Line part: {:?}", line_parts[i]);
			opcode_parts.push(match line_parts[i] {
				"rr"       => OpcodePart::RegisterDouble,
				"r0"       => OpcodePart::RegisterMSB,
				"0r"       => OpcodePart::RegisterLSB,
				"a" | "aa" => OpcodePart::Address,
				"v" | "vv" => OpcodePart::Value,
				"0" | "00" => OpcodePart::Null,
				cmd        => OpcodePart::Command(match u8::from_str_radix(cmd, 16) {
					Ok(c)	=> c,
					Err(e)	=> panic!("Error while parsing command. {:?}", e),
				}),
			});
		}

		res.push(Instruction::new(
			line_parts[0].to_string(),
			parameters,
			opcode_parts
		));
	}
	
	return Ok(res);
}