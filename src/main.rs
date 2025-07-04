
use clap::Parser;
use std::fs::File;
use std::io::Read;

mod cpu;
mod window;
mod terminal;

#[derive(Parser)]
#[command(name = "chip8")]
#[command(about = "A CHIP-8 emulator (terminal mode by default)")]
struct Args {
    #[arg(long, help = "Run in window mode instead of terminal mode")]
    window: bool,
}

fn read_rom_file(filename: &str) -> Result<Vec<u8>, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)?;
    Ok(bytes)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    if args.window {
        let mut _cpu = cpu::CPU::new();
        let rom_bytes = read_rom_file("roms/PONG.c8")?;
        _cpu.load_program(&rom_bytes);
        let mut gui = window::WindowApp::new(_cpu);
        gui.run();
    } else {
        let _cpu = cpu::CPU::new();
        let mut term = terminal::TerminalApp::new(_cpu);
        term.run()?;
    }

    Ok(())
}
