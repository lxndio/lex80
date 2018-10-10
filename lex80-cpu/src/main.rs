mod memory;
mod bit;

use memory::*;

fn main() {
    let mut mem: Memory = Memory::new();

    mem.load_from_file("test.out".to_string(), 0);

    mem.print(0, 10);
}

