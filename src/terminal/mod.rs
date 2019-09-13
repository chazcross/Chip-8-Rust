extern crate crossterm;

use crossterm::{input, InputEvent, KeyEvent, RawScreen};
use std::convert::AsRef;
use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::widgets::{Block, Borders, List, Paragraph, Text, Widget};
use tui::{Frame, Terminal};

use super::cpu;
use super::cpu::disassembler;

pub struct TerminalApp {
    cpu: cpu::CPU,
    items: Vec<disassembler::Dissemble>,
    offset: u16,
}

impl TerminalApp {
    pub fn new(cpu: cpu::CPU) -> TerminalApp {
        let mut app = TerminalApp {
            cpu: cpu,
            items: vec![],
            offset: 0,
        };

        app.items = app.cpu.disassemble_program();

        return app;
    }

    pub fn run(&mut self) {
        if let Ok(_raw) = RawScreen::into_raw_mode() {
            let input = input();
            let mut stdin = input.read_async();

            let backend = CrosstermBackend::new();
            let mut terminal = Terminal::new(backend).unwrap();
            terminal.clear().unwrap();

            loop {
                if let Some(key_event) = stdin.next() {
                    if self.process_input_event(key_event) {
                        break;
                    }
                }

                self.cpu.do_cycle();

                terminal
                    .draw(|mut f| {
                        let chunks = Layout::default()
                            .direction(Direction::Horizontal)
                            .margin(1)
                            .constraints(
                                [Constraint::Percentage(50), Constraint::Percentage(50)].as_ref(),
                            )
                            .split(f.size());

                        self.display_disassemble_program(&mut f, chunks[0]);
                        self.display_executing_instruction(&mut f, chunks[1]);
                    })
                    .unwrap();
            }
        }
    }

    pub fn display_disassemble_program<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let style = Style::default().fg(Color::White);

        let items = self.items.iter().skip(self.offset as usize).map(|item| {
            Text::styled(
                format!(
                    "{:#x} {:#x} {}",
                    item.memory_location, item.opcode, item.assembly
                ),
                style,
            )
        });

        List::new(items)
            .block(Block::default().borders(Borders::ALL).title("List"))
            .render(f, chunk);
    }

    pub fn display_executing_instruction<B: Backend>(&mut self, f: &mut Frame<B>, chunk: Rect) {
        let style = Style::default().fg(Color::White);

        let block = Block::default()
            .borders(Borders::ALL)
            .title_style(Style::default());

        let text = [
            Text::styled(format!("Opcode: {:#x} \n", self.cpu.opcode), style),
            Text::styled(
                format!("Program Counter: {:#x} \n", self.cpu.program_counter),
                style,
            ),
            Text::styled(format!("Register [I]: {} \n", self.cpu.i_register), style),
            Text::styled(
                format!("Register {}: {:#x} \n", 0, self.cpu.registers[0]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 1, self.cpu.registers[1]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 2, self.cpu.registers[2]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 3, self.cpu.registers[3]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 4, self.cpu.registers[4]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 5, self.cpu.registers[5]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 6, self.cpu.registers[6]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 7, self.cpu.registers[7]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 8, self.cpu.registers[8]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 9, self.cpu.registers[9]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 10, self.cpu.registers[10]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 11, self.cpu.registers[11]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 12, self.cpu.registers[12]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 13, self.cpu.registers[13]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 14, self.cpu.registers[14]),
                style,
            ),
            Text::styled(
                format!("Register {}: {:#x} \n", 15, self.cpu.registers[15]),
                style,
            ),
            Text::styled(
                format!("Delay Counter: {:#x} \n", self.cpu.delay_timer),
                style,
            ),
            Text::styled(
                format!("Sound Counter: {:#x} \n", self.cpu.sound_timer),
                style,
            ),
            Text::styled(format!("Key: {:#x} \n", self.cpu.key_press), style),
        ];

        Paragraph::new(text.iter())
            .block(block.clone().title("Left, no wrap"))
            .render(f, chunk);
    }

    pub fn process_input_event(&mut self, key_event: InputEvent) -> bool {
        match key_event {
            InputEvent::Keyboard(k) => match k {
                KeyEvent::Char(c) => match c {
                    'q' => {
                        println!("The 'q' key is hit and the program is not listening to input anymore.\n\n");
                        return true;
                    }
                    _ => {}
                },
                KeyEvent::Up => {
                    if self.offset != 0 {
                        self.offset -= 1;
                    }
                }
                KeyEvent::Down => {
                    if self.offset + 1 < self.cpu.program_size {
                        self.offset += 1;
                    }
                }
                _ => {}
            },
            _ => {}
        }

        return false;
    }
}
