pub enum Bit {
    On,
    Off,
}

impl ToString for Bit {
    fn to_string(&self) -> String {
        match self {
            Bit::On  => "1".to_string(),
            Bit::Off => "0".to_string(),
        }
    }
}

pub fn u8_to_bit_vec(byte: u8) -> Vec<Bit> {
    let mut res: Vec<Bit> = Vec::new();
    
    for i in 0..8 {
        res.push(match byte & (1 << i) {
            1 => Bit::On,
            0 => Bit::Off,
            n => panic!("Error #1. Value should've been 0 or 1 but was: {:?}", n),
        });
    }

    return res;
}