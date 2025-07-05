use super::CPU;
use std::fmt::Write;

pub struct Dissemble {
    pub memory_location: u16,
    pub opcode: u16,
    pub assembly: String,
}

impl CPU {
    pub fn disassemble_program(&mut self) -> Vec<Dissemble> {
        let mut vec = Vec::<Dissemble>::new();

        let mut counter: u16 = 0x200;

        while counter < self.program_size {
            self.fetch_opcode(counter as usize);

            let diss = decode(self.opcode, counter);
            vec.push(diss);
            counter += 2;
        }

        self.program_counter = 0x200;

        return vec;
    }
}

pub fn decode(opcode: u16, memory_location: u16) -> Dissemble {
    let mut diss = Dissemble {
        memory_location: memory_location.clone(),
        opcode: opcode.clone(),
        assembly: "".to_string(),
    };

    let nibble = opcode & 0xF000;

    let x: u8 = ((opcode & 0x0F00) >> 8) as u8;
    let y: u8 = ((opcode & 0x00F0) >> 4) as u8;
    let nn: u8 = (opcode & 0x00FF) as u8;
    let nnn: u16 = opcode & 0x0FFF;
    let n: u8 = (opcode & 0x000F) as u8;

    match nibble {
        0x0000 => match opcode & 0x0FFF {
            0x00E0 => diss.assembly = "ERASE".to_string(),
            0x00EE => diss.assembly = "Return".to_string(),
            _ if nnn != 0x00E0 && nnn != 0x00EE => write!(diss.assembly, "NOP {:#X}", nnn).unwrap(),
            _ => diss.assembly = "Not implemented yet".to_string(),
        },
        0x1000 => write!(diss.assembly, "GOTO {:#X}", nnn).unwrap(),
        0x2000 => write!(diss.assembly, "DO {:#X}", nnn).unwrap(),
        0x3000 => write!(diss.assembly, "SKF V{}={:#X}", x, nn).unwrap(),
        0x4000 => write!(diss.assembly, "SKF V{}≠{:#X}", x, nn).unwrap(),
        0x5000 => write!(diss.assembly, "SKF V{}=V{}", x, y).unwrap(),
        0x6000 => write!(diss.assembly, "V{}={:#X}", x, nn).unwrap(),
        0x7000 => write!(diss.assembly, "V{}+={:#X}", x, nn).unwrap(),
        0x8000 => match opcode & 0xF00F {
            0x8000 => write!(diss.assembly, "V{}=V{}", x, y).unwrap(),
            0x8001 => write!(diss.assembly, "V{}|=V{}", x, y).unwrap(),
            0x8002 => write!(diss.assembly, "V{}&=V{}", x, y).unwrap(),
            0x8003 => write!(diss.assembly, "V{}^=V{}", x, y).unwrap(),
            0x8004 => write!(diss.assembly, "V{}+=V{}", x, y).unwrap(),
            0x8005 => write!(diss.assembly, "V{}-=V{}", x, y).unwrap(),
            0x8006 => write!(diss.assembly, "V{}=V{}>>1", x, y).unwrap(),
            0x8007 => write!(diss.assembly, "V{}=V{}-V{}", x, y, x).unwrap(),
            0x800E => write!(diss.assembly, "V{}=V{}<<1", x, y).unwrap(),
            _ => write!(diss.assembly, "{:#X} not handled yet", opcode).unwrap(),
        },
        0x9000 => write!(diss.assembly, "SKF V{}≠V{}", x, y).unwrap(),
        0xA000 => write!(diss.assembly, "I={:#X}", nnn).unwrap(),
        0xB000 => write!(diss.assembly, "GOTO V0+{:#X}", nnn).unwrap(),
        0xC000 => write!(diss.assembly, "V{}=RND.{:#X}", x, nn).unwrap(),
        0xD000 => write!(diss.assembly, "Draw {} Rows @X{},Y{}", n, x, y).unwrap(),
        0xe000 => match opcode & 0xF0FF {
            0xE09E => write!(diss.assembly, "SKF V{}=KEY", x).unwrap(),
            0xE0A1 => write!(diss.assembly, "SKF V{}≠KEY", x).unwrap(),
            _ => diss.assembly = "Not implemented yet".to_string(),
        },
        0xf000 => match opcode & 0xF0FF {
            0xF007 => write!(diss.assembly, "V{}=TIME", x).unwrap(),
            0xF00A => write!(diss.assembly, "V{}=KEY", x).unwrap(),
            0xF015 => write!(diss.assembly, "TIME=V{}", x).unwrap(),
            0xF018 => write!(diss.assembly, "TONE=V{}", x).unwrap(),
            0xF01E => write!(diss.assembly, "I=I+V{}", x).unwrap(),
            0xF029 => write!(diss.assembly, "I=DSP,V{}", x).unwrap(),
            0xF033 => write!(diss.assembly, "MI=DEQ,V{}", x).unwrap(),
            0xF055 => write!(diss.assembly, "MI=V0:V{}", x).unwrap(),
            0xF065 => write!(diss.assembly, "V0:V{}=MI", x).unwrap(),
            _ => diss.assembly = "Not implemented yet".to_string(),
        },
        _ => diss.assembly = "Not implemented yet".to_string(),
    }

    return diss;
}
