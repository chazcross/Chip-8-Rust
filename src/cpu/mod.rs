use rand;
use std::fs::File;
use std::io::Read;

#[cfg(test)]
mod cpu_tests;
pub mod disassembler;

pub struct CPU {
    pub opcode: u16,
    pub memory: [u8; 4096],
    pub registers: [u8; 16],
    pub i_register: u16,
    pub program_counter: u16,
    pub gfx: [[bool; 32]; 64], //   [bool; 64 * 32],
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub stack: Vec<u16>,
    pub sp: u8,
    pub key: [u8; 16],
    pub key_press: u8,
    pub program_size: u16,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            opcode: 0,
            memory: [0; 4096],
            registers: [0; 16],
            i_register: 0,
            program_counter: 0x200,
            gfx: [[false; 32]; 64], // [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![],
            sp: 0,
            key: [0; 16],
            key_press: 0,
            program_size: 0,
        };

        cpu.load_fonts();

        return cpu;
    }

    pub fn read_file(&mut self) {
        let file = File::open("../roms/PONG.c8").unwrap();

        for byte in file.bytes() {
            self.memory[self.program_counter as usize] = byte.unwrap();
            self.program_counter += 1;
        }

        self.program_size = self.program_counter;
        self.program_counter = 0x200;
    }

    pub fn do_cycle(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        self.fetch_opcode(self.program_counter as usize);
        self.execute_opcode();
    }

    fn fetch_opcode(&mut self, memory_location: usize) {
        let a = self.memory[memory_location] as u16;
        let b = self.memory[memory_location + 1] as u16;
        self.opcode = a << 8 | b;
        self.program_counter += 2;
    }

    fn load_fonts(&mut self) {
        for i in 0..FONT_SET.len() {
            self.memory[i] = FONT_SET[i];
        }
    }

    pub fn execute_opcode(&mut self) {
        let nibble = self.opcode & 0xF000;

        let x: u8 = ((self.opcode & 0x0F00) >> 8) as u8;
        let y: u8 = ((self.opcode & 0x00F0) >> 4) as u8;
        let nn: u8 = (self.opcode & 0x00FF) as u8;
        let nnn: u16 = self.opcode & 0x0FFF;
        let n: u8 = (self.opcode & 0x000F) as u8;

        match nibble {
            0x0000 => match self.opcode & 0x0FFF {
                0x0E0 => self.op_00e0(),
                0x00EE => self.op_00ee(),
                _ => self.op_ni(),
            },
            0x1000 => self.op_1nnn(nnn),
            0x2000 => self.op_2nnn(nnn),
            0x3000 => self.op_3xnn(x, nn),
            0x4000 => self.op_4xnn(x, nn),
            0x5000 => self.op_5xy0(x, y),
            0x6000 => self.op_6xnn(x, nn),
            0x7000 => self.op_7xnn(x, nn),
            0x8000 => match self.opcode & 0xF00F {
                0x8000 => self.op_8xy0(x, y),
                0x8001 => self.op_8xy1(x, y),
                0x8002 => self.op_8xy2(x, y),
                0x8003 => self.op_8xy3(x, y),
                0x8004 => self.op_8xy4(x, y),
                0x8005 => self.op_8xy5(x, y),
                0x8006 => self.op_8xy6(x, y),
                0x8007 => self.op_8xy7(x, y),
                0x800E => self.op_8xye(x, y),
                _ => self.op_ni(),
            },
            0x9000 => self.op_ni(),
            0xA000 => self.op_annn(nnn),
            0xB000 => self.op_ni(),
            0xC000 => self.op_cxnn(x, nn),
            0xD000 => self.op_dxyn(x as usize, y as usize, n as usize),
            0xE000 => match self.opcode & 0xF0FF {
                0xE0A1 => self.op_exa1(x),
                _ => self.op_ni(),
            },
            0xF000 => match self.opcode & 0xF0FF {
                0xF007 => self.op_fx07(x),
                0xF018 => self.op_fx18(x),
                0xF01E => self.op_fx1e(x),
                0xF015 => self.op_fx15(x),
                0xF029 => self.op_fx29(x),
                0xF033 => self.op_fx33(x),
                0xF065 => self.op_fx65(x),
                _ => self.op_ni(),
            },
            _ => self.op_ni(),
        }
    }

    fn op_ni(&mut self) {
        panic!(
            "{:#x} {:#X} not implemented yet",
            self.program_counter, self.opcode
        );
    }

    fn op_00e0(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                self.gfx[x][y] = false;
            }
        }
    }

    fn op_00ee(&mut self) {
        let pc = self.stack.pop().unwrap();
        self.program_counter = pc;
    }

    fn op_1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.program_counter);
        self.program_counter = nnn;
    }

    fn op_3xnn(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];

        if vx == nn {
            self.program_counter += 2;
        }
    }

    fn op_4xnn(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];

        if vx != nn {
            self.program_counter += 2;
        }
    }

    fn op_5xy0(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if vx == vy {
            self.program_counter += 2;
        }
    }

    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }

    fn op_7xnn(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];
        self.registers[x as usize] = vx.wrapping_add(nn);
    }

    fn op_8xy0(&mut self, x: u8, y: u8) {
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vy;
    }

    fn op_8xy1(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx | vy;
    }

    fn op_8xy2(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx & vy;
    }

    fn op_8xy3(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx ^ vy;
    }

    fn op_8xy4(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, overflow) = vx.overflowing_add(vy);

        self.registers[x as usize] = val;
        self.registers[0xF as usize] = overflow as u8;
    }

    fn op_8xy5(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, borrow) = vx.overflowing_sub(vy);

        self.registers[x as usize] = val;
        self.registers[0xF as usize] = borrow as u8;
    }

    fn op_8xy6(&mut self, x: u8, y: u8) {
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vy >> 1;
        self.registers[0xF as usize] = vy & 0x1;
    }

    fn op_8xy7(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, borrow) = vy.overflowing_sub(vx);

        self.registers[x as usize] = val;
        self.registers[0xF as usize] = borrow as u8;
    }

    fn op_8xye(&mut self, x: u8, y: u8) {
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vy << 1;
        self.registers[0xF as usize] = (vy & 0x80) >> 7;
    }

    fn op_annn(&mut self, nnn: u16) {
        self.i_register = nnn;
    }

    fn op_cxnn(&mut self, x: u8, nn: u8) {
        let random = rand::random::<u8>();
        self.registers[x as usize] = nn & random;
    }

    fn op_dxyn(&mut self, x: usize, y: usize, rows: usize) {
        let vx = self.registers[x];
        let vy = self.registers[y];
        self.registers[0xF] = 0;

        for row in 0..rows {
            let font = self.memory[self.i_register as usize + row];
            let y_pos = ((vy + row as u8) % 32) as usize;

            for column in 0..8 {
                //sprites are 8px wide
                let x_pos = ((vx + column) % 64) as usize;
                let pixel = (font >> (7 - column)) & 1 != 0;
                self.registers[0xF] |= (pixel & self.gfx[x_pos][y_pos]) as u8;
                self.gfx[x_pos][y_pos] ^= pixel;
            }
        }
    }

    fn op_exa1(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        if vx != self.key_press {
            self.program_counter += 2;
        }
    }

    fn op_fx07(&mut self, x: u8) {
        self.registers[x as usize] = self.delay_timer;
    }

    fn op_fx18(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u8;
        self.sound_timer = vx;
    }

    fn op_fx15(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u8;
        self.delay_timer = vx;
    }

    fn op_fx1e(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u16;
        self.i_register += vx;
    }

    fn op_fx29(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u16;

        self.i_register = vx * 5;
    }

    fn op_fx33(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        let hundreds = (vx / 100) as u8;
        let tens = (vx % 100 / 10) as u8;
        let ones = (vx % 10) as u8;

        self.memory[self.i_register as usize] = hundreds;
        self.memory[self.i_register as usize + 1] = tens;
        self.memory[self.i_register as usize + 2] = ones;
    }

    fn op_fx65(&mut self, x: u8) {
        let dl = x + 1;

        for i in 0..dl as u16 {
            let i_val = self.memory[(self.i_register + i) as usize];
            self.registers[i as usize] = i_val;
        }
    }
}

pub static FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0      11110000, 10010000, 10010000, 10010000, 11110000
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

// 0x000-0x1FF - Chip 8 interpreter
// 0x200-0xFFF - Program ROM and work RAM
