pub struct Instruction {
    value: u16,
}

impl Instruction {
    pub fn new(content: u16) -> Self {
        Self { value: content }
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn opcode(&self) -> u8 {
        ((self.value >> 12) & 0xF) as u8
    }

    pub fn x(&self) -> u8 {
        ((self.value >> 8) & 0xF) as u8
    }

    pub fn y(&self) -> u8 {
        ((self.value >> 4) & 0xF) as u8
    }

    pub fn nnn(&self) -> u16 {
        self.value & 0xFFF
    }

    pub fn nn(&self) -> u8 {
        (self.value & 0xFF) as u8
    }

    pub fn n(&self) -> u8 {
        (self.value & 0xF) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_value() {
        let instr = Instruction::new(0x1234);
        let value = instr.value();
        assert_eq!(value, 0x1234);
    }

    #[test]
    fn gets_x() {
        let instr = Instruction::new(0x1234);
        let x = instr.x();
        assert_eq!(x, 0x2);
    }

    #[test]
    fn gets_y() {
        let instr = Instruction::new(0x1234);
        let y = instr.y();
        assert_eq!(y, 0x3);
    }

    #[test]
    fn gets_nnn() {
        let instr = Instruction::new(0x1234);
        let nnn = instr.nnn();
        assert_eq!(nnn, 0x234);
    }

    #[test]
    fn gets_nn() {
        let instr = Instruction::new(0x1234);
        let nn = instr.nn();
        assert_eq!(nn, 0x34);
    }

    #[test]
    fn gets_n() {
        let instr = Instruction::new(0x1234);
        let n = instr.n();
        assert_eq!(n, 0x4);
    }
}
