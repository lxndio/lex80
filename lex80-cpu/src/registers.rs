// lex80 CPU Simulator - Register Unit
// lxndio, 2018-10-10

pub struct Registers {
	contents: [u8; 9],
}

impl Registers {
	pub fn new() -> Registers {
		Registers {
			contents: [0; 9],
		}
	}

	pub fn set(&mut self, register: usize, byte: u8) -> bool {
		if register >= 1 && register <= 9 {
			self.contents[register] = byte;
			return true;
		}

		return false;
	}

	// Increases the register by 1
	pub fn increase(&mut self, register: usize) -> bool {
		if register >= 1 && register <= 9 {
			self.contents[register] += 1;
			return true;
		}

		return false;
	}

	// Decreases the register by 1
	pub fn decrease(&mut self, register: usize) -> bool {
		if register >= 1 && register <= 9 {
			self.contents[register] -= 1;
			return true;
		}

		return false;
	}

	pub fn get(&self, register: usize) -> Option<u8> {
		if register >= 1 && register <= 9 {
			return Some(self.contents[register]);
		}

		return None;
	}
}