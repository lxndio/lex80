extern crate clap;

mod assembler;
mod instruction;

use std::io::Result;

use clap::{Arg, App};

use assembler::*;
use instruction::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    // Handle command line arguments
    let args = App::new("Lex80 Assembler")
        .version(VERSION)
        .author(AUTHORS)
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Activates verbose mode"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the .l80 file to assemble")
            .required(true)
            .index(1))
        .arg(Arg::with_name("OUTPUT")
            .help("Sets the .out file containing the assembled opcodes")
            .required(true)
            .index(2))
        .get_matches();

    let verbose_mode = args.is_present("verbose");
    let input_file = args.value_of("INPUT").unwrap();
    let output_file = args.value_of("OUTPUT").unwrap();

    let instructions: Vec<Instruction> = match generate_instructions(verbose_mode) {
        Ok(i)   => i,
        Err(e)  => panic!("Error while generation instructions: {:?}", e),
    };

    for i in &instructions {
        println!("{:?}", i);
    }

    let mut assembler: Assembler = Assembler::new(input_file.to_string(), output_file.to_string(), instructions);

    assembler.assemble(verbose_mode);
}

fn generate_instructions(verbose: bool) -> Result<Vec<Instruction>> {
    let mut res: Vec<Instruction> = Vec::new();

    println!("=> Loading instructions...");

    let opcodes = include_str!("opcodes.l80op");

    for line in opcodes.lines() {
        // Check if line has characters (if not, skip the line)
        if line.len() == 0 {
            continue;
        }

        verbose_println(verbose, format!("Adding opcode: {:?}", line));

        let line_parts: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();

        let mut parameters: Vec<Parameter> = Vec::new();
        for i in 1..3 {
            parameters.push(match line_parts[i] {
                "r" => Parameter::Register,
                "a" => Parameter::Address,
                "v" => Parameter::Value,
                "0" => Parameter::Null,
                _   => panic!("Found wrong parameter type in opcodes.l80op"),
            });
        }

        let mut opcode_parts: Vec<OpcodePart> = Vec::new();
        for i in 3..line_parts.len() {
            verbose_println(verbose, format!("Line part: {:?}", line_parts[i]));
            opcode_parts.push(match line_parts[i] {
                "rr"       => OpcodePart::RegisterDouble,
                "r0"       => OpcodePart::RegisterMSB,
                "0r"       => OpcodePart::RegisterLSB,
                "a" | "aa" => OpcodePart::Address,
                "v" | "vv" => OpcodePart::Value,
                "0" | "00" => OpcodePart::Null,
                cmd        => OpcodePart::Command(match u8::from_str_radix(cmd, 16) {
                    Ok(c)   => c,
                    Err(e)  => panic!("Error while parsing command. {:?}", e),
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

fn verbose_println(verbose: bool, msg: String) {
    if verbose { println!("{}", msg) }
}