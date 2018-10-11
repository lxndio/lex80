extern crate clap;

mod bit;
mod memory;
mod registers;
mod instructions;

use clap::{Arg, App};

use memory::*;
use registers::*;
use instructions::*;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");

fn main() {
    // Handle command line arguments
    let args = App::new("Lex80 CPU Simulator")
        .version(VERSION)
        .author(AUTHORS)
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Activates verbose mode"))
        .arg(Arg::with_name("INPUT")
            .help("Sets the .out file to put in memory")
            .required(true)
            .index(1))
        .get_matches();

    let verbose_mode = args.is_present("verbose");
    let input_file = args.value_of("INPUT").unwrap();

    // Generate main memory
    let mut mem: Memory = Memory::new();

    // Generate registers
    let mut reg: Registers = Registers::new();

    // Fill memory with input file at location 0x0000
    mem.load_from_file(input_file.to_string(), 0x0000);

    // Run the CPU
    let mut running: bool = true;

    while running {
        run_instruction(instruction_to_array(&mem, reg.get(1).unwrap() as usize), &mut mem, &mut reg);
    }

    mem.print(0, 10);
}