use super::CPU;

#[test]
fn op_00e0() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x00E0;

    cpu.execute_opcode();

    let mut has_unset_px = false;

    for y in 0..32 {
        for x in 0..64 {
            has_unset_px &= cpu.gfx[x][y];
        }
    }

    assert_eq!(has_unset_px, false);
}

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
fn op_0nnn_basic() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x0123;
    cpu.program_counter = 0x202;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0x202);
}

#[test]
fn op_0nnn_different_address() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x0ABC;
    cpu.program_counter = 0x300;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0x300);
}

#[test]
fn op_bnnn_basic() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xB123;
    cpu.registers[0] = 0x50;
    cpu.program_counter = 0x200;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0x50 + 0x123);
}

#[test]
fn op_bnnn_with_zero_v0() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xB456;
    cpu.registers[0] = 0x00;
    cpu.program_counter = 0x200;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0x456);
}

#[test]
fn op_bnnn_with_max_v0() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xB100;
    cpu.registers[0] = 0xFF;
    cpu.program_counter = 0x200;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0xFF + 0x100);
}

#[test]
fn op_bnnn_wraparound() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xBFFF;
    cpu.registers[0] = 0xFF;
    cpu.program_counter = 0x200;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0xFF + 0xFFF);
}

#[test]
fn op_0nnn_zero_address() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x0000;
    cpu.program_counter = 0x400;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0x400);
}

#[test]
fn op_0nnn_max_address() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x0FFF;
    cpu.program_counter = 0x500;

    cpu.execute_opcode();
    assert_eq!(cpu.program_counter, 0x500);
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
fn op_5xy0_true() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x5200;
    cpu.registers[2] = 2;
    cpu.registers[0] = 2;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200 + 2);
}

#[test]
fn op_5xy0_false() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x5200;
    cpu.registers[2] = 2;
    cpu.registers[0] = 1;

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
fn op_8xy1() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8431;
    cpu.registers[3] = 0x1;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0x3);
}

#[test]
fn op_8xy1_same() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8431;
    cpu.registers[3] = 0x2;
    cpu.registers[4] = 0x2;

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
fn op_8xy3() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8433;
    cpu.registers[3] = 0x1;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0x3);
}

#[test]
fn op_8xy3_unset() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8433;
    cpu.registers[3] = 0x2;
    cpu.registers[4] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[4], 0x0);
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
    cpu.opcode = 0x8985;
    cpu.registers[9] = 0x2;
    cpu.registers[8] = 0x1;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[0x9], 1);
    assert_eq!(cpu.registers[0xf], 1); // VF=1 when NO borrow occurs
}

#[test]
fn op_8xy5_borrow() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8985;
    cpu.registers[9] = 0x2;
    cpu.registers[8] = 0xFF;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[0x9], 0x3);
    assert_eq!(cpu.registers[0xf], 0); // VF=0 when borrow occurs
}

#[test]
fn op_8xy6() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8986;
    cpu.registers[8] = 0x2;

    cpu.execute_opcode();
    assert_eq!(cpu.registers[0x9], 0x1);
    assert_eq!(cpu.registers[0xF], 0x0);
}

#[test]
fn op_8xy6_shift() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8986;
    cpu.registers[8] = 0x5;

    cpu.execute_opcode();
    assert_eq!(cpu.registers[0x9], 0x2);
    assert_eq!(cpu.registers[0xF], 0x1);
}

#[test]
fn op_8xy7_noborrow() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8987;
    cpu.registers[9] = 0x1;
    cpu.registers[8] = 0x2;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[9], 0x1);
    assert_eq!(cpu.registers[0xf], 1); // VF=1 when NO borrow occurs
}

#[test]
fn op_8xy7_borrow() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x8987;
    cpu.registers[9] = 0x2;
    cpu.registers[8] = 0x1;

    cpu.execute_opcode();

    assert_eq!(cpu.registers[9], 0xFF);
    assert_eq!(cpu.registers[0xf], 0); // VF=0 when borrow occurs
}

#[test]
fn op_8xye() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x898E;
    cpu.registers[8] = 0x2;

    cpu.execute_opcode();
    assert_eq!(cpu.registers[0x9], 0x4);
    assert_eq!(cpu.registers[0xF], 0x0);
}

#[test]
fn op_8xye_shift() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x898E;
    cpu.registers[8] = 0xFF;

    cpu.execute_opcode();
    assert_eq!(cpu.registers[0x9], 0xFE);
    assert_eq!(cpu.registers[0xF], 0x1);
}

#[test]
fn op_9xy0_true() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x9230;
    cpu.registers[2] = 0;
    cpu.registers[3] = 1;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200 + 2);
}

#[test]
fn op_xy0_false() {
    let mut cpu = CPU::new();
    cpu.opcode = 0x3230;
    cpu.registers[2] = 1;
    cpu.registers[3] = 1;

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200);
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
    cpu.press_key(Some(1));

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200);
}

#[test]
fn op_exa1_not_pressed() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xE1A1;
    cpu.registers[0x1] = 1;
    cpu.press_key(Some(2));

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200 + 2);
}

#[test]
fn op_ex9e_pressed() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xE19E;
    cpu.registers[0x1] = 1;
    cpu.press_key(Some(1));

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200 + 2);
}

#[test]
fn op_ex9e_not_pressed() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xE19E;
    cpu.registers[0x1] = 1;
    cpu.press_key(Some(2));

    cpu.execute_opcode();

    assert_eq!(cpu.program_counter, 0x200);
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

#[test]
fn op_fx29() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xFA29;
    cpu.registers[0xA] = 9;

    cpu.execute_opcode();

    assert_eq!(cpu.i_register, 9 * 5);
}

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
fn op_fx55() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xF955;
    cpu.i_register = 0x260;

    cpu.registers[0] = 1;
    cpu.registers[1] = 2;
    cpu.registers[2] = 3;
    cpu.registers[3] = 4;
    cpu.registers[4] = 5;
    cpu.registers[5] = 6;
    cpu.registers[6] = 7;
    cpu.registers[7] = 8;
    cpu.registers[8] = 9;
    cpu.registers[9] = 10;

    cpu.execute_opcode();

    assert_eq!(cpu.memory[0x260], 1);
    assert_eq!(cpu.memory[0x261], 2);
    assert_eq!(cpu.memory[0x262], 3);
    assert_eq!(cpu.memory[0x263], 4);
    assert_eq!(cpu.memory[0x264], 5);
    assert_eq!(cpu.memory[0x265], 6);
    assert_eq!(cpu.memory[0x266], 7);
    assert_eq!(cpu.memory[0x267], 8);
    assert_eq!(cpu.memory[0x268], 9);
    assert_eq!(cpu.memory[0x269], 10);
    assert_eq!(cpu.i_register, 0x260); // I should be unchanged
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

#[test]
fn op_dxyn() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xDAB6;

    cpu.registers[10] = 0x2;
    cpu.registers[11] = 0xc;
    cpu.registers[12] = 0x3f;
    cpu.registers[13] = 0xc;
    cpu.i_register = 0x2EA;

    cpu.memory[0x2EA] = 0x80;
    cpu.memory[0x2EB] = 0x80;
    cpu.memory[0x2EC] = 0x80;
    cpu.memory[0x2ED] = 0x80;
    cpu.memory[0x2EE] = 0x80;
    cpu.memory[0x2EF] = 0x80;

    cpu.execute_opcode();

    assert_eq!(cpu.gfx[2][12], true)
}

#[test]
fn op_fx0a_no_key() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xF50A;
    cpu.program_counter = 0x202;
    
    cpu.execute_opcode();
    
    assert_eq!(cpu.waiting_for_key, Some(5));
    assert_eq!(cpu.program_counter, 0x200);
}

#[test]
fn op_fx0a_with_key() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xF50A;
    cpu.program_counter = 0x202;
    cpu.press_key(Some(0x8));
    
    cpu.execute_opcode();
    
    assert_eq!(cpu.registers[5], 0x8);
    assert_eq!(cpu.waiting_for_key, None);
    assert_eq!(cpu.program_counter, 0x202);
}

#[test]
fn op_fx0a_cycle_waiting() {
    let mut cpu = CPU::new();
    cpu.opcode = 0xF30A;
    cpu.program_counter = 0x202;
    
    cpu.execute_opcode();
    
    assert_eq!(cpu.waiting_for_key, Some(3));
    assert_eq!(cpu.program_counter, 0x200);
    
    cpu.do_cycle();
    
    assert_eq!(cpu.waiting_for_key, Some(3));
    assert_eq!(cpu.program_counter, 0x200);
}

#[test]
fn op_fx0a_cycle_key_pressed() {
    let mut cpu = CPU::new();
    
    cpu.memory[0x200] = 0xF3;
    cpu.memory[0x201] = 0x0A;
    cpu.memory[0x202] = 0x00;
    cpu.memory[0x203] = 0xE0;
    
    cpu.do_cycle();
    
    assert_eq!(cpu.waiting_for_key, Some(3));
    assert_eq!(cpu.program_counter, 0x200);
    
    cpu.press_key(Some(0xF));
    cpu.do_cycle();
    
    assert_eq!(cpu.registers[3], 0xF);
    assert_eq!(cpu.waiting_for_key, None);
    assert_eq!(cpu.program_counter, 0x202);
}
