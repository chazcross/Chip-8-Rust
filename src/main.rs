use std::io;

mod cpu;
mod terminal;

fn main() -> Result<(), io::Error> {
    let mut _cpu = cpu::CPU::new();
    _cpu.read_file();

    let mut term = terminal::TerminalApp::new(_cpu);
    term.run();

    Ok(())
}
