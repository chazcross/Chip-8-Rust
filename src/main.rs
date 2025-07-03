
mod cpu;
mod window;
mod terminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut _cpu = cpu::CPU::new();
    _cpu.read_file();

    // let mut gui = window::WindowApp::new(_cpu);
    // gui.run();

    let mut term = terminal::TerminalApp::new(_cpu);
    term.run()?;

    Ok(())
}
