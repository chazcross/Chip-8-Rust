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

            // for i in buffer.iter_mut() {
            //     *i = 0; // write something more funny here!
            // }

            // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
            window.update_with_buffer(&buffer).unwrap();
        }
    }
}
