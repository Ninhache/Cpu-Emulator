use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::instructions::instructions as instructions;

type InstructionFn = fn();

fn to_u5(number: u8) -> [bool; 5] {
    if number > 0x1E {
        panic!("{number} is superior to the limit ! ({})", 0x1E);
    } else {
        let mut result = [false; 5];
        let mut n = number;

        for i in (0..5).rev() {
            result[i] = n & 1 == 1;
            n >>= 1;
        }

        result
    }
}

lazy_static! {
    pub static ref OPCODES: HashMap<[bool; 5], InstructionFn> = {
        let mut map = HashMap::new();
        map.insert(to_u5(0x00), instructions::MOVE as InstructionFn);
        map.insert(to_u5(0x01), instructions::PUSH as InstructionFn);
        map
    };
}


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
}