pub enum Parameter {
    Register,
    Address,
    Value,
    Null,
}

pub enum OpcodePart {
    Command(u8),
    RegisterDouble,
    RegisterLSB,
    RegisterMSB,
    Address,
    Value,
    Null,
}

pub struct Instruction {
    name: String,
    parameters: Vec<Parameter>,
    opcode_parts: Vec<OpcodePart>,
}

impl Instruction {
    pub fn new(name: String, parameters: Vec<Parameter>, opcode_parts: Vec<OpcodePart>) -> Instruction {
        Instruction {
            name: name,
            parameters: parameters,
            opcode_parts: opcode_parts,
        }
    }

    pub fn generate_opcode(&self, parameters: Vec<u8>) -> Vec<u8> {
        let mut res: Vec<u8> = Vec::new();
        let mut param_counter: usize = 0;

        for part in &self.opcode_parts {
            match part {
                &OpcodePart::Command(c) => res.push(c),
                &OpcodePart::RegisterDouble => {
                    res.push(parameters[param_counter]);
                    res.push(parameters[param_counter+1]);
                    param_counter += 2;
                },
                &OpcodePart::RegisterMSB    => {
                    res.push(parameters[param_counter]);
                    param_counter += 1;
                },
                &OpcodePart::RegisterLSB    => {
                    res.push(parameters[param_counter]);
                    param_counter += 1;
                },
                &OpcodePart::Address        => {
                    res.push(parameters[param_counter]);
                    res.push(parameters[param_counter+1]);
                    param_counter += 2;
                },
                &OpcodePart::Value          => {
                    res.push(parameters[param_counter]);
                    param_counter += 1;
                },
                &OpcodePart::Null           => {
                    res.push(0);
                }
            }
        }

        return res;
    }

    pub fn name(&self) -> &String { &self.name }

    pub fn opcode_part(&self, i: usize) -> Option<&OpcodePart> {
        if i < *&self.opcode_parts.len() {
            Some(&self.opcode_parts[i])
        } else {
            None
        }
    }
}