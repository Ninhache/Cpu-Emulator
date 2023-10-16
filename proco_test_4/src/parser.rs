use crate::utils::{BitInt, warning, alert};

use regex::{self, Regex};

enum InstructionFormat {
    Format0op(_Format0opLayout),
    Format1op(_Format1opLayout),
    Format2op(_Format2opLayout),
}

struct _Format0opLayout {
    opcode: BitInt::<5>,
    op_reserved: BitInt::<11>,
}

struct _Format1opLayout {
    opcode: BitInt::<5>,
    op_type: BitInt::<3>,
    op_value: BitInt::<8>,
}

struct _Format2opLayout {
    opcode: BitInt::<5>,
    op_type_dest: BitInt::<3>,
    op_type_source: BitInt::<3>,
    op_value: BitInt::<5>,
}

use std::{collections::HashMap, ops::Deref};

pub fn tokenize(asm_code: &str) -> Vec<&str> {
    let re = Regex::new(r"^(.*?)(?:;|$)").unwrap();

    let mut line = asm_code;
            
    if let Some(tokens) = re.captures(&line) {
        if let Some(without_comment) = tokens.get(1) {
            line = without_comment.as_str();
        }        
    }

    line.split(|c| c == ' ' || c == ',')
                               .filter(|part| part.len() > 0)
                               .collect()
}


fn parse(tokens: Vec<&str>, instruction_set: &HashMap<&str, BitInt<5>>) -> Result<Vec<InstructionFormat>, String> {
    let mut instructions: Vec<InstructionFormat> = Vec::new();

    // let mut iter = tokens.iter().peekable();
    // ADD R6, #0b000

    let instructions = match tokens.len() {
        1 => {
            // instruction_set.get(k)
            let instruction = tokens.get(0).unwrap();
            let opcode: BitInt<5> = *instruction_set.get(instruction).unwrap();

            instructions.push(InstructionFormat::Format0op(_Format0opLayout {
                opcode: BitInt::<5>::new(*opcode).unwrap(),
                op_reserved: BitInt::<11>::new(0).unwrap()
            }))
        },
        2 => {
            let instruction = tokens.get(0).unwrap();
            let operand_1 = tokens.get(1).unwrap();

            let opcode: BitInt<5> = *instruction_set.get(instruction).unwrap();
            let op_type = match parse_operand_type(operand_1) {
                Ok(val) => val,
                Err(_) => {
                    alert(format!("Unknown instruction: {}", instruction).as_str());
                    panic!("");
                },
            };
            let op_value = ???;




        },
        3 => { },
        _ => { },
    };



    // while let Some(&token) = iter.next() {
    //     if let Some(&opcode) = instruction_set.get(token) {

    //     } else {
    //         return Err(format!("Unknown instruction: {}", token));
    //     }
    // };

    Ok(instructions)
}

// fn parse_operand_value(operand: &str) -> Result<BitInt???, String> {
//     if operand.starts_with("R") {
//         let reg_num: u8 = operand[1..].parse().map_err(|_| format!("Invalid register: {}", operand))?;
//         if reg_num > 7 {
//             return Err(format!("Invalid register: {}", operand));
//         }
//         BitInt::<11>::new(value)
//     } else if operand.starts_with("#") {
//         let imm_val: u8 = operand[1..].parse().map_err(|_| format!("Invalid immediate value: {}", operand))?;
//         Ok(int_to_bool_array(imm_val, 8))
//     } else if operand.starts_with("[") && operand.ends_with("]") {
//         let address = &operand[1..operand.len()-1];
//         if address.starts_with("R") {
//             let reg_num: u8 = address[1..].parse().map_err(|_| format!("Invalid register in address: {}", operand))?;
//             if reg_num > 7 {
//                 return Err(format!("Invalid register in address: {}", operand));
//             }
//             Ok(int_to_bool_array(reg_num, 8))
//         } else {
//             let direct_address: u8 = address.parse().map_err(|_| format!("Invalid memory address: {}", operand))?;
//             Ok(int_to_bool_array(direct_address, 8))
//         }
//     } else {
//         Err(format!("Unknown operand: {}", operand))
//     }
// }

// Could probably be better lol
fn parse_operand_type(operand: &str) -> Result<BitInt<3>, String> {
    if operand.starts_with("R") {
        Ok(BitInt::<3>::new(0b000).unwrap()) // Register
    } else
    if operand.starts_with("-(") && operand.ends_with(")") {
        Ok(BitInt::<3>::new(0b001).unwrap()) // Register pre decrement
    } else
    if operand.starts_with("+(") && operand.ends_with(")") {
        Ok(BitInt::<3>::new(0b011).unwrap()) // Register post increment
    } else
    
    if operand.starts_with("@") {
        Ok(BitInt::<3>::new(0b101).unwrap()) // Adress
    } else
    
    if operand.starts_with("(") && operand.ends_with(")") {
        Ok(BitInt::<3>::new(0b010).unwrap()) // Indirect register
    } else
    
    if operand.starts_with("#") {
        Ok(BitInt::<3>::new(100).unwrap()) // Immediate value
    } else {
        Err("Unknown operand type".to_string())
    }
}

// fn generate_machine_code(parsed_instructions: Vec<InstructionFormat>) -> Vec<u8> {
//     let mut machine_code = Vec::new();

//     for instruction in parsed_instructions {
//         match instruction {
//             InstructionFormat::Format0op(instr) => {
//                 // Convert instr into binary machine code and append to machine_code
                
//             }
//             InstructionFormat::Format1op(instr) => {
//                 // Convert instr into binary machine code and append to machine_code
//             }
//             InstructionFormat::Format2op(instr) => {
//                 // Convert instr into binary machine code and append to machine_code
//             }
//         }
//     }

//     machine_code
// }
