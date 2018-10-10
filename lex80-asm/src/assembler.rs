// lex80 Assembler
// lxndio, 2018-07-16

use std::fs::File;
use std::io::{BufRead, BufReader, Write, Result, Error, ErrorKind};
use std::collections::HashMap;

use instruction::*;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub struct Assembler {
    src_file: String,
    output_file: String,
    program: Vec<u8>,
    instructions: Vec<Instruction>,
    labels: HashMap<String, u16>,
    assembled: bool,
}

impl Assembler {
    pub fn new(src_file: String, output_file: String, instructions: Vec<Instruction>) -> Assembler {
        Assembler {
            src_file: src_file,
            output_file: output_file,
            program: Vec::new(),
            instructions: instructions,
            labels: HashMap::new(),
            assembled: false,
        }
    }

    pub fn assemble(&mut self, verbose: bool) -> Result<()> {
        // Define register list
        let register_list: Vec<String> = vec_of_strings!("pc", "sp", "a", "b", "c", "d", "e", "f", "g");

        // Load file
        println!("=> Loading input file...");
        let file = File::open(&self.src_file)?;

        // Clear assembled program vector and set assembled to false
        self.program = Vec::new();
        self.assembled = false;

        let mut current_address: u16 = 0;

        // Read file and assemble the code
        println!("=> Assembling code...");
        for line in BufReader::new(file).lines() {
            let line_str = match line {
                Ok(l)   => l,
                Err(e)  => panic!("Error while parsing line. {:?}", e),
            };

            // Check if line has characters (if not, skip the line)
            if line_str.len() == 0 {
                continue;
            }

            // Check if line is a comment
            if line_str.starts_with('#') {
                continue;
            }

            let line_parts: Vec<&str> = line_str.split_whitespace().collect::<Vec<&str>>();
            let mut i: usize = 0;

            // Check if line is labeled
            // Note: .unwrap() should be save here without error handling,
            //       because line_str's length is greater than 0 (as checked before)
            if line_parts[i].chars().last().unwrap() == ':' {
                let mut cmd: String = line_parts[i].to_string();
                cmd.truncate(line_parts[i].len() - 1);

                self.labels.insert(cmd, current_address);

                i += 1;
            }

            verbose_println(verbose, format!("Assembling command: {:?}", line_parts[i].to_string()));

            // Check if instruction exists
            if !Assembler::instruction_exists(&self, line_parts[i].to_string()) {
                // TODO fix error handling
                println!("Instruction does not exist: {:?}", line_parts[i].to_string());
                return Err(Error::new(ErrorKind::Other, format!("Instruction does not exist: {:?}", line_parts[i].to_string())));
            }

            // Get instruction information from database
            let instruction: &Instruction = match Assembler::get_instruction(&self.instructions, line_parts[i].to_string()) {
                Some(i) => i,
                None    => panic!("hi 1"),
            };

            // Generate instruction opcode and push it to program
            let mut instr_params: Vec<u8> = Vec::new();

            for j in i+1..line_parts.len() {
                // Store line part seperately for now
                let mut line_part: String = line_parts[j].to_string();

                // Remove comma at at the end of the part (if there is one)
                if line_part.chars().last().unwrap() == ',' { // TODO make safe instead of unwrap, possibly already safe in this case?
                    line_part = line_part.replace(",", "");
                }

                // Check if part is a register, a value or an address
                if register_list.contains(&line_part.to_string()) {
                    // TODO find better solution
                    let arr_num: u8 = match Assembler::get_pos_in_arr(&register_list, &line_part.to_string()) {
                        Some(p) => (p as u8) + 1, // TODO is casting safe in this case?
                        None    => panic!("hi ... couldn't possibly fail here, whatever..."),
                    };

                    verbose_println(verbose, format!("Register: {:?}, number: {:b}", line_parts[j], arr_num));

                    instr_params.push(arr_num);

                    continue;
                } else if line_part.starts_with('$') {
                    line_part = line_part.replace("$", "");
                    line_part.truncate(3);

                    let line_part_u8: u8 = match line_part.parse::<u8>() {
                        Ok(i)  => i,
                        Err(e) => panic!("hi ... number was greater than 255 (8 bit) {:?}", e),
                    };

                    verbose_println(verbose, format!("Line part: {:?}", line_part_u8));

                    instr_params.push(0b1010);

                    continue;
                } else if line_part.starts_with('#') {
                    // Comment mode
                    break;
                } else {
                    // TODO
                    instr_params.push(0b1111);

                    continue;
                }
            }

            let instr_opcode: Vec<u8> = instruction.generate_opcode(instr_params);

            for byte in instr_opcode {
                self.program.push(byte);
            }

            // Set current address one instruction ahead
            current_address += 32;
        }

        // Save program to tmp_file
        let mut file_out = File::create(&self.output_file)?;

        for byte in &self.program {
            file_out.write(&[*byte])?;
        }

        // Set assembled to true and return
        self.assembled = true;

        return Ok(());
    }

    fn instruction_exists(&self, name: String) -> bool {
        for instruction in &self.instructions {
            if instruction.name().to_string() == name {
                return true;
            }
        }

        return false;
    }

    fn get_instruction(instructions: &Vec<Instruction>, name: String) -> Option<&Instruction> {
        for instruction in instructions {
            if instruction.name().to_string() == name {
                return Some(instruction);
            }
        }

        return None;
    }

    fn get_pos_in_arr(arr: &[String], elem: &String) -> Option<usize> {
        let mut i: usize = 0;

        for a in arr {
            if a == elem { return Some(i); }
            i += 1;
        }

        return None;
    }
}

fn verbose_println(verbose: bool, msg: String) {
    if verbose { println!("{}", msg) }
}