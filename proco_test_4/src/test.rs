#[test]
fn jmp() {
    // add will be skipped by the first jump and the the indirect jmp will
    // branch to 0x24 due to memory init
    let (mut cpu, mut memory) = setup_simple_cpu("
        jmp #0x14
        add #0, r0
        jmp (r0)+");
    cpu.registers[0] = 0;
    let mem = &mut memory;
    mem.write_u16(0, 0x24);
    cpu.step(mem);
    assert_eq!(cpu.registers[PC], 0x14);
    cpu.step(mem);
    assert_eq!(cpu.registers[PC], 0x24);
    assert_eq!(cpu.registers[0], 2);
}
#[test]
fn movelh() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        move	#0, R0
        move	#0xCAFE, (R0)
        move	#0xFADE, R1
        move	#0xBEEF, R2
        move.l	(R0), R1
        move.h	(R0), R2
        ");
    let mem = &mut memory;
    // move	#0, R0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0);
    // move	#0xCAFE, (R0)
    cpu.step(mem);
    assert_eq!(mem.read_u16(0), 0xcafe);
    // move	#0xFADE, R1
    cpu.step(mem);
    assert_eq!(cpu.registers[1], 0xfade);
    // move	#0xBEEF, R2
    cpu.step(mem);
    assert_eq!(cpu.registers[2], 0xbeef);
    // move.l	(R0), R1
    cpu.step(mem);
    assert_eq!(cpu.registers[1], 0xfafe);
    // move.h	(R0), R2
    cpu.step(mem);
    assert_eq!(cpu.registers[2], 0xbeca);
}
#[test]
fn test_move() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        move #0xcafe, r0
        move r0, r1
        move #0, r2
        move #4, r3
        move r0, (r2)
        move r0, (r3)+
        move #0xab, r0
        move r0, -(r3)
        move #0xcafe, r0
        move.l #0xab, r0
        move.h #0xcd, r0
        move #0xdead, r1
        move.h r1, r0
        move.l r0, r1
        move #0xcafe, r0
        move r0, @0
        move #0, r3
        move.l #0xef, (r3)+
        move @0, r1
        move #0, r2
        move #0, r3
        move.l @0, r2
        move.h @0, r3
        move #0xabcd, r0
        move.l #1, r0
        ");
    let mem = &mut memory;
    // move #0xcafe, r0
    cpu.step(mem);
    assert_eq!(0xcafe, cpu.registers[0]);
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    // move r0, r1
    cpu.step(mem);
    assert_eq!(0xcafe, cpu.registers[1]);
    // move #0, r2
    cpu.step(mem);
    assert_eq!(0, cpu.registers[2]);
    assert!(cpu.get_z());
    assert!(!cpu.get_n());
    // move #4, r3
    cpu.step(mem);
    assert_eq!(4, cpu.registers[3]);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // move r0, (r2)
    cpu.step(mem);
    assert_eq!(mem.read_u16(0), 0xcafe);
    assert!(cpu.get_n());
    assert!(!cpu.get_z());
    // move r0, (r3)+
    cpu.step(mem);
    assert_eq!(mem.read_u16(4), 0xcafe);
    assert_eq!(cpu.registers[3], 6);
    assert!(cpu.get_n());
    assert!(!cpu.get_z());
    // move #0xab, r0
    cpu.step(mem);
    assert_eq!(0xab, cpu.registers[0]);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // move r0, -(r3)
    cpu.step(mem);
    assert_eq!(mem.read_u16(4), 0xab);
    assert_eq!(cpu.registers[3], 4);
    assert!(!cpu.get_n());
    assert!(!cpu.get_z());
    // move #0xcafe, r0
    cpu.step(mem);
    // move.l #0xab, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xcaab);
    // move.h #0xcd, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xca00);
    // move #0xdead, r1
    cpu.step(mem);
    assert_eq!(cpu.registers[1], 0xdead);
    // move.h r1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xcade);
    // move.l r0, r1
    cpu.step(mem);
    assert_eq!(cpu.registers[1], 0xdede);
    // move #0xcafe, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xcafe);
    // move r0, @0
    cpu.step(mem);
    assert_eq!(mem.read_u16(0), 0xcafe);
    // move #0, r3
    cpu.step(mem);
    assert_eq!(cpu.registers[3], 0);
    // move.l #0xef, (r3)+
    cpu.step(mem);
    assert_eq!(cpu.registers[3], 1);
    assert_eq!(mem.read_u16(0), 0xcaef);
    // mem[0] is 0xcaef
    // move @0, r1
    cpu.step(mem);
    assert_eq!(cpu.registers[1], 0xcaef);
    // move #0, r2
    cpu.step(mem);
    assert_eq!(cpu.registers[2], 0);
    // move #0, r3
    cpu.step(mem);
    assert_eq!(cpu.registers[3], 0);
    // move.l @0, r2
    cpu.step(mem);
    assert_eq!(cpu.registers[2], 0x00ef);
    // mem[0] is 0xcaef so move 0xca in r3 low
    // move.h @0, r3
    cpu.step(mem);
    assert_eq!(cpu.registers[3], 0x00ca);
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    // move #0xabcd, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xabcd);
    assert!(cpu.get_n());
    assert!(!cpu.get_z());
    // move.l #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xab01);
    assert!(!cpu.get_n()); // not n because the SOURCE is not negative.
    assert!(!cpu.get_z());
}
#[test]
fn push() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        push r0
        push (r1)+
        push (r1)
        push -(r1)
        push r1
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0x7ade;
    cpu.registers[1] = 0;
    cpu.registers[SP] = 0xEA;
    mem.write_u16(0, 0xabcd);
    mem.write_u16(2, 0x1234);
    // r0: 7ade, r1: 0, @0: abcd, @2: 1234, sp: EA
    // push r0
    cpu.step(mem);
    assert_eq!(mem.read_u16(0xe8), 0x7ade);
    assert_eq!(cpu.registers[SP], 0xe8);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // push (r1)+
    cpu.step(mem);
    assert_eq!(mem.read_u16(0xe6), 0xabcd);
    assert_eq!(cpu.registers[SP], 0xe6);
    assert_eq!(cpu.registers[1], 2);
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    // push (r1)
    cpu.step(mem);
    assert_eq!(mem.read_u16(0xe4), 0x1234);
    assert_eq!(cpu.registers[SP], 0xe4);
    assert_eq!(cpu.registers[1], 2);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // push -(r1)
    cpu.step(mem);
    assert_eq!(mem.read_u16(0xe2), 0xabcd);
    assert_eq!(cpu.registers[SP], 0xe2);
    assert_eq!(cpu.registers[1], 0);
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    // push r1
    cpu.step(mem);
    assert_eq!(mem.read_u16(0xe0), 0);
    assert_eq!(cpu.registers[SP], 0xe0);
    assert!(cpu.get_z());
    assert!(!cpu.get_n());
}
#[test]
fn pop() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        pop r0
        pop (r1)+
        pop (r1)
        pop -(r1)
        pop r1
        pop r2
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0x7ade;
    cpu.registers[1] = 0;
    cpu.registers[SP] = 0xE0;
    mem.write_u16(0xe0, 0x1122);
    mem.write_u16(0xe2, 0x3344);
    mem.write_u16(0xe4, 0x5566);
    mem.write_u16(0xe6, 0x7788);
    mem.write_u16(0xe8, 0x9999);
    mem.write_u16(0xea, 0);
    // pop r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0x1122);
    assert_eq!(cpu.registers[SP], 0xe2);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // pop (r1)+
    cpu.step(mem);
    assert_eq!(mem.read_u16(0), 0x3344);
    assert_eq!(cpu.registers[SP], 0xe4);
    assert_eq!(cpu.registers[1], 2);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // pop (r1)
    cpu.step(mem);
    assert_eq!(mem.read_u16(2), 0x5566);
    assert_eq!(cpu.registers[SP], 0xe6);
    assert_eq!(cpu.registers[1], 2);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // pop -(r1)
    cpu.step(mem);
    assert_eq!(mem.read_u16(0), 0x7788);
    assert_eq!(cpu.registers[SP], 0xe8);
    assert_eq!(cpu.registers[1], 0);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // pop r1
    cpu.step(mem);
    assert_eq!(cpu.registers[1], 0x9999);
    assert_eq!(cpu.registers[SP], 0xea);
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    // pop r2
    cpu.step(mem);
    assert_eq!(cpu.registers[2], 0);
    assert_eq!(cpu.registers[SP], 0xec);
    assert!(cpu.get_z());
    assert!(!cpu.get_n());
}

#[test]
fn add() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        add #1, r0
        add #0xfffe, r0
        add #1, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0;

    // add #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 1);
    assert!(!cpu.get_c());
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());

    // add #0xfffe, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xffff);
    assert!(!cpu.get_c());
    assert!(!cpu.get_z());
    assert!(cpu.get_n());

    // add #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0);
    assert!(cpu.get_c());
    assert!(cpu.get_z());
    assert!(!cpu.get_n());
}

#[test]
fn sub() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        sub #1, r0
        sub #0xfffe, r0
        sub #1, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0;

    // sub #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xffff);
    assert!(cpu.get_c());
    assert!(!cpu.get_z());
    assert!(cpu.get_n());

    // sub #0xfffe, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 1);
    assert!(!cpu.get_c());
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());

    // sub #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0);
    assert!(!cpu.get_c());
    assert!(cpu.get_z());
    assert!(!cpu.get_n());
}

#[test]
fn cmp() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        cmp #1, r0
        move #0xfffe, r0
        cmp #0xfffe, r0
        cmp #0xfffc, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0;

    // cmp #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0);
    assert!(cpu.get_c());
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    // move #0xfffe, r0
    cpu.step(mem);
    // cmp #0xfffe, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xfffe);
    assert!(!cpu.get_c());
    assert!(cpu.get_z());
    assert!(!cpu.get_n());

    // cmp #0xfffc, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xfffe);
    assert!(!cpu.get_c());
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
}

#[test]
fn lsl() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        lsl #8, r0
        lsl #1, r0
        lsl #8, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0xff;

    // lsl #8, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xff00);
    assert!(!cpu.get_c());
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    // lsl #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xfe00);
    assert!(cpu.get_c());
    assert!(!cpu.get_z());
    assert!(cpu.get_n());

    // lsl #8, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0);
    assert!(!cpu.get_c());
    assert!(cpu.get_z());
    assert!(!cpu.get_n());
}

#[test]
fn and_() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        and #0x0f, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0xdead;
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0x000d);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    assert!(!cpu.get_c());
}
#[test]
fn or_() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        or #0x0f, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0xdead;
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xdeaf);
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    assert!(!cpu.get_c());
}

#[test]
fn xor() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        xor #0x0f, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0xdead;
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0xdea2);
    assert!(!cpu.get_z());
    assert!(cpu.get_n());
    assert!(!cpu.get_c());
}
#[test]
fn not_() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        not r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0b1010101010101010;
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0b0101010101010101);
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    assert!(!cpu.get_c());
}

#[test]
fn bxx() {
    let tests = [
        // (Instruction, branch destination pc value, state for no branch,
        // state for branch). 0b1000 for no branch means do not test no
        // branch
        ("bcc #4", 0x14u16, 0b111u16, 0b110u16),
        ("bcs #4", 0x14, 0b110, 0b111),
        ("beq #4", 0x14, 0b101, 0b111),
        ("bne #4", 0x14, 0b111, 0b101),
        ("blt #4", 0x14, 0b110, 0b111),
        ("ble #4", 0x14, 0b100, 0b101),
        ("ble #4", 0x14, 0b100, 0b110),
        ("ble #4", 0x14, 0b100, 0b111),
        ("bgt #4", 0x14, 0b111, 0b110),

        ("bge #4", 0x14, 0b101, 0b100),
        ("bge #4", 0x14, 0b101, 0b110),
        ("bge #4", 0x14, 0b101, 0b111),

        ("bra #4", 0x14, 0b1000, 0b000),
        ("bra #4", 0x14, 0b1000, 0b001),
        ("bra #4", 0x14, 0b1000, 0b010),
        ("bra #4", 0x14, 0b1000, 0b011),
        ("bra #4", 0x14, 0b1000, 0b100),
        ("bra #4", 0x14, 0b1000, 0b101),
        ("bra #4", 0x14, 0b1000, 0b110),
        ("bra #4", 0x14, 0b1000, 0b111),

        ("bcc #-4", 0xc, 0b111, 0b110),
        ("bcs #-4", 0xc, 0b110, 0b111),
        ("beq #-4", 0xc, 0b101, 0b111),
        ("bne #-4", 0xc, 0b111, 0b101),
        ("blt #-4", 0xc, 0b110, 0b111),
        ("ble #-4", 0xc, 0b100, 0b101),
        ("ble #-4", 0xc, 0b100, 0b110),
        ("ble #-4", 0xc, 0b100, 0b111),
        ("bgt #-4", 0xc, 0b111, 0b110),
        ("bge #-4", 0xc, 0b101, 0b100),
        ("bge #-4", 0xc, 0b101, 0b110),
        ("bge #-4", 0xc, 0b101, 0b111),

        ("bra #-4", 0xc, 0b1000, 0b000),
        ("bra #-4", 0xc, 0b1000, 0b001),
        ("bra #-4", 0xc, 0b1000, 0b010),
        ("bra #-4", 0xc, 0b1000, 0b011),
        ("bra #-4", 0xc, 0b1000, 0b100),
        ("bra #-4", 0xc, 0b1000, 0b101),
        ("bra #-4", 0xc, 0b1000, 0b110),
        ("bra #-4", 0xc, 0b1000, 0b111),
    ];
    for (code, pc, flag_no_branch, flags_branch) in tests.iter() {
        // Test no branch
        if *flag_no_branch != 0b1000 {
            let (mut cpu, mut mem) = setup_simple_cpu(code);
            cpu.state_register = *flag_no_branch;
            cpu.step(&mut mem);
            assert_eq!(cpu.registers[PC], 0x12);
        }
        // Test branch
        let (mut cpu, mut mem) = setup_simple_cpu(code);
        cpu.state_register = *flags_branch;
        cpu.step(&mut mem);
        assert_eq!(cpu.registers[PC], *pc);
    }
}

#[test]
fn rts() {
    let (mut cpu, mut memory) = setup_simple_cpu("rts");
    let mem = &mut memory;
    cpu.registers[SP] = 0xf0;
    mem.write_u16(0xf0, 0xaa);
    cpu.step(mem);
    assert_eq!(cpu.registers[SP], 0xf2);
    assert_eq!(cpu.registers[PC], 0xaa);
}

#[test]
fn trigger_interrupt() {
    let (mut cpu, mut memory) = setup_simple_cpu("add #0, r0");
    let mem = &mut memory;
    cpu.registers[SP] = 0xf0;
    cpu.state_register = 0xdeaf;
    mem.write_u8(3, 0xaa);
    cpu.trigger_interrupt(mem, 3);
    assert_eq!(cpu.registers[SP], 0xec);
    assert_eq!(cpu.registers[PC], 0xaa);
    assert_eq!(mem.read_u16(0xec), 0xdeaf); // old flags
    assert_eq!(mem.read_u16(0xee), 0x10);   // old pc
}

#[test]
fn rte() {
    let (mut cpu, mut memory) = setup_simple_cpu("rte");
    let mem = &mut memory;
    cpu.registers[SP] = 0xf0;
    mem.write_u16(0xf0, 0xdead);
    mem.write_u16(0xf2, 0xaa);
    cpu.step(mem);
    assert_eq!(cpu.registers[SP], 0xf4);
    assert_eq!(cpu.registers[PC], 0xaa);
    assert_eq!(cpu.state_register, 0xdead);
}

#[test]
fn bsr_jsr() {
    let tests = [
        // (instruction, sp before, sp after, pc after)
        ("bsr #4", 0xf0, 0xee, 0x14),
        ("bsr #-4", 0xf0, 0xee, 0x0c),
        ("jsr #0x22", 0xf0, 0xee, 0x22),
    ];
    for (code, sp_before, sp_after, pc_after) in tests.iter() {
        let (mut cpu, mut memory) = setup_simple_cpu(code);
        let mem = &mut memory;
        cpu.registers[SP] = *sp_before;
        cpu.step(mem);
        assert_eq!(cpu.registers[PC], *pc_after);
        assert_eq!(cpu.registers[SP], *sp_after);
        // +2 because the pushed PC is the PC of the instruction that follows a bsr/jsr
        assert_eq!(mem.read_u16(cpu.registers[SP]), crate::game::RESET_ADDR+2);
    }
}
#[test]
fn jxx() {
    let tests = [
        // (Instruction, branch destination pc value, state for no branch,
        // state for branch). 0b1000 for no branch means do not test no
        // branch
        ("jcc #0x14", 0x14u16, 0b111u16, 0b110u16),
        ("jcs #0x14", 0x14, 0b110, 0b111),
        ("jeq #0x14", 0x14, 0b101, 0b111),
        ("jne #0x14", 0x14, 0b111, 0b101),
        ("jlt #0x14", 0x14, 0b110, 0b111),
        ("jle #0x14", 0x14, 0b100, 0b101),
        ("jle #0x14", 0x14, 0b100, 0b110),
        ("jle #0x14", 0x14, 0b100, 0b111),
        ("jgt #0x14", 0x14, 0b111, 0b110),

        ("jge #0x14", 0x14, 0b101, 0b100),
        ("jge #0x14", 0x14, 0b101, 0b110),
        ("jge #0x14", 0x14, 0b101, 0b111),

        ("jmp #0x14", 0x14, 0b1000, 0b000),
        ("jmp #0x14", 0x14, 0b1000, 0b001),
        ("jmp #0x14", 0x14, 0b1000, 0b010),
        ("jmp #0x14", 0x14, 0b1000, 0b011),
        ("jmp #0x14", 0x14, 0b1000, 0b100),
        ("jmp #0x14", 0x14, 0b1000, 0b101),
        ("jmp #0x14", 0x14, 0b1000, 0b110),
        ("jmp #0x14", 0x14, 0b1000, 0b111),
    ];
    for (code, pc, flag_no_branch, flags_branch) in tests.iter() {
        // Test no branch
        if *flag_no_branch != 0b1000 {
            let (mut cpu, mut mem) = setup_simple_cpu(code);
            cpu.state_register = *flag_no_branch;
            cpu.step(&mut mem);
            assert_eq!(cpu.registers[PC], 0x12);
        }
        // Test branch
        let (mut cpu, mut mem) = setup_simple_cpu(code);
        cpu.state_register = *flags_branch;
        cpu.step(&mut mem);
        assert_eq!(cpu.registers[PC], *pc);
    }
}
#[test]
fn instructions_clears_carry() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        move #0, r0
        push r0
        pop r0
        and #0, r0
        or #0, r0
        xor #0, r0
        not r0
        ");
    let mem = &mut memory;
    cpu.registers[SP] = 0xf0;
    // move #0, r0
    cpu.set_c();
    cpu.step(mem);
    assert!(!cpu.get_c());
    // push r0
    cpu.set_c();
    cpu.step(mem);
    assert!(!cpu.get_c());
    // pop r0
    cpu.set_c();
    cpu.step(mem);
    assert!(!cpu.get_c());
    // and #0, r0
    cpu.set_c();
    cpu.step(mem);
    assert!(!cpu.get_c());
    // or #0, r0
    cpu.set_c();
    cpu.step(mem);
    assert!(!cpu.get_c());
    // xor #0, r0
    cpu.set_c();
    cpu.step(mem);
    assert!(!cpu.get_c());
    // not r0
    cpu.set_c();
    cpu.step(mem);
    assert!(!cpu.get_c());
}
#[test]
fn lsr() {
    let (mut cpu, mut memory) = setup_simple_cpu("
        lsr #8, r0
        lsr #1, r0
        lsr #8, r0
        ");
    let mem = &mut memory;
    cpu.registers[0] = 0xffff;

    // lsr #8, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0x00ff);
    assert!(cpu.get_c());
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());
    // lsr #1, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0x007f);
    assert!(cpu.get_c());
    assert!(!cpu.get_z());
    assert!(!cpu.get_n());

    // lsl #8, r0
    cpu.step(mem);
    assert_eq!(cpu.registers[0], 0);
    assert!(!cpu.get_c());
    assert!(cpu.get_z());
    assert!(!cpu.get_n());
}

