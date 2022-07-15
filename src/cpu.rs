use rand::prelude::*;

use crate::{cpu::instruction::Instruction, gpu::Gpu};

mod instruction;

const MEMORY_SIZE: usize = 4096;
const REGISTER_COUNT: usize = 16;
const RESERVED_START: usize = 0x200;
const RESERVED_END: usize = 352;
const PROGRAM_START: usize = 0x200;
const STEP_SIZE: usize = 2;
const STACK_SIZE: usize = 16;

const FONT_START_ADDR: usize = 0x050;
const FONT_SPRITE_SIZE: usize = 5;
const FONT_SPRITE_COUNT: usize = 16;
const FONT_BLOCK_SIZE: usize = FONT_SPRITE_SIZE * FONT_SPRITE_COUNT;
const FONT: [u8; FONT_BLOCK_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Cpu<'a> {
    gpu: &'a mut Gpu,
    pub memory: [u8; MEMORY_SIZE],
    registers: [u8; REGISTER_COUNT],
    pub address_register: usize,
    pub pc: usize,
    pub stack: [usize; STACK_SIZE],
    pub sp: usize,
    pub delay_timer: u8,
    pub sound_timer: u8,
    keys: u16,
    running: bool,
    pub redraw: bool,
}

impl<'a> Cpu<'a> {
    pub fn new(gpu: &'a mut Gpu) -> Self {
        let mut cpu = Self {
            gpu,
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTER_COUNT],
            address_register: 0,
            pc: PROGRAM_START,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: 0,
            running: true,
            redraw: false,
        };
        cpu.reset();
        cpu
    }

    pub fn reset(&mut self) {
        self.gpu.clear();
        self.registers.fill(0);
        self.address_register = 0;
        self.pc = PROGRAM_START;
        self.stack.fill(0);
        self.sp = 0;
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.keys = 0;
        self.running = true;
        self.redraw = false;
        self.memory[FONT_START_ADDR..(FONT_START_ADDR + FONT.len())].copy_from_slice(&FONT);
    }

    pub fn register(&self, idx: usize) -> u8 {
        self.registers[idx]
    }

    pub fn load(&mut self, program_bytes: &[u8]) {
        let available = MEMORY_SIZE - RESERVED_START - RESERVED_END;
        if program_bytes.len() > available {
            panic!(
                "Program too large! ({} bytes, available: {} bytes)",
                program_bytes.len(),
                available
            );
        }
        let start_addr = RESERVED_START;
        let end_addr = MEMORY_SIZE - RESERVED_END - 1;
        println!("Loading program ({} bytes)", program_bytes.len());
        println!(
            "Start addr: {:#04X}, end addr: {:#04X}",
            start_addr, end_addr
        );
        self.memory[start_addr..(program_bytes.len() + start_addr)].copy_from_slice(program_bytes);
    }

    fn read16(&self, addr: usize) -> u16 {
        let hi = self.memory[addr];
        let lo = self.memory[addr + 1];
        ((hi as u16) << 8) | (lo as u16)
    }

    pub fn screen(&self) -> &[bool] {
        self.gpu.screen()
    }

    fn stack_push(&mut self, value: usize) {
        self.stack[self.sp] = value;
        self.sp += 1;
    }

    fn stack_pop(&mut self) -> usize {
        self.sp -= 1;
        self.stack[self.sp]
    }

    pub fn step(&mut self) {
        if !self.running {
            return;
        }

        let instr = Instruction::new(self.read16(self.pc));
        self.pc += STEP_SIZE;
        self.decode(&instr);

        if self.pc > MEMORY_SIZE {
            panic!("PC outside of memory");
        }
    }

    pub fn tick_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    fn decode(&mut self, instruction: &Instruction) {
        // println!("Decoding instruction: {:04X}", instruction.value());
        let opcode = instruction.opcode();
        let variant_1 = instruction.n();
        let variant_2 = instruction.nn();
        let variant_3 = instruction.nnn();
        match (opcode, variant_1, variant_2, variant_3) {
            (0, _, _, 0x0E0) => {
                // CLR
                self.gpu.clear();
                self.redraw = true;
            }

            (0, _, _, 0x0EE) => {
                // RET
                let return_addr = self.stack_pop();
                self.pc = return_addr;
            }

            (0, _, _, _) => {
                // SYS
                let x = instruction.x();
                let nn = instruction.nn();
                if x == 0xF {
                    println!("DBG:EXIT({})", nn);
                    // exit(nn as i32);
                    self.running = false;
                } else {
                    panic!("Machine code subroutines are not supported")
                }
            }

            (1, _, _, _) => {
                // JMP
                let addr = instruction.nnn() as usize;
                self.pc = addr;
            }

            (2, _, _, _) => {
                // CALL
                let return_addr = self.pc;
                self.stack_push(return_addr);
                let sub_addr = instruction.nnn() as usize;
                self.pc = sub_addr;
            }

            (3, _, _, _) => {
                // SEQ
                let x_val = self.registers[instruction.x() as usize];
                let nn = instruction.nn();
                if x_val == nn {
                    self.pc += STEP_SIZE;
                }
            }

            (4, _, _, _) => {
                // SNE
                let x_val = self.registers[instruction.x() as usize];
                let nn = instruction.nn();
                if x_val != nn {
                    self.pc += STEP_SIZE;
                }
            }

            (5, 0, _, _) => {
                // SEQ
                let x_val = self.registers[instruction.x() as usize];
                let y_val = self.registers[instruction.y() as usize];
                if x_val == y_val {
                    self.pc += STEP_SIZE;
                }
            }

            (6, _, _, _) => {
                // SET
                self.registers[instruction.x() as usize] = instruction.nn();
            }

            (7, _, _, _) => {
                // ADD
                let x = instruction.x() as usize;
                self.registers[x] = self.registers[x].wrapping_add(instruction.nn());
            }

            (8, 0, _, _) => {
                // SETR
                let y_val = self.registers[instruction.y() as usize];
                self.registers[instruction.x() as usize] = y_val;
            }

            (8, 1, _, _) => {
                // OR
                let x_val = self.registers[instruction.x() as usize];
                let y_val = self.registers[instruction.y() as usize];
                self.registers[instruction.x() as usize] = x_val | y_val;
            }

            (8, 2, _, _) => {
                // AND
                let x_val = self.registers[instruction.x() as usize];
                let y_val = self.registers[instruction.y() as usize];
                self.registers[instruction.x() as usize] = x_val & y_val;
            }

            (8, 3, _, _) => {
                // XOR
                let x_val = self.registers[instruction.x() as usize];
                let y_val = self.registers[instruction.y() as usize];
                self.registers[instruction.x() as usize] = x_val ^ y_val;
            }

            (8, 4, _, _) => {
                // ADC
                let x = instruction.x() as usize;
                let x_val = self.registers[x];
                let y_val = self.registers[instruction.y() as usize];
                let new = x_val.wrapping_add(y_val);
                self.registers[x] = new;
                self.registers[0xF] = if new < x_val { 1 } else { 0 };
            }

            (8, 5, _, _) => {
                // SUB
                let x = instruction.x() as usize;
                let x_val = self.registers[x];
                let y_val = self.registers[instruction.y() as usize];
                let new = x_val.wrapping_sub(y_val);
                self.registers[x] = new;
                self.registers[0xF] = if new > x_val { 1 } else { 0 };
            }

            (8, 6, _, _) => {
                // SHR
                let y_val = self.registers[instruction.y() as usize];
                self.registers[0xF] = y_val & 0x1;
                let shifted = y_val >> 1;
                self.registers[instruction.x() as usize] = shifted;
            }

            (8, 7, _, _) => {
                // SUBN
                let x = instruction.x() as usize;
                let x_val = self.registers[x];
                let y_val = self.registers[instruction.y() as usize];
                let new = y_val.wrapping_sub(x_val);
                self.registers[x] = new;
                self.registers[0xF] = if new > y_val { 1 } else { 0 };
            }

            (8, 0xE, _, _) => {
                // SHL
                let y_val = self.registers[instruction.y() as usize];
                self.registers[0xF] = (y_val >> 7) & 0x1;
                let shifted = y_val << 1;
                self.registers[instruction.x() as usize] = shifted;
            }

            (9, 0, _, _) => {
                // SNE
                let x_val = self.registers[instruction.x() as usize];
                let y_val = self.registers[instruction.y() as usize];
                if x_val != y_val {
                    self.pc += STEP_SIZE;
                }
            }

            (0xA, _, _, _) => {
                // STO
                self.address_register = instruction.nnn() as usize;
            }

            (0xB, _, _, _) => {
                // JMPR
                self.pc = instruction.nnn() as usize + self.registers[0] as usize;
            }

            (0xC, _, _, _) => {
                // RND
                let num: u8 = rand::thread_rng().gen();
                let mask = instruction.nn();
                let masked = num & mask;
                self.registers[instruction.x() as usize] = masked;
            }

            (0xD, _, _, _) => {
                // DRW
                let x = self.registers[instruction.x() as usize];
                let y = self.registers[instruction.y() as usize];
                let size = instruction.n() as usize;
                let sprite = &self.memory[self.address_register..(self.address_register + size)];
                let hit = self.gpu.draw_sprite(x as usize, y as usize, sprite);
                self.registers[0xF] = if hit { 1 } else { 0 };
                self.redraw = true;
            }

            (0xE, _, 0x9E, _) => {
                // SKP
                let key = self.registers[instruction.x() as usize];
                let pressed = self.is_key_pressed(key);
                if pressed {
                    self.pc += STEP_SIZE;
                }
            }

            (0xE, _, 0xA1, _) => {
                // SKN
                let key = self.registers[instruction.x() as usize];
                let pressed = self.is_key_pressed(key);
                if !pressed {
                    self.pc += STEP_SIZE;
                }
            }

            (0xF, _, 0x07, _) => {
                // LDT
                self.registers[instruction.x() as usize] = self.delay_timer;
            }

            (0xF, _, 0x0A, _) => {
                // WFK
                if let Some(key) = self.get_active_key() {
                    self.registers[instruction.x() as usize] = key;
                } else {
                    self.pc -= STEP_SIZE;
                }
            }

            (0xF, _, 0x15, _) => {
                // SDT
                self.delay_timer = self.registers[instruction.x() as usize];
            }

            (0xF, _, 0x18, _) => {
                // SST
                self.sound_timer = self.registers[instruction.x() as usize];
            }

            (0xF, _, 0x1E, _) => {
                self.address_register += self.registers[instruction.x() as usize] as usize;
            }

            (0xF, _, 0x29, _) => {
                let digit = self.registers[instruction.x() as usize];
                let offset = digit as usize * FONT_SPRITE_SIZE;
                let addr = FONT_START_ADDR + offset;
                self.address_register = addr;
            }

            (0xF, _, 0x33, _) => {
                // BCD
                let x_val = self.registers[instruction.x() as usize];
                let store_idx = self.address_register;
                let hundreds = x_val / 100;
                let tens = (x_val / 10) % 10;
                let ones = x_val % 10;
                self.memory[store_idx] = hundreds;
                self.memory[store_idx + 1] = tens;
                self.memory[store_idx + 2] = ones;
            }

            (0xF, _, 0x55, _) => {
                let x_size = instruction.x() as usize;
                for reg_idx in 0..=x_size {
                    self.memory[self.address_register as usize + reg_idx] = self.registers[reg_idx];
                }
                self.address_register += x_size + 1;
            }

            (0xF, _, 0x65, _) => {
                let x_size = instruction.x() as usize;
                for reg_idx in 0..=x_size {
                    self.registers[reg_idx] = self.memory[self.address_register as usize + reg_idx];
                }
                self.address_register += x_size + 1;
            }

            _ => {
                panic!("Unknown instruction {:04X}", instruction.value());
            }
        }
    }

    fn is_key_pressed(&self, key: u8) -> bool {
        let mask = 1 << key;
        let masked = self.keys & mask;
        masked == mask
    }

    fn get_active_key(&self) -> Option<u8> {
        for i in 0..0xF {
            let mask = 1 << i;
            if self.keys & mask == mask {
                return Some(i);
            }
        }

        None
    }

    pub fn set_key(&mut self, key: u8, pressed: bool) {
        let mask = 1 << key;
        if pressed {
            self.keys |= mask;
        } else {
            self.keys &= !mask;
        }
    }

    pub fn dump(&self) {
        self.dump_memory();
        self.dump_registers();
        println!("Addr reg (h)  {:04X}", self.address_register);
    }

    pub fn dump_memory(&self) {
        const WIDTH: usize = 16;
        print!("Offset (h) ");
        for x in 0..WIDTH {
            print!(" {:02X}", x);
        }
        println!();
        let lines = MEMORY_SIZE / WIDTH;
        let mut last_zero = false;
        for l in 0..lines {
            let offset = 0x10 * l;
            let next_row = &self.memory[offset..(offset + WIDTH)];
            let all_zero = next_row.iter().all(|b| b == &0);
            if all_zero {
                if !last_zero {
                    println!("  *  ");
                }
                last_zero = true;
                continue;
            }
            last_zero = false;
            print!("{:08X}   ", offset);
            for i in 0..WIDTH {
                let idx = offset + i;
                if idx >= MEMORY_SIZE {
                    print!("   ");
                } else {
                    print!(" {:02X}", self.memory[offset + i])
                }
            }
            println!();
        }
    }

    pub fn dump_registers(&self) {
        print!("Register (h) ");
        for x in 0..REGISTER_COUNT {
            print!(" {:02X}", x);
        }
        println!();
        print!("Value    (h) ");
        for i in 0..REGISTER_COUNT {
            print!(" {:02X}", self.registers[i]);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_16_correctly() {
        let mut gpu = Gpu::new();
        let mut cpu = Cpu::new(&mut gpu);
        let program = [0x01_u8, 0x02, 0x03, 0x04];
        cpu.load(&program);
        let val16 = cpu.read16(0x200);
        assert_eq!(val16, 0x0102)
    }
}
