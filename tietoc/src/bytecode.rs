use crate::{opcodes::Opcode, value::Value};

#[derive(Debug, Clone)]
pub struct Bytecode {
    consts: Vec<Value>,
    code:  Vec<Opcode>
}

impl Bytecode {
    pub fn new() -> Self {
        Self {
            consts: Vec::new(),
            code: Vec::new()
        }
    }
    pub fn dump(&self, path: String) {
        let mut content: Vec<u8> = Vec::with_capacity(16 + self.code.len() + self.consts.len());
        // header
        content.extend(b"Tieto");
        content.extend(env!("CARGO_PKG_VERSION").split('.').map(|str| {str.parse::<u8>().unwrap()}));
        content.extend((self.code.len() as u32).to_le_bytes());
        content.extend((self.consts.len() as u32).to_le_bytes());
        // contents
        content.extend(self.code.iter().map(|&opcode| opcode as u8));
        for value in &self.consts {
            content.extend(value.to_le_bytes());
        }
        // write
        std::fs::write(path, content).expect("Could not open file for bytecode dumping.");
    }
}