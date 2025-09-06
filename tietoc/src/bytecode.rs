use crate::{opcodes::Opcode, value::Value};

pub struct Bytecode {
    consts: Vec<Value>,
    code:  Vec<Opcode>
}

impl Bytecode {
    fn dump(self, path: String) {
        let mut content: Vec<u8> = Vec::with_capacity(16 + self.code.len() + self.consts.len());
        // Header
        content.extend(b"Tieto");
        content.extend(env!("CARGO_PKG_VERSION").split('.').map(|str| {str.parse::<u8>().unwrap()}));
        content.extend((self.code.len() as u32).to_le_bytes());
        content.extend((self.consts.len() as u32).to_le_bytes());
        // Contents
        content.extend(self.code.iter().map(|&opcode| opcode as u8));
        for value in self.consts {
            content.extend(value.0.to_le_bytes());
        }
        // Write
        std::fs::write(path, content).expect("Could not open file for bytecode dumping.");
    }
}