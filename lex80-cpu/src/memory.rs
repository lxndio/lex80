// lex80 CPU Simulator - Memory Unit
// lxndio, 2018-07-17

#![allow(dead_code)]

use std::fs::File;
use std::io::{Read, Result};

// TODO make memory variable in size

pub struct Memory {
    size: usize,
    bytes: [u8; 65536],
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            size: 65536,
            bytes: [0; 65536],
        }
    }

    pub fn set(&mut self, address: usize, byte: u8) -> bool {
        // Currently it is only possible to access whole blocks
        // TODO change that

        if address % 8 == 0 && address < self.size - 8 {
            self.bytes[address/8] = byte;
            return true;
        }

        return false;
    }

    pub fn get(&self, address: usize) -> Option<u8> {
        // see note at set function, TODO

        if address % 8 == 0 && address < self.size - 8 {
            return Some(self.bytes[address/8]);
        }

        return None;
    }

    pub fn load_from_file(&mut self, input_file: String, starting_addr: usize) -> Result<()> {
        let file = File::open(input_file)?;

        let mut current_addr: usize = starting_addr;

        for byte in file.bytes() {
            &self.set(current_addr, byte.unwrap());

            current_addr += 8;
        }

        return Ok(());
    }

    pub fn print(&self, starting_addr: usize, blocks: usize) -> bool {
        // TODO make safe so that it can't get out of bounds

        if starting_addr % 8 != 0 {
            return false;
        }

        let mut i: usize = starting_addr/8;

        while i < (starting_addr/8) + blocks {
            println!(
                "{:X}:\t{:X}\t{:X}\t{:X}\t{:X}",
                i*8, self.bytes[i], self.bytes[i+1], self.bytes[i+2], self.bytes[i+3]
            );

            i += 4;
        }

        return true;
    }
}