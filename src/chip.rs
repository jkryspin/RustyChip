use std::fs;

mod cpu;

#[derive(Clone)]
pub struct Chip {
    pub cpu: cpu::Cpu,
}

impl Chip {
    pub fn new(contents: Vec<u8>) -> Self {
        Chip {
            cpu: cpu::Cpu::new(contents),
        }
    }

    pub fn update(&mut self, pressed_keys: [u8; 16]) {
        let byte1 = self.cpu.ram[self.cpu.pc as usize];
        let byte2 = self.cpu.ram[(self.cpu.pc + 1) as usize];
        let val = (byte1 as u16 & 0xF);
        let op_data = Op {
            op: (byte1 >> 4) & 0xF,
            x: (byte1 & 0xF),
            y: ((byte2 >> 4) & 0xF),
            n: (byte2 & 0xF),
            nn: byte2,
            nnn: ((val << 8) | byte2 as u16),
        };
        self.cpu.execute_op(op_data, pressed_keys);
    }
}

pub struct Op {
    op: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
}
