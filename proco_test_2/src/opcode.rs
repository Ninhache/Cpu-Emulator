use lazy_static::lazy_static;

// type InstructionFn = fn();

// fn to_u5(number: u8) -> [bool; 5] {
//     if number > 0x1E {
//         panic!("{number} is superior to the limit ! ({})", 0x1E);
//     } else {
//         let mut result = [false; 5];
//         let mut n = number;

//         for i in (0..5).rev() {
//             result[i] = n & 1 == 1;
//             n >>= 1;
//         }

//         result
//     }
// }

// lazy_static! {
//     pub static ref OPCODES: HashMap<[bool; 5], InstructionFn> = {
//         let mut map = HashMap::new();
//         map.insert(to_u5(0x00), instructions::MOVE as InstructionFn);
//         map.insert(to_u5(0x01), instructions::PUSH as InstructionFn);
//         map
//     };
// }


// fn create_opcode_map() -> HashMap<[bool; 5], InstructionFn>{
//     const MAP: HashMap<[bool; 5], InstructionFn> = HashMap::new();
//     MAP.insert(toU5(0), instructions::MOVE as InstructionFn);
//     MAP.insert(toU5(1), instructions::PUSH as InstructionFn);
//     MAP
// }
// const OPCODES: HashMap<[bool; 5], InstructionFn> = create_opcode_map();

pub mod opcode {

    struct _Format0opLayout {
        opcode: [bool; 5],
        op_reserved : [bool; 11],
    }

    struct _Format1opLayout {
        opcode: [bool; 5],
        op_type: [bool; 3],
        op_value: [bool; 8],
    }

    struct _Format2opLayout {
        opcode: [bool; 5],
        op_dest: [bool; 3],
        op_source: [bool; 3],
        op_value: [bool; 5],
    }

    use crate::opcode::lazy_static;
    use std::collections::HashMap;

    lazy_static! {
    pub static ref OPCODES: HashMap<&'static str, u8> = {
        let mut map = HashMap::new();
        map.insert("MOVE",  0x00);
        map.insert("PUSH",  0x01);
        map.insert("POP",   0x02);
        map.insert("ADD",   0x03);
        map.insert("CMP",   0x04);
        map.insert("SUB",   0x05);
        map.insert("LSL",   0x06);
        map.insert("LSR",   0x07);
        map.insert("AND",   0x08);
        map.insert("OR",    0x09);
        map.insert("XOR",   0x0A);
        map.insert("NOT",   0x0B);
        map.insert("BCC",   0x0C);
        map.insert("BGT",   0x0C);
        map.insert("BCS",   0x0D);
        map.insert("BLT",   0x0D);
        map.insert("BEQ",   0x0E);
        map.insert("BNE",   0x0F);
        map.insert("BLE",   0x10);
        map.insert("BGE",   0x11);
        map.insert("BRA",   0x12);
        map.insert("BSR",   0x13);
        map.insert("JCC",   0x14);
        map.insert("JGT",   0x14);
        map.insert("JCS",   0x15);
        map.insert("JLT",   0x15);
        map.insert("JEQ",   0x16);
        map.insert("JNE",   0x17);
        map.insert("JLE",   0x18);
        map.insert("JGE",   0x19);
        map.insert("JMP",   0x1A);
        map.insert("JSR",   0x1B);
        map.insert("RTS",   0x1C);
        map.insert("TRAP",  0x1D);
        map.insert("RTE",   0x1E);

        map
    };
}
}


