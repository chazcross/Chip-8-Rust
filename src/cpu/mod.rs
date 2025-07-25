use rand;

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
    pub key_press: Option<u8>,
    pub waiting_for_key: Option<u8>,
    pub program_size: u16,
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            opcode: 0,
            memory: [0; 4096],
            registers: [0; 16], //Registers V0-VF
            i_register: 0, 
            program_counter: 0x200,
            gfx: [[false; 32]; 64], // [false; 64 * 32],
            delay_timer: 0,
            sound_timer: 0,
            stack: vec![],
            sp: 0,
            key_press: None,
            waiting_for_key: None,
            program_size: 0,
        };

        cpu.load_fonts();

        return cpu;
    }

    pub fn load_program(&mut self, bytes: &[u8]) {
        let mut addr = 0x200;
        for byte in bytes {
            self.memory[addr] = *byte;
            addr += 1;
        }
        self.program_size = addr as u16;
        self.program_counter = 0x200;
    }

    pub fn reset(&mut self) {
        self.opcode = 0;
        self.memory = [0; 4096];
        self.registers = [0; 16];
        self.i_register = 0;
        self.program_counter = 0x200;
        self.gfx = [[false; 32]; 64];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.stack.clear();
        self.sp = 0;
        self.key_press = None;
        self.waiting_for_key = None;
        self.program_size = 0;
        
        self.load_fonts();
    }

    pub fn do_cycle(&mut self) {
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if let Some(reg) = self.waiting_for_key {
            if let Some(key) = self.key_press {
                self.registers[reg as usize] = key;
                self.waiting_for_key = None;
            } else {
                return;
            }
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
                0x00E0 => self.op_00e0(),
                0x00EE => self.op_00ee(),
                _ if nnn != 0x00E0 && nnn != 0x00EE => self.op_0nnn(nnn),
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
            0x9000 => self.op_9xy0(x, y),
            0xA000 => self.op_annn(nnn),
            0xB000 => self.op_bnnn(nnn),
            0xC000 => self.op_cxnn(x, nn),
            0xD000 => self.op_dxyn(x as usize, y as usize, n as usize),
            0xE000 => match self.opcode & 0xF0FF {
                0xE09E => self.op_ex9e(x),
                0xE0A1 => self.op_exa1(x),
                _ => self.op_ni(),
            },
            0xF000 => match self.opcode & 0xF0FF {
                0xF007 => self.op_fx07(x),
                0xF00A => self.op_fx0a(x),
                0xF018 => self.op_fx18(x),
                0xF01E => self.op_fx1e(x),
                0xF015 => self.op_fx15(x),
                0xF029 => self.op_fx29(x),
                0xF033 => self.op_fx33(x),
                0xF055 => self.op_fx55(x),
                0xF065 => self.op_fx65(x),
                _ => self.op_ni(),
            },
            _ => self.op_ni(),
        }
    }

    fn op_ni(&mut self) {
        panic!(
            "{:#x} {:#06X} not implemented yet",
            self.program_counter, self.opcode
        );
    }

    // 0NNN: No operation (historically called machine language subroutine, but modern implementations treat as no-op)
    fn op_0nnn(&mut self, _nnn: u16) {
        // No operation - do nothing
    }

    // 00E0: Clear the display
    fn op_00e0(&mut self) {
        for y in 0..32 {
            for x in 0..64 {
                self.gfx[x][y] = false;
            }
        }
    }

    // 00EE: Return from subroutine
    fn op_00ee(&mut self) {
        let pc = self.stack.pop().unwrap();
        self.program_counter = pc;
    }

    // 1NNN: Jump to address NNN
    fn op_1nnn(&mut self, nnn: u16) {
        self.program_counter = nnn;
    }

    // 2NNN: Call subroutine at NNN
    fn op_2nnn(&mut self, nnn: u16) {
        self.stack.push(self.program_counter);
        self.program_counter = nnn;
    }

    // 3XNN: Skip next instruction if VX equals NN
    fn op_3xnn(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];

        if vx == nn {
            self.program_counter += 2;
        }
    }

    // 4XNN: Skip next instruction if VX does not equal NN
    fn op_4xnn(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];

        if vx != nn {
            self.program_counter += 2;
        }
    }

    // 5XY0: Skip next instruction if VX equals VY
    fn op_5xy0(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if vx == vy {
            self.program_counter += 2;
        }
    }

    // 6XNN: Set VX to NN
    fn op_6xnn(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }

    // 7XNN: Add NN to VX (carry flag not changed)
    fn op_7xnn(&mut self, x: u8, nn: u8) {
        let vx = self.registers[x as usize];
        self.registers[x as usize] = vx.wrapping_add(nn);
    }

    // 8XY0: Set VX to the value of VY
    fn op_8xy0(&mut self, x: u8, y: u8) {
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vy;
    }

    // 8XY1: Set VX to VX OR VY
    fn op_8xy1(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx | vy;
    }

    // 8XY2: Set VX to VX AND VY
    fn op_8xy2(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx & vy;
    }

    // 8XY3: Set VX to VX XOR VY
    fn op_8xy3(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vx ^ vy;
    }

    // 8XY4: Add VY to VX. VF is set to 1 when there's a carry, 0 otherwise
    fn op_8xy4(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, overflow) = vx.overflowing_add(vy);

        self.registers[x as usize] = val;
        self.registers[0xF as usize] = overflow as u8;
    }

    // 8XY5: Subtract VY from VX. VF is set to 0 when there's a borrow, 1 otherwise
    fn op_8xy5(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, borrow) = vx.overflowing_sub(vy);

        self.registers[x as usize] = val;
        self.registers[0xF as usize] = (!borrow) as u8;
    }

    // 8XY6: Store the least significant bit of VY in VF and shift VY right by 1, store result in VX
    fn op_8xy6(&mut self, x: u8, y: u8) {
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vy >> 1;
        self.registers[0xF as usize] = vy & 0x1;
    }

    // 8XY7: Set VX to VY minus VX. VF is set to 0 when there's a borrow, 1 otherwise
    fn op_8xy7(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        let (val, borrow) = vy.overflowing_sub(vx);

        self.registers[x as usize] = val;
        self.registers[0xF as usize] = (!borrow) as u8;
    }

    // 8XYE: Store the most significant bit of VY in VF and shift VY left by 1, store result in VX
    fn op_8xye(&mut self, x: u8, y: u8) {
        let vy = self.registers[y as usize];

        self.registers[x as usize] = vy << 1;
        self.registers[0xF as usize] = (vy & 0x80) >> 7;
    }

    // 9XY0: Skip next instruction if VX does not equal VY
    fn op_9xy0(&mut self, x: u8, y: u8) {
        let vx = self.registers[x as usize];
        let vy = self.registers[y as usize];

        if vx != vy {
            self.program_counter += 2;
        }
    }

    // ANNN: Set I to the address NNN
    fn op_annn(&mut self, nnn: u16) {
        self.i_register = nnn;
    }

    // BNNN: Jump to address NNN plus V0
    fn op_bnnn(&mut self, nnn: u16) {
        self.program_counter = self.registers[0] as u16 + nnn;
    }

    // CXNN: Set VX to the result of a bitwise AND operation on a random number and NN
    fn op_cxnn(&mut self, x: u8, nn: u8) {
        let random = rand::random::<u8>();
        self.registers[x as usize] = nn & random;
    }

    // DXYN: Draw a sprite at coordinate (VX, VY) with N bytes of sprite data starting at address I
    fn op_dxyn(&mut self, x: usize, y: usize, rows: usize) {
        let vx = self.registers[x];
        let vy = self.registers[y];
        self.registers[0xF] = 0;

        for row in 0..rows {
            let font = self.memory[self.i_register as usize + row];
            let y_pos = ((vy as u16 + row as u16) % 32) as usize;

            for column in 0..8 {
                //sprites are 8px wide
                let x_pos = ((vx as u16 + column as u16) % 64) as usize;
                let pixel = (font >> (7 - column)) & 1 != 0;
                self.registers[0xF] |= (pixel & self.gfx[x_pos][y_pos]) as u8; //check for collision
                self.gfx[x_pos][y_pos] ^= pixel;
            }
        }
    }

    // EX9E: Skip next instruction if key stored in VX is pressed
    fn op_ex9e(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        if self.is_key_press(vx) {
            self.program_counter += 2;
        }
    }

    // EXA1: Skip next instruction if key stored in VX is not pressed
    fn op_exa1(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        if !self.is_key_press(vx) {
            self.program_counter += 2;
        }
    }

    // FX07: Set VX to the value of the delay timer
    fn op_fx07(&mut self, x: u8) {
        self.registers[x as usize] = self.delay_timer;
    }

    // FX0A: Wait for a keypress and store the result in register VX
    fn op_fx0a(&mut self, x: u8) {
        if let Some(key) = self.key_press {
            self.registers[x as usize] = key;
            self.waiting_for_key = None;
        } else {
            self.waiting_for_key = Some(x);
            self.program_counter -= 2;
        }
    }

    // FX18: Set the sound timer to VX
    fn op_fx18(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u8;
        self.sound_timer = vx;
    }

    // FX15: Set the delay timer to VX
    fn op_fx15(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u8;
        self.delay_timer = vx;
    }

    // FX1E: Add VX to I
    fn op_fx1e(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u16;
        self.i_register += vx;
    }

    // FX29: Set I to the location of the sprite for the character in VX
    fn op_fx29(&mut self, x: u8) {
        let vx = self.registers[x as usize] as u16;

        self.i_register = vx * 5;
    }

    // FX33: Store the binary-coded decimal representation of VX at addresses I, I+1, and I+2
    fn op_fx33(&mut self, x: u8) {
        let vx = self.registers[x as usize];
        let hundreds = (vx / 100) as u8;
        let tens = (vx % 100 / 10) as u8;
        let ones = (vx % 10) as u8;

        self.memory[self.i_register as usize] = hundreds;
        self.memory[self.i_register as usize + 1] = tens;
        self.memory[self.i_register as usize + 2] = ones;
    }

    // FX55: Store V0 to VX (including VX) in memory starting at address I
    fn op_fx55(&mut self, x: u8) {
        let dl = x + 1;

        for i in 0..dl as u16 {
            let reg_val = self.registers[i as usize];
            self.memory[(self.i_register + i) as usize] = reg_val;
        }
    }

    // FX65: Fill V0 to VX (including VX) with values from memory starting at address I
    fn op_fx65(&mut self, x: u8) {
        let dl = x + 1;

        for i in 0..dl as u16 {
            let i_val = self.memory[(self.i_register + i) as usize];
            self.registers[i as usize] = i_val;
        }
    }

    pub fn press_key(&mut self, key: Option<u8>) {
        self.key_press = key;
    }

    fn is_key_press(&mut self, key_code: u8) -> bool {
        if let Some(key) = self.key_press {
            return key == key_code;
        }

        return false;
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
