
use clap::Parser;

mod cpu;
mod window;
mod terminal;

#[derive(Parser)]
#[command(name = "chip8")]
#[command(about = "A CHIP-8 emulator")]
struct Args {
    #[arg(long, help = "Run in terminal mode instead of window mode")]
    terminal: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let mut _cpu = cpu::CPU::new();
    _cpu.read_file();

    if args.terminal {
        let mut term = terminal::TerminalApp::new(_cpu);
        term.run()?;
    } else {
        let mut gui = window::WindowApp::new(_cpu);
        gui.run();
    }

    Ok(())
}
