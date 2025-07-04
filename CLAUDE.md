# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a CHIP-8 emulator written in Rust that can run in two modes:
- **Window Mode**: A GUI interface using minifb for graphics rendering
- **Terminal Mode**: A TUI interface using ratatui/crossterm for terminal-based display

The emulator includes a built-in disassembler and CPU state visualization for debugging.

## Build and Run Commands

### Basic Usage
```bash
# Build the project
cargo build

# Run in terminal mode (default)
cargo run

# Run in window mode
cargo run -- --window

# Show help
cargo run -- --help
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test <test_name>
```

### Development Commands
```bash
# Check code without building
cargo check

# Build with release optimizations
cargo build --release

# Run with release optimizations
cargo run --release
```

## Architecture

### Core Components

**CPU Module** (`src/cpu/mod.rs`):
- Implements the complete CHIP-8 instruction set
- Handles 4KB memory, 16 registers, timers, and graphics buffer
- Uses a fetch-decode-execute cycle pattern
- Program counter starts at 0x200 (standard CHIP-8 convention)

**Display Modes**:
- **Terminal App** (`src/terminal/mod.rs`): Default mode. Uses ratatui for terminal-based display with three panels:
  - Disassembly view with scrolling
  - CPU state (registers, timers, current instruction)
  - Graphics display using Unicode block characters
- **Window App** (`src/window/mod.rs`): Uses minifb for 64x32 pixel display with 16x scaling

**Disassembler** (`src/cpu/disassembler.rs`):
- Converts opcodes to human-readable assembly
- Generates instruction listing for debugging
- Integrated into both display modes

### Key Technical Details

**Memory Layout**:
- 0x000-0x1FF: CHIP-8 interpreter (font data)
- 0x200-0xFFF: Program ROM and work RAM

**Input Handling**:
- CHIP-8 uses a 16-key hexadecimal keypad (0-F)
- Mapped to QWERTY layout: 1234/QWER/ASDF/ZXCV

**Graphics**:
- 64x32 monochrome display
- XOR-based sprite drawing with collision detection
- Sprites are 8 pixels wide, 1-15 pixels tall

## Testing

The project includes comprehensive unit tests for CPU opcodes in `src/cpu/cpu_tests.rs`. Tests cover:
- All implemented CHIP-8 instructions
- Edge cases like overflow/underflow
- Graphics operations
- Memory operations
- Control flow instructions

## ROM Loading

Currently hardcoded to load "roms/PONG.c8". The `roms/` directory contains:
- `PONG.c8`: Classic Pong game
- `Fishie.ch8`: Another test ROM

## Dependencies

Key external crates:
- `minifb`: Window management and graphics for GUI mode
- `ratatui`: Terminal UI framework
- `crossterm`: Cross-platform terminal manipulation
- `clap`: Command-line argument parsing
- `rand`: Random number generation for CXNN instruction