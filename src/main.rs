use std::io;

mod cpu;
mod window;

fn main() -> Result<(), io::Error> {
    let mut _cpu = cpu::CPU::new();
    _cpu.read_file();

    let mut gui = window::WindowApp::new(_cpu);
    gui.run();

    Ok(())
}
