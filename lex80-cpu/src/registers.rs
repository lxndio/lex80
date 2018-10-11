// lex80 CPU Simulator - Register Unit
// lxndio, 2018-10-10

pub struct Registers {
	contents: [u16; 12],
}

impl Registers {
	pub fn new() -> Registers {
		Registers {
			contents: [0; 12],
		}
	}

	pub fn set_8bit(&mut self, register: usize, byte: u8) -> bool {
		if register >= 1 && register <= 12 {
			if register % 2 != 0 {
				self.contents[register] = (self.contents[register] & 0x00FF) + ((byte as u16) << 8);
			} else {
				self.contents[register] = (self.contents[register] & 0xFF00) + (byte as u16);
			}
			return true;
		}

		return false;
	}

	pub fn set_16bit(&mut self, register: usize, bytes: u16) -> bool {
		if register >= 1 && register <= 12 {
			self.contents[register] = bytes;
			return true;
		}

		return false;
	}

	// Increases the register by 1
	pub fn increase(&mut self, register: usize) -> bool {
		if register >= 1 && register <= 12 {
			self.contents[register] += 1;
			return true;
		}

		return false;
	}

	// Decreases the register by 1
	pub fn decrease(&mut self, register: usize) -> bool {
		if register >= 1 && register <= 12 {
			self.contents[register] -= 1;
			return true;
		}

		return false;
	}

	pub fn get_8bit(&self, register: usize) -> Option<u8> {
		if register >= 1 && register <= 12 {
			if register % 2 != 0 {
				return Some((self.contents[register] >> 8) as u8);
			} else {
				return Some((self.contents[register] & 0x00FF) as u8);
			}
		}

		return None;
	}

	pub fn get_16bit(&self, register: usize) -> Option<u16> {
		if register >= 1 && register <= 12 {
			return Some(self.contents[register]);
		}

		return None;
	}
}