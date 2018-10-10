mod memory;
mod bit;

use memory::*;

fn main() {
    let mut mem: Memory = Memory::new();

    mem.set(0, 0x01);
    mem.set(8, 0x02);
    mem.set(16, 0x03);
    mem.set(24, 0x04);
    mem.set(32, 0x05);
    mem.set(40, 0x06);
    mem.set(48, 0x07);
    mem.set(56, 0x08);
    mem.set(64, 0x09);
    mem.set(72, 0x0A);

    mem.print(0, 10);
}