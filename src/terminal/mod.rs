extern crate crossterm;

use crossterm::{terminal, event::{self, Event, KeyCode, KeyEvent, KeyEventKind}};
use std::convert::AsRef;
use std::io::stdout;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::text::{Text, Line};
use ratatui::{Frame, Terminal};

use super::cpu;
use super::cpu::disassembler;
use std::fs;
use std::io::Read;

#[derive(PartialEq)]
enum AppState {
    RomSelection,
    Emulating,
    Debugging,
}

pub struct TerminalApp {
    cpu: cpu::CPU,
    items: Vec<disassembler::Dissemble>,
    offset: u16,
    current_key: Option<u8>,
    last_key_time: std::time::Instant,
    app_state: AppState,
    rom_files: Vec<String>,
    selected_rom: usize,
    rom_scroll_offset: usize,
    debug_mode: bool,
    step_requested: bool,
}

impl TerminalApp {
    pub fn new(cpu: cpu::CPU) -> TerminalApp {
        let mut app = TerminalApp {
            cpu: cpu,
            items: vec![],
            offset: 0,
            current_key: None,
            last_key_time: std::time::Instant::now(),
            app_state: AppState::RomSelection,
            rom_files: vec![],
            selected_rom: 0,
            rom_scroll_offset: 0,
            debug_mode: false,
            step_requested: false,
        };

        app.scan_rom_directory();

        return app;
    }

    fn scan_rom_directory(&mut self) {
        if let Ok(entries) = fs::read_dir("roms") {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        if let Some(file_name) = path.file_name() {
                            if let Some(file_str) = file_name.to_str() {
                                // Only include .ch8 files
                                if file_str.ends_with(".ch8") {
                                    self.rom_files.push(file_str.to_string());
                                }
                            }
                        }
                    }
                }
            }
        }
        self.rom_files.sort();
    }

    fn load_selected_rom(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.selected_rom < self.rom_files.len() {
            let rom_path = format!("roms/{}", self.rom_files[self.selected_rom]);
            
            let mut file = fs::File::open(&rom_path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            
            self.cpu.load_program(&bytes);
            self.items = self.cpu.disassemble_program();
            self.offset = 0;
            self.app_state = AppState::Emulating;
        }
        Ok(())
    }

    fn load_selected_rom_debug(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.selected_rom < self.rom_files.len() {
            let rom_path = format!("roms/{}", self.rom_files[self.selected_rom]);
            
            let mut file = fs::File::open(&rom_path)?;
            let mut bytes = Vec::new();
            file.read_to_end(&mut bytes)?;
            
            self.cpu.load_program(&bytes);
            self.items = self.cpu.disassemble_program();
            self.offset = 0;
            self.debug_mode = true;
            self.step_requested = false;
            self.app_state = AppState::Debugging;
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        terminal::enable_raw_mode()?;
        
        let backend = CrosstermBackend::new(stdout());
            let mut terminal = Terminal::new(backend).unwrap();
            terminal.hide_cursor().unwrap();
            terminal.clear().unwrap();

            let mut last_render = std::time::Instant::now();
            let render_interval = std::time::Duration::from_millis(16); // ~60 FPS
            
            loop {
                if event::poll(std::time::Duration::from_millis(1))? {
                    if let Event::Key(key_event) = event::read()? {
                        if self.process_input_event(key_event) {
                            break;
                        }
                    }
                }
                
                match self.app_state {
                    AppState::RomSelection => {
                        // Only redraw if enough time has passed
                        if last_render.elapsed() >= render_interval {
                            terminal
                                .draw(|f| {
                                    let area = f.area();
                                    self.display_rom_selection(f, area);
                                })
                                .unwrap();
                            last_render = std::time::Instant::now();
                        }
                    }
                    AppState::Emulating => {
                        // Check if key has timed out (no repeat event for 100ms means released)
                        if self.current_key.is_some() && self.last_key_time.elapsed() > std::time::Duration::from_millis(100) {
                            self.current_key = None;
                        }
                        
                        // Always apply the current key state
                        self.cpu.press_key(self.current_key);

                        self.cpu.do_cycle();

                        // Only redraw if enough time has passed
                        if last_render.elapsed() >= render_interval {
                            terminal
                                .draw(|mut f| {
                                    let chunks = Layout::default()
                                        .direction(Direction::Horizontal)
                                        .margin(1)
                                        .constraints(
                                            [
                                                Constraint::Percentage(20),
                                                Constraint::Percentage(20),
                                                Constraint::Percentage(60),
                                            ]
                                            .as_ref(),
                                        )
                                        .split(f.area());

                                    self.display_disassemble_program(&mut f, chunks[0]);
                                    self.display_executing_instruction(&mut f, chunks[1]);
                                    self.display_grfx(&mut f, chunks[2])
                                })
                                .unwrap();
                            last_render = std::time::Instant::now();
                        }
                    }
                    AppState::Debugging => {
                        // Check if key has timed out (no repeat event for 100ms means released)
                        if self.current_key.is_some() && self.last_key_time.elapsed() > std::time::Duration::from_millis(100) {
                            self.current_key = None;
                        }
                        
                        // Always apply the current key state
                        self.cpu.press_key(self.current_key);

                        // Only execute next instruction if step was requested
                        if self.step_requested {
                            self.cpu.do_cycle();
                            self.step_requested = false;
                        }

                        // Only redraw if enough time has passed
                        if last_render.elapsed() >= render_interval {
                            terminal
                                .draw(|mut f| {
                                    let chunks = Layout::default()
                                        .direction(Direction::Horizontal)
                                        .margin(1)
                                        .constraints(
                                            [
                                                Constraint::Percentage(20),
                                                Constraint::Percentage(20),
                                                Constraint::Percentage(60),
                                            ]
                                            .as_ref(),
                                        )
                                        .split(f.area());

                                    self.display_disassemble_program(&mut f, chunks[0]);
                                    self.display_executing_instruction(&mut f, chunks[1]);
                                    self.display_grfx(&mut f, chunks[2])
                                })
                                .unwrap();
                            last_render = std::time::Instant::now();
                        }
                    }
                }
                
                // Sleep briefly to prevent excessive CPU usage
                std::thread::sleep(std::time::Duration::from_micros(500));
            }
        
        terminal.clear()?;
        terminal.show_cursor()?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn display_disassemble_program(&mut self, f: &mut Frame, chunk: Rect) {
        let style = Style::default().fg(Color::White);
        let current_style = Style::default().fg(Color::Black).bg(Color::Yellow);

        // Auto-scroll to follow program counter in debug mode
        if self.app_state == AppState::Debugging {
            self.auto_scroll_to_current_instruction(chunk.height as usize);
        }

        let items: Vec<ListItem> = self.items.iter().skip(self.offset as usize).map(|item| {
            let is_current = item.memory_location == self.cpu.program_counter - 2;
            let item_style = if is_current && self.app_state == AppState::Debugging {
                current_style
            } else {
                style
            };
            
            ListItem::new(Line::from(vec![ratatui::text::Span::styled(
                format!(
                    "{:#x} {:#06X} {}",
                    item.memory_location, item.opcode, item.assembly
                ),
                item_style,
            )]))
        }).collect();

        let title = match self.app_state {
            AppState::Debugging => "Assembly - Current instruction highlighted",
            _ => "Assembly - pgup/pgdown to scroll",
        };

        let list_widget = List::new(items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(title),
            );
        
        f.render_widget(list_widget, chunk);
    }

    fn auto_scroll_to_current_instruction(&mut self, visible_height: usize) {
        // Find the index of the current instruction
        let current_pc = self.cpu.program_counter - 2;
        if let Some(current_index) = self.items.iter().position(|item| item.memory_location == current_pc) {
            let available_height = if visible_height > 2 { visible_height - 2 } else { 1 }; // Account for borders
            
            // Check if current instruction is visible
            let current_offset = self.offset as usize;
            if current_index < current_offset {
                // Current instruction is above visible area - scroll up
                self.offset = current_index as u16;
            } else if current_index >= current_offset + available_height {
                // Current instruction is below visible area - scroll down
                self.offset = (current_index - available_height + 1) as u16;
            }
        }
    }

    pub fn display_executing_instruction(&mut self, f: &mut Frame, chunk: Rect) {
        let style = Style::default().fg(Color::White);

        let block = Block::default()
            .borders(Borders::ALL)
;

        let mut keys: String = String::from("");

        match self.cpu.key_press {
            Some(0x0) => keys += " 0",
            Some(0x1) => keys += " 1",
            Some(0x2) => keys += " 2",
            Some(0x3) => keys += " 3",
            Some(0x4) => keys += " 4",
            Some(0x5) => keys += " 5",
            Some(0x6) => keys += " 6",
            Some(0x7) => keys += " 7",
            Some(0x8) => keys += " 8",
            Some(0x9) => keys += " 9",
            Some(0xA) => keys += " A",
            Some(0xB) => keys += " B",
            Some(0xC) => keys += " C",
            Some(0xD) => keys += " D",
            Some(0xE) => keys += " E",
            Some(0xF) => keys += " F",
            _ => {}
        }

        let text = vec![
            Line::from(vec![ratatui::text::Span::styled(format!("Opcode: {:#x}", self.cpu.opcode), style)]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Program Counter: {:#x}", self.cpu.program_counter),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register [I]: {:#x}", self.cpu.i_register),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 0, self.cpu.registers[0]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 1, self.cpu.registers[1]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 2, self.cpu.registers[2]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 3, self.cpu.registers[3]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 4, self.cpu.registers[4]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 5, self.cpu.registers[5]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 6, self.cpu.registers[6]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 7, self.cpu.registers[7]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 8, self.cpu.registers[8]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 9, self.cpu.registers[9]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 10, self.cpu.registers[10]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 11, self.cpu.registers[11]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 12, self.cpu.registers[12]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 13, self.cpu.registers[13]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 14, self.cpu.registers[14]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Register {}: {:#x}", 15, self.cpu.registers[15]),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Delay Counter: {:#x}", self.cpu.delay_timer),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(
                format!("Sound Counter: {:#x}", self.cpu.sound_timer),
                style,
            )]),
            Line::from(vec![ratatui::text::Span::styled(format!("Key: {}", keys), style)]),
        ];

        let paragraph_widget = Paragraph::new(Text::from(text))
            .block(block.clone().title("CPU info"));
        
        f.render_widget(paragraph_widget, chunk);
    }

    pub fn display_grfx(&mut self, f: &mut Frame, chunk: Rect) {
        let style = Style::default().fg(Color::White);

        let has_px = Style::default().fg(Color::White);
        let no_pxx = Style::default().fg(Color::Black);

        let block = Block::default()
            .borders(Borders::ALL)
;

        let mut text = vec![Line::from(vec![ratatui::text::Span::styled("", style)])];

        for y in 0..32 {
            let mut line_spans = vec![];
            for x in 0..64 {
                let color = if self.cpu.gfx[x][y] == true {
                    has_px
                } else {
                    no_pxx
                };

                line_spans.push(ratatui::text::Span::styled(format!("{}", "\u{2588}"), color));
            }
            text.push(Line::from(line_spans));
        }

        let title = match self.app_state {
            AppState::Debugging => "UI - Debug Mode: → to step, Enter to run, Space to toggle, ESC to exit",
            _ => "UI - Press Space for debug mode, ESC to return to ROM selection",
        };
        
        let paragraph_widget = Paragraph::new(Text::from(text))
            .block(block.clone().title(title));
        
        f.render_widget(paragraph_widget, chunk);
    }

    pub fn display_rom_selection(&mut self, f: &mut Frame, area: Rect) {
        let style = Style::default().fg(Color::White);
        let selected_style = Style::default().fg(Color::Black).bg(Color::White);

        // Calculate available height for list items (subtract 2 for borders)
        let available_height = area.height as usize - 2;
        
        // Ensure selected item is visible
        if self.selected_rom < self.rom_scroll_offset {
            self.rom_scroll_offset = self.selected_rom;
        } else if self.selected_rom >= self.rom_scroll_offset + available_height {
            self.rom_scroll_offset = self.selected_rom - available_height + 1;
        }

        let mut list_items: Vec<ListItem> = Vec::new();
        
        // Only show items that fit on screen starting from scroll offset
        let visible_files = self.rom_files.iter()
            .skip(self.rom_scroll_offset)
            .take(available_height)
            .enumerate();
        
        for (display_index, rom_file) in visible_files {
            let actual_index = self.rom_scroll_offset + display_index;
            let item_style = if actual_index == self.selected_rom {
                selected_style
            } else {
                style
            };
            
            list_items.push(ListItem::new(Line::from(vec![
                ratatui::text::Span::styled(rom_file.clone(), item_style)
            ])));
        }

        let list_widget = List::new(list_items)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Select ROM - Use ↑/↓ to navigate, Enter to run, Space for debug mode, Esc to quit"),
            );
        
        f.render_widget(list_widget, area);
    }

    pub fn process_input_event(&mut self, key_event: KeyEvent) -> bool {
        match self.app_state {
            AppState::RomSelection => {
                match key_event.kind {
                    KeyEventKind::Press => {
                        match key_event.code {
                            KeyCode::Up => {
                                if self.selected_rom > 0 {
                                    self.selected_rom -= 1;
                                }
                            }
                            KeyCode::Down => {
                                if self.selected_rom + 1 < self.rom_files.len() {
                                    self.selected_rom += 1;
                                }
                            }
                            KeyCode::Enter => {
                                if let Err(e) = self.load_selected_rom() {
                                    println!("Error loading ROM: {}", e);
                                }
                            }
                            KeyCode::Char(' ') => {
                                if let Err(e) = self.load_selected_rom_debug() {
                                    println!("Error loading ROM in debug mode: {}", e);
                                }
                            }
                            KeyCode::Esc => {
                                println!("Escape pressed, exiting...\n");
                                return true;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            AppState::Emulating => {
                match key_event.kind {
                    KeyEventKind::Press | KeyEventKind::Repeat => {
                        match key_event.code {
                            KeyCode::Char(c) => match c {
                                '1' => { self.current_key = Some(0x1); self.last_key_time = std::time::Instant::now(); }
                                '2' => { self.current_key = Some(0x2); self.last_key_time = std::time::Instant::now(); }
                                '3' => { self.current_key = Some(0x3); self.last_key_time = std::time::Instant::now(); }
                                '4' => { self.current_key = Some(0xC); self.last_key_time = std::time::Instant::now(); }
                                'q' => { self.current_key = Some(0x4); self.last_key_time = std::time::Instant::now(); }
                                'w' => { self.current_key = Some(0x5); self.last_key_time = std::time::Instant::now(); }
                                'e' => { self.current_key = Some(0x6); self.last_key_time = std::time::Instant::now(); }
                                'r' => { self.current_key = Some(0xD); self.last_key_time = std::time::Instant::now(); }
                                'a' => { self.current_key = Some(0x7); self.last_key_time = std::time::Instant::now(); }
                                's' => { self.current_key = Some(0x8); self.last_key_time = std::time::Instant::now(); }
                                'd' => { self.current_key = Some(0x9); self.last_key_time = std::time::Instant::now(); }
                                'f' => { self.current_key = Some(0xE); self.last_key_time = std::time::Instant::now(); }
                                'z' => { self.current_key = Some(0xA); self.last_key_time = std::time::Instant::now(); }
                                'x' => { self.current_key = Some(0x0); self.last_key_time = std::time::Instant::now(); }
                                'c' => { self.current_key = Some(0xB); self.last_key_time = std::time::Instant::now(); }
                                'v' => { self.current_key = Some(0xF); self.last_key_time = std::time::Instant::now(); }
                                ' ' => {
                                    // Toggle to debug mode
                                    self.debug_mode = true;
                                    self.step_requested = false;
                                    self.app_state = AppState::Debugging;
                                }
                                _ => {}
                            }
                            KeyCode::PageUp => {
                                if self.offset != 0 {
                                    self.offset -= 10;
                                }
                            }
                            KeyCode::PageDown => {
                                if self.offset + 1 < self.cpu.program_size {
                                    self.offset += 10;
                                }
                            }
                            KeyCode::Esc => {
                                self.cpu.reset();
                                self.items.clear();
                                self.offset = 0;
                                self.app_state = AppState::RomSelection;
                            }
                            _ => {}
                        }
                    }
                    KeyEventKind::Release => {
                        match key_event.code {
                            KeyCode::Char(c) => match c {
                                '1' | '2' | '3' | '4' | 'q' | 'w' | 'e' | 'r' |
                                'a' | 's' | 'd' | 'f' | 'z' | 'x' | 'c' | 'v' => {
                                    self.current_key = None;
                                }
                                _ => {}
                            }
                            _ => {}
                        }
                    }
                }
            }
            AppState::Debugging => {
                match key_event.kind {
                    KeyEventKind::Press | KeyEventKind::Repeat => {
                        match key_event.code {
                            KeyCode::Char(c) => match c {
                                '1' => { self.current_key = Some(0x1); self.last_key_time = std::time::Instant::now(); }
                                '2' => { self.current_key = Some(0x2); self.last_key_time = std::time::Instant::now(); }
                                '3' => { self.current_key = Some(0x3); self.last_key_time = std::time::Instant::now(); }
                                '4' => { self.current_key = Some(0xC); self.last_key_time = std::time::Instant::now(); }
                                'q' => { self.current_key = Some(0x4); self.last_key_time = std::time::Instant::now(); }
                                'w' => { self.current_key = Some(0x5); self.last_key_time = std::time::Instant::now(); }
                                'e' => { self.current_key = Some(0x6); self.last_key_time = std::time::Instant::now(); }
                                'r' => { self.current_key = Some(0xD); self.last_key_time = std::time::Instant::now(); }
                                'a' => { self.current_key = Some(0x7); self.last_key_time = std::time::Instant::now(); }
                                's' => { self.current_key = Some(0x8); self.last_key_time = std::time::Instant::now(); }
                                'd' => { self.current_key = Some(0x9); self.last_key_time = std::time::Instant::now(); }
                                'f' => { self.current_key = Some(0xE); self.last_key_time = std::time::Instant::now(); }
                                'z' => { self.current_key = Some(0xA); self.last_key_time = std::time::Instant::now(); }
                                'x' => { self.current_key = Some(0x0); self.last_key_time = std::time::Instant::now(); }
                                'c' => { self.current_key = Some(0xB); self.last_key_time = std::time::Instant::now(); }
                                'v' => { self.current_key = Some(0xF); self.last_key_time = std::time::Instant::now(); }
                                ' ' => {
                                    // Toggle back to normal emulation mode
                                    self.debug_mode = false;
                                    self.step_requested = false;
                                    self.app_state = AppState::Emulating;
                                }
                                _ => {}
                            }
                            KeyCode::Right => {
                                // Step to next instruction
                                self.step_requested = true;
                            }
                            KeyCode::Enter => {
                                // Exit debug mode and run normally
                                self.debug_mode = false;
                                self.step_requested = false;
                                self.app_state = AppState::Emulating;
                            }
                            KeyCode::PageUp => {
                                if self.offset != 0 {
                                    self.offset -= 10;
                                }
                            }
                            KeyCode::PageDown => {
                                if self.offset + 1 < self.cpu.program_size {
                                    self.offset += 10;
                                }
                            }
                            KeyCode::Esc => {
                                self.cpu.reset();
                                self.items.clear();
                                self.offset = 0;
                                self.debug_mode = false;
                                self.step_requested = false;
                                self.app_state = AppState::RomSelection;
                            }
                            _ => {}
                        }
                    }
                    KeyEventKind::Release => {
                        match key_event.code {
                            KeyCode::Char(c) => match c {
                                '1' | '2' | '3' | '4' | 'q' | 'w' | 'e' | 'r' |
                                'a' | 's' | 'd' | 'f' | 'z' | 'x' | 'c' | 'v' => {
                                    self.current_key = None;
                                }
                                _ => {}
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        return false;
    }
}
