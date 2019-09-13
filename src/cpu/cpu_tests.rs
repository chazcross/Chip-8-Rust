use super::CPU;

#[test]
fn op_00ee() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x00EE;
    cpu.stack.push(0x20ff);
    cpu.program_counter = 0x2022;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0x20ff);
}

#[test]
fn op_1nnn() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x1123;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x0123);
}

#[test]
fn op_2nnn() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x2220;
    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x220);
    assert_eq!(cpu.stack.pop().unwrap(), 0x200);
}

#[test]
fn op_3xnn_true() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x3200;
    cpu.registers[2] = 0;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200 + 2);
}

#[test]
fn op_3xnn_false() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x3200;
    cpu.registers[2] = 1;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200);
}

#[test]
fn op_4xnn_true() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x4202;
    cpu.registers[2] = 0;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200 + 2);
}

#[test]
fn op_4xnn_false() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x4202;
    cpu.registers[2] = 2;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200);
}

#[test]
fn op_6xnn() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x6A02;
    cpu.execute_opcode();

    assert_eq!(cpu.registers[10], 2);
}

#[test]
fn op_7xnn() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x7407;
    cpu.registers[4] = 2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 9);
}

#[test]
fn op_7xnn_overflow() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x74ff;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 1);
    assert_eq!(cpu.registers[0xf as usize], 0);
}

#[test]
fn op_8xy0() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8430;
    cpu.registers[3] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0x2);
}

#[test]
fn op_8xy2_dont_flip() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8432;
    cpu.registers[3] = 0x2;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0x2);
}

#[test]
fn op_8xy2_flip() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8432;
    cpu.registers[3] = 0x2;
    cpu.registers[4] = 0x1;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0);
}

#[test]
fn op_8xy4_nocarry() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8434;
    cpu.registers[3] = 0x2;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0x4);
    assert_eq!(cpu.registers[0xF], 0);
}

#[test]
fn op_8xy4_carry() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8434;
    cpu.registers[3] = 0xFF;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0x1);
    assert_eq!(cpu.registers[0xF], 1);
}

#[test]
fn op_8xy5_noborrow() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8435;
    cpu.registers[3] = 0xFF;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[0xf], 0);
}

#[test]
fn op_8xy5_borrow() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8435;
    cpu.registers[3] = 0x2;
    cpu.registers[4] = 0xFF;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[0xf], 1);
}

#[test]
fn op_annn() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xA2EA;
    cpu.execute_opcode();

    assert_eq!(cpu.i_register, 0x2ea);
}

// #[test]
// fn op_cxnn() {
//     let mut cpu = CPU::new();
//     cpu.opcode = 0xC122;
//     cpu.execute_opcode();
//     //how to test?
//     //Sets VX to the result of a bitwise and operation on a random number (Typically: 0 to 255) and NN.
// }

#[test]
fn op_exa1_pressed() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xE1A1;
    cpu.registers[0x1] = 1;
    cpu.key_press = 1;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200);
}

#[test]
fn op_exa1_not_pressed() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xE1A1;
    cpu.registers[0x1] = 1;
    cpu.key_press = 2;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200 + 2);
}

#[test]
fn op_fx07() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xFA07;
    cpu.delay_timer = 9;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[10], 9);
}

#[test]
fn op_fx15() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xFA15;
    cpu.registers[10] = 9;

    cpu.execute_opcode();

    assert_eq!(cpu.delay_timer, 9);
}

#[test]
fn op_fx18() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xFA18;
    cpu.registers[0xA] = 9;

    cpu.execute_opcode();

    assert_eq!(cpu.sound_timer, 9);
}

#[test]
fn op_fx1e() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xFA1E;
    cpu.registers[0xA] = 9;

    cpu.execute_opcode();

    assert_eq!(cpu.i_register, 9);
}

// #[test]
// fn op_fx29() {
//     let mut cpu = CPU::new();
//     cpu.opcode = 0xFA29;
//     cpu.registers[0xA] = 9;

//     cpu.execute_opcode();

//     assert_eq!(cpu.i_register, 9);
// }

#[test]
fn op_fx33() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xF133;
    cpu.registers[1] = 123;
    cpu.i_register = 0x260;

    cpu.execute_opcode();

    assert_eq!(cpu.memory[0x260], 1);
    assert_eq!(cpu.memory[0x260 + 1], 2);
    assert_eq!(cpu.memory[0x260 + 2], 3);
}

#[test]
fn op_fx65() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xF965;
    cpu.i_register = 0x260;

    cpu.memory[0x260] = 1;
    cpu.memory[0x261] = 2;
    cpu.memory[0x262] = 3;
    cpu.memory[0x263] = 4;
    cpu.memory[0x264] = 5;
    cpu.memory[0x265] = 6;
    cpu.memory[0x266] = 7;
    cpu.memory[0x267] = 8;
    cpu.memory[0x268] = 9;
    cpu.memory[0x269] = 10;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[0], 1);
    assert_eq!(cpu.registers[1], 2);
    assert_eq!(cpu.registers[2], 3);
    assert_eq!(cpu.registers[3], 4);
    assert_eq!(cpu.registers[4], 5);
    assert_eq!(cpu.registers[5], 6);
    assert_eq!(cpu.registers[6], 7);
    assert_eq!(cpu.registers[7], 8);
    assert_eq!(cpu.registers[8], 9);
    assert_eq!(cpu.registers[9], 10);
}
