extern crate minifb;
use minifb::{Key, Scale, Window, WindowOptions};

use super::cpu;
use super::cpu::disassembler;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct WindowApp {
    cpu: cpu::CPU,
    items: Vec<disassembler::Dissemble>,
    offset: u16,
}

impl WindowApp {
    pub fn new(cpu: cpu::CPU) -> WindowApp {
        let mut app = WindowApp {
            cpu: cpu,
            items: vec![],
            offset: 0,
        };

        app.items = app.cpu.disassemble_program();

        return app;
    }

    pub fn run(&mut self) {
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        let mut window = Window::new(
            "Test - ESC to exit",
            WIDTH,
            HEIGHT,
            WindowOptions {
                scale: Scale::X16,
                ..WindowOptions::default()
            },
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        while window.is_open() && !window.is_key_down(Key::Escape) {
            let mut addr: u32 = 0;
            
            if window.is_key_down(Key::Key1) { self.cpu.press_key(Some(0x1)); }
            else if window.is_key_down(Key::Key2) { self.cpu.press_key(Some(0x2)); }
            else if window.is_key_down(Key::Key3) { self.cpu.press_key(Some(0x3)); }
            else if window.is_key_down(Key::Key4) { self.cpu.press_key(Some(0xC)); }
            else if window.is_key_down(Key::Q) { self.cpu.press_key(Some(0x4)); }
            else if window.is_key_down(Key::W) { self.cpu.press_key(Some(0x5)); }
            else if window.is_key_down(Key::E) { self.cpu.press_key(Some(0x6)); }
            else if window.is_key_down(Key::R) { self.cpu.press_key(Some(0xD)); }
            else if window.is_key_down(Key::A) { self.cpu.press_key(Some(0x7)); }
            else if window.is_key_down(Key::S) { self.cpu.press_key(Some(0x8)); }
            else if window.is_key_down(Key::D) { self.cpu.press_key(Some(0x9)); }
            else if window.is_key_down(Key::F) { self.cpu.press_key(Some(0xE)); }
            else if window.is_key_down(Key::Z) { self.cpu.press_key(Some(0xA)); }
            else if window.is_key_down(Key::X) { self.cpu.press_key(Some(0x0)); }
            else if window.is_key_down(Key::C) { self.cpu.press_key(Some(0xB)); }
            else if window.is_key_down(Key::V) { self.cpu.press_key(Some(0xF)); }
            else { self.cpu.press_key(None); }

            self.cpu.do_cycle();

            for y in 0..32 {
                for x in 0..64 {
                    if self.cpu.gfx[x][y] == true {
                        buffer[addr as usize] = 0xFFF;
                    } else {
                        buffer[addr as usize] = 0x000;
                    }
                    addr += 1;
                }
            }
            
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        }
    }
}
