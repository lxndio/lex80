// lex80 Assembler
// lxndio, 2018-07-16

use std::fs::File;
use std::io::{BufRead, BufReader, Write, Result, Error, ErrorKind};
use std::collections::HashMap;

use instruction::*;

//macro_rules! vec_of_strings {
//    ($($x:expr),*) => (vec![$($x.to_string()),*]);
//}

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
        let mut register_list: HashMap<String, u8> = HashMap::new();
        register_list.insert("pc".to_string(), 1);
        register_list.insert("sp".to_string(), 2);
        register_list.insert("a".to_string(), 3);
        register_list.insert("b".to_string(), 4);
        register_list.insert("c".to_string(), 5);
        register_list.insert("d".to_string(), 6);
        register_list.insert("e".to_string(), 7);
        register_list.insert("f".to_string(), 8);
        register_list.insert("g".to_string(), 9);

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

            // Split line into parts
            let line_parts: Vec<&str> = line_str.split_whitespace().collect::<Vec<&str>>();
            let mut current_part: usize = 0;

            // Check if line is labeled
            // Note: .unwrap() should be save here without error handling,
            //       because line_str's length is greater than 0 (as checked before)
            if line_parts[current_part].chars().last().unwrap() == ':' {
                let mut cmd: String = line_parts[current_part].to_string();
                cmd.truncate(line_parts[current_part].len() - 1);

                self.labels.insert(cmd, current_address);

                current_part += 1;
            }

            verbose_println(verbose, format!("Assembling command: {:?}", line_parts[current_part].to_string()));

            // Check if instruction exists
            if !Assembler::instruction_exists(&self, line_parts[current_part].to_string()) {
                // TODO fix error handling
                println!("Instruction does not exist: {:?}", line_parts[current_part].to_string());
                return Err(Error::new(ErrorKind::Other, format!("Instruction does not exist: {:?}", line_parts[current_part].to_string())));
            }

            // Get instruction information from database
            let instruction: &Instruction = match Assembler::get_instruction(&self.instructions, line_parts[current_part].to_string()) {
                Some(i) => i,
                None    => panic!("hi 1"),
            };

            // Generate instruction opcode and push it to program
            let mut instr_params: Vec<u8> = Vec::new();

            for i in current_part+1..line_parts.len() {
                // Store line part seperately for now
                let mut line_part: String = line_parts[i].to_string();

                // Remove comma at at the end of the part (if there is one)
                if line_part.chars().last().unwrap() == ',' { // TODO make safe instead of unwrap, possibly already safe in this case?
                    line_part = line_part.replace(",", "");
                }

                // Check if part is a register, a value or an address
                if register_list.contains_key(&line_part.to_string()) {
                    // Part is a register, now get the corresponding numerical representation
                    let register_num: u8 = *register_list.get(&line_part.to_string()).unwrap();

                    verbose_println(verbose, format!("Register: {:?}, number: {:x}", line_parts[i], register_num));

                    instr_params.push(register_num);

                    continue;
                } else if line_part.starts_with('$') {
                    // Part is an address
                } else if line_part.starts_with(':') {
                    // Part is a label
                    // TODO implement
                } else if line_part.starts_with('#') {
                    // Part is a comment
                    break;
                } else {
                    // Part is a value
                    let line_part_u8: u8 = match line_part.parse::<u8>() {
                        Ok(i)  => i,
                        Err(e) => panic!("No values greater than 255 (8 bit) are allowed but given: {:?}", e),
                    };

                    verbose_println(verbose, format!("Line part: {:?}", line_part_u8));

                    instr_params.push(line_part_u8);

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