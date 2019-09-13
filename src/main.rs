//extern crate clap;

//use clap::{App, Arg};
use std::io;

mod cpu;
mod terminal;

fn main() -> Result<(), io::Error> {
    // let matches = App::new("chip 8")
    //     .arg(Arg::with_name("disassemble").short("d"))
    //     .arg(Arg::with_name("ui test").short("u"))
    //     .get_matches();

    let mut _cpu = cpu::CPU::new();
    _cpu.read_file();

    let mut term = terminal::TerminalApp::new(_cpu);
    term.run();

    // if matches.is_present("disassemble") {
    //     let diss = disassemble::disassemble_program(_cpu.memory, _cpu.program_size);
    //     for i in diss {
    //         println!("{:#x} {:#x} {}", i.memory_location, i.opcode, i.assembly)
    //     }
    // } else if matches.is_present("ui test") {

    // } else {
    //     while _cpu.program_counter < _cpu.program_size {
    //         _cpu.do_cycle();
    //     }
    // }

    Ok(())
}
