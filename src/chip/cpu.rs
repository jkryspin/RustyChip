use crate::chip::Op;

#[derive(Clone)]
pub struct Cpu {
    pub pc: i16,
    pub ram: [u8; 4096],
    pub display: [[bool; 32]; 64],
    v: [u8; 16],
    i: u16,
    stack: Vec<u16>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 512,
            ram: [0; 4096],
            display: [[false; 32]; 64],
            stack: Vec::new(),
            v: [0; 16],
            i: 0,
        }
    }

    pub fn init_ram(&mut self, items: Vec<u8>) {
        let sprites = Cpu::get_sprites();
        for (i, sprite) in sprites.iter().enumerate() {
            self.ram[i] = *sprite;
        }
        for (i, el) in items.iter().enumerate() {
            self.ram[512 + i] = *el;
        }
        &self.pretty_print();
    }
    pub fn execute_op(&mut self, op: Op) {
        self.pc += 2;
        match op.op {
            0x0 => {
                if op.nn == 0xE0 {
                    self.display.fill([false; 32]);
                } else if op.nn == 0xEE {
                    self.pc = self.stack.pop().expect("Should never pop too far") as i16;
                } else {
                    self.log_not_implemented();
                }
                println!("poggers")
            }
            1 => self.pc = op.nnn as i16,
            2 => self.stack.push(self.pc as u16),
            3 => {
                if self.v[op.x as usize] == op.nn {
                    self.pc += 2;
                }
            }
            4 => {
                if self.v[op.x as usize] != op.nn {
                    self.pc += 2;
                }
            }
            5 => {
                if self.v[op.x as usize] == self.v[op.y as usize] {
                    self.pc += 2;
                }
            }
            6 => self.v[op.x as usize] = op.nn,
            7 => self.v[op.x as usize] = self.v[op.x as usize].wrapping_add(op.nn),
            8 => match op.n {
                0 => self.v[op.x as usize] = self.v[op.y as usize],
                1 => self.v[op.x as usize] = (self.v[op.x as usize] | self.v[op.y as usize]),
                2 => self.v[op.x as usize] = (self.v[op.x as usize] & self.v[op.y as usize]),
                3 => self.v[op.x as usize] = (self.v[op.x as usize] ^ self.v[op.y as usize]),
                4 => {
                    let result = self.v[op.x as usize] + self.v[op.y as usize];
                    self.v[0xF] = 0;
                    if result > 0xFF {
                        self.v[0xF] = 1;
                    }
                    self.v[op.x as usize] = result;
                }
                5 => {
                    self.v[0xF] = 0;
                    if (self.v[op.x as usize] > self.v[op.y as usize]) {
                        self.v[0xF] = 1;
                    }
                    self.v[op.x as usize] =
                        self.v[op.x as usize].wrapping_sub(self.v[op.y as usize]);
                }
                6 => {
                    self.v[0xF] = self.v[op.x as usize] & 0x1;
                    self.v[op.x as usize] >>= 1;
                }
                7 => {
                    self.v[0xF] = 0;
                    if (self.v[op.y as usize] > self.v[op.x as usize]) {
                        self.v[0xF] = 1;
                    }
                    self.v[op.x as usize] =
                        self.v[op.y as usize].wrapping_sub(self.v[op.x as usize]);
                }
                0xE => {
                    self.v[0xF] = (self.v[op.x as usize] >> 7) & 0x01;
                    self.v[op.x as usize] = self.v[op.x as usize] << 1;
                }
                _ => self.log_not_implemented(),
            },
            0xA => {
                self.i = op.nnn;
            }

            0xD => {
                let xVal = self.v[op.x as usize];
                let yVal = self.v[op.y as usize];

                let xCoord = xVal & 63;
                let yCoord = yVal & 31;
                self.v[0xF as usize] = 0;

                let mut index = 0;
                for row in 0..op.n {
                    let y = yCoord + row;
                    if y >= 32 {
                        break;
                    }

                    let sprite = self.ram[(self.i + index) as usize];

                    for col in 0..8 {
                        let x = xCoord + col;
                        if x >= 64 {
                            break;
                        }

                        println!("{} x", x);
                        println!("{} y", y);
                        let oldPixel = self.display[x as usize][y as usize];
                        let currentPos = 7 - col;
                        let toShift = 1 << currentPos;

                        // check the current bit is on or not
                        let newPixel = (sprite & toShift) != 0x0;
                        if (oldPixel && newPixel) {
                            self.display[x as usize][y as usize] = false;
                            self.v[0xF] = 1;
                        } else if newPixel && !oldPixel {
                            self.display[x as usize][y as usize] = true;
                        }
                    }
                    index += 1;
                }
            }
            0xF => match op.nn {
                0x1E => self.i = self.i.wrapping_add(self.v[op.x as usize].into()),
                0x55 => {
                    for i in 0..=op.x {
                        self.ram[(self.i + i as u16) as usize] = self.v[i as usize];
                    }
                }
                0x33 => {
                    let mut temp = self.v[op.x as usize];
                    self.ram[(self.i + 2) as usize] = temp % 10;
                    temp /= 10;
                    self.ram[(self.i + 1) as usize] = temp % 10;
                    temp /= 10;
                    self.ram[self.i as usize] = temp;
                }
                0x65 => {
                    for i in 0..=op.x {
                        self.v[i as usize] = self.ram[(self.i + i as u16) as usize];
                    }
                }
                _ => {
                    println!("{:#04X?}", op.nn);
                    panic!("No match, you screwed up implementation")
                }
            },
            _ => {
                println!("{:#04X?}", op.op);
                println!("{}", op.op);
                panic!("No match, you screwed up implementation")
            }
        }
    }

    fn log_not_implemented(&self) {
        panic!("no match!! ")
    }

    fn pretty_print(&self) {
        for item in self.ram {
            println!("{:#04X?}", item);
        }
    }
    fn get_sprites() -> [u8; 80] {
        let sprites: [u8; 80] = [
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
            0xF0, 0x80, 0xF0, 0x80, 0x80,
        ]; // F
        return sprites;
    }
}
