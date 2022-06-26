use std::fs;

mod cpu;

#[derive(Clone)]
pub struct Chip {
    cpu: cpu::Cpu,
}

impl Chip {
    pub fn new() -> Self {
        Chip {
            cpu: cpu::Cpu::new(),
        }
    }
    pub fn load_rom(&mut self) {
        let contents = fs::read("./src/roms/BC_test.ch8").expect("File");
        self.cpu.init_ram(contents);
    }

    pub fn update(&mut self) {
        let byte1: u8 = self.cpu.ram[self.cpu.pc as usize];
        let byte2: u8 = self.cpu.ram[(self.cpu.pc + 1) as usize];
        let val = (byte1 & 0xF) as u16;
        let op_data = Op {
            op: (byte1 >> 4) & 0xF,
            x: (byte1 & 0xF),
            y: ((byte2 >> 4) & 0xF),
            n: (byte2 & 0xF),
            nn: byte2,
            nnn: ((val << 8) | (byte2 as u16)) as u16,
        };
        self.cpu.execute_op(op_data);
    }
}

struct Op {
    op: u8,
    x: u8,
    y: u8,
    n: u8,
    nn: u8,
    nnn: u16,
}
