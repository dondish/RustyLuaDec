// Instruction encoding
pub enum Instruction {
    IAbc { b: u16, c: u16, a: u8, opcode: u8 },
    IABx { bx: u32, a: u8, opcode: u8 },
    IAsBx { sbx: u32, a: u8, opcode: u8 },
}

impl Instruction {
}
