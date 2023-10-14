extern crate termcolor;
mod utils;

extern crate regex;
use regex::{Regex};
use utils::info;

use std::collections::HashMap;
use std::fmt::{self};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;
use std::{result, any};

use crate::utils::{log, err, shape};

struct CPUState {
    registers: [u16; 8],
    memory: Vec<u8>,
    labels: HashMap<String, u8>,
    program_counter: u16,
    sr: [bool; 16],
}

#[derive(Debug)]
enum Operand {
    Register(u8),
    PreDecrementedRegister(u8),
    PostIncrementedRegister(u8),
    MemoryAddress(u8),
    IndirectAddress(u8),
    ImmediateValueDEC(u16),
    ImmediateValueHEX(u16),
    ImmediateValueBIN(u16),
    Label(String)
}

fn ordinalToRegister(cardinal: u8) -> &'static str {
    match cardinal {
        0 => "Register",
        1 => "PreDecrementedRegister",
        2 => "PostIncrementedRegister",
        3 => "MemoryAddress",
        4 => "IndirectAddress",
        5 => "ImmediateValueDEC",
        6 => "ImmediateValueHEX",
        7 => "ImmediateValueBIN",
        8 => "Label",
        _ => "Unknown"
    }
}

impl Operand {
    pub fn cardinal(&self) -> u8 {
        match self {
            Operand::Register(_) => 0,
            Operand::PreDecrementedRegister(_) => 1,
            Operand::PostIncrementedRegister(_) => 2,
            Operand::MemoryAddress(_) => 3,
            Operand::IndirectAddress(_) => 4,
            Operand::ImmediateValueDEC(_) => 5,
            Operand::ImmediateValueHEX(_) => 6,
            Operand::ImmediateValueBIN(_) => 7,
            Operand::Label(_) => 8,
        }
    }
}


// Register(),               // Rn
// IndirectAddress(),        // (Rn)
// PreDecrementedRegister(), // -(Rn)
// PostIncrementRegister(),  // +(Rn)

// Address(),                // @v
// ImmediateValueDEC(),      // #v
// ImmediateValueHEX(),      // #0xv
// ImmediateValueBIN(),      // #bv


impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Operand")
    }
}


#[derive(Debug)]
enum Instruction {
    MOVE(Operand, Operand),
    PUSH(Operand),
    POP(Operand),
    ADD(Operand, Operand),
    CMP(Operand, Operand),
    SUB(Operand, Operand),
    LSL(Operand, Operand),
    LSR(Operand, Operand),
    AND(Operand, Operand),
    OR(Operand, Operand),
    XOR(Operand, Operand),
    NOT(Operand),
    BCC(Operand),
    BGT(Operand),
    BCS(Operand),
    BLT(Operand),
    BEQ(Operand),
    BNE(Operand),
    BLE(Operand),
    BGE(Operand),
    BRA(Operand),
    JCC(Operand),
    JCS(Operand),
    JEQ(Operand),
    JNE(Operand),
    JLT(Operand),
    JGT(Operand),
    JGE(Operand),
    JMP(Operand),
    BSR(Operand),
    JSR(Operand),
    RTS(Operand),
    TRAP(Operand),
    RTE(Operand)
}

// Implement Display for Instruction
impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::MOVE(source, destination) => write!(f, "MOVE({},{})", source, destination),
            Instruction::PUSH(source) => write!(f, "PUSH({})", source),
            Instruction::POP(source) => write!(f, "POP({})", source),
            Instruction::ADD(source, destination) => write!(f, "ADD({},{})", source, destination),
            Instruction::CMP(source, destination) => write!(f, "CMP({},{})", source, destination),
            Instruction::SUB(source, destination) => write!(f, "SUB({},{})", source, destination),
            Instruction::LSL(source, destination) => write!(f, "LSL({},{})", source, destination),
            Instruction::LSR(source, destination) => write!(f, "LSR({},{})", source, destination),
            Instruction::AND(source, destination) => write!(f, "AND({},{})", source, destination),
            Instruction::OR(source,  destination) => write!(f, "OR({}),{}", source, destination),
            Instruction::XOR(source, destination) => write!(f, "XOR({},{})", source, destination),
            Instruction::NOT(source) => write!(f, "NOT({})", source),
            Instruction::BCC(source) => write!(f, "BCC({})", source),
            Instruction::BGT(source) => write!(f, "BGT({})", source),
            Instruction::BCS(source) => write!(f, "BCS({})", source),
            Instruction::BLT(source) => write!(f, "BLT({})", source),
            Instruction::BEQ(source) => write!(f, "BEQ({})", source),
            Instruction::BNE(source) => write!(f, "BNE({})", source),
            Instruction::BLE(source) => write!(f, "BLE({})", source),
            Instruction::BGE(source) => write!(f, "BGE({})", source),
            Instruction::BRA(source) => write!(f, "BRA({})", source),
            Instruction::JCC(source) => write!(f, "JCC({})", source),
            Instruction::JCS(source) => write!(f, "JCS({})", source),
            Instruction::JEQ(source) => write!(f, "JEQ({})", source),
            Instruction::JNE(source) => write!(f, "JNE({})", source),
            Instruction::JLT(source) => write!(f, "JLT({})", source),
            Instruction::JGT(source) => write!(f, "JGT({})", source),
            Instruction::JGE(source) => write!(f, "JGE({})", source),
            Instruction::JMP(source) => write!(f, "JMP({})", source),
            Instruction::BSR(source) => write!(f, "BSR({})", source),
            Instruction::JSR(source) => write!(f, "JSR({})", source),
            Instruction::RTS(source) => write!(f, "RTS({})", source),
            Instruction::TRAP(source) => write!(f, "TRAP({})", source),
            Instruction::RTE(source) => write!(f, "RTE({})", source)
        }
    }
}

fn parse_operand(operand_str: &str) -> Result<Operand, String> {
    // println!(">> operand_str : {operand_str}");
    info(format!(">> operand_str  {}", operand_str).as_str());

    if operand_str.starts_with("R") {
        operand_str[1..].parse::<u8>()
            .map(Operand::Register)
            .map_err(|_| "Invalid register".to_string())
    } else

    if operand_str.starts_with("-(") && operand_str.ends_with(")") {
        operand_str[3..operand_str.len()-1].parse::<u8>()
            .map(Operand::PreDecrementedRegister)
            .map_err(|_| "Invalid pre-decremented register".to_string())
    } else

    if operand_str.starts_with("+(") && operand_str.ends_with(")") {
        operand_str[3..operand_str.len()-1].parse::<u8>()
            .map(Operand::PostIncrementedRegister)
            .map_err(|_| "Invalid post-incremented register".to_string())
    } else
    
    if operand_str.starts_with("@0x") {
        u8::from_str_radix(&operand_str[3..], 8)
            .map(Operand::MemoryAddress)
            .map_err(|_| "Invalid address".to_string())
    } else
    
    if operand_str.starts_with("(") && operand_str.ends_with(")") {
        operand_str[2..operand_str.len()-1].parse::<u8>()
            .map(Operand::IndirectAddress)
            .map_err(|_| "Invalid indirect address".to_string())
    } else
    
    if operand_str.starts_with("#") {
        let tmp: &str = &operand_str[1..];

        if tmp.starts_with("0x") {
            u16::from_str_radix(&operand_str[2..], 16)
                .map(Operand::ImmediateValueHEX)
                .map_err(|_| "Invalid hex value".to_string())
        } else if tmp.starts_with("0b") {
            u16::from_str_radix(&operand_str[2..], 2)
                .map(Operand::ImmediateValueBIN)
                .map_err(|_| "Invalid binary value".to_string())
        } else if is_decimal(tmp) {
            tmp.parse::<u16>()
                .map(Operand::ImmediateValueDEC)
                .map_err(|_| "Invalid decimal value".to_string())
        } else {
            // todo: register label in map or  something
            Ok(Operand::Label(tmp.to_string()))
        }
    } else {
        Err("Unknown operand type".to_string())
    }
}

fn is_decimal(input: &str) -> bool {
    match input.parse::<i16>() {
        Ok(_) => true,
        Err(_) => false
    }
}


fn parse_instruction(line: &str) -> Result<Instruction, String> {
    let parts: Vec<&str> = line.split(|c| c == ' ' || c == ',')
                               .filter(|part| part.len() > 0)
                               .collect();
    // println!("PARTS: {}", parts.join(", "));

    let opcode = parts[0];
    // println!("OPCODE {}", opcode);


    let operands: Vec<&str> = parts[1..].to_vec();
    // println!("OPERANDS: {}", operands.join(", "));

    log(format!("PARSING  {}", line).as_str());

    match opcode {
        // "MOVE"=> Instruction::MOVE(parse_operand(operands[0]), parse_operand(operands[1])),
        "MOVE" => Ok(Instruction::MOVE(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "PUSH"=> Ok(Instruction::PUSH(
            parse_operand(operands[0])?,
        )),
        "POP" => Ok(Instruction::POP(
            parse_operand(operands[0])?,
        )),
        "ADD" => Ok(Instruction::ADD(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "CMP" => Ok(Instruction::CMP(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "SUB" => Ok(Instruction::SUB(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "LSL" => Ok(Instruction::LSL(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "LSR" => Ok(Instruction::LSR(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "AND" => Ok(Instruction::AND(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "OR"  => Ok(Instruction::OR(
             parse_operand(operands[0])?,
             parse_operand(operands[1])?,
            )),
        "XOR" => Ok(Instruction::XOR(
            parse_operand(operands[0])?,
            parse_operand(operands[1])?,
        )),
        "NOT" => Ok(Instruction::NOT(
            parse_operand(operands[0])?,
        )),
        "BCC" => Ok(Instruction::BCC(
            parse_operand(operands[0])?,
        )),
        "BGT" => Ok(Instruction::BGT(
            parse_operand(operands[0])?,
        )),
        "BCS" => Ok(Instruction::BCS(
            parse_operand(operands[0])?,
        )),
        "BLT" => Ok(Instruction::BLT(
            parse_operand(operands[0])?,
        )),
        "BEQ" => Ok(Instruction::BEQ(
            parse_operand(operands[0])?,
        )),
        "BNE" => Ok(Instruction::BNE(
            parse_operand(operands[0])?,
        )),
        "BLE" => Ok(Instruction::BLE(
            parse_operand(operands[0])?,
        )),
        "BGE" => Ok(Instruction::BGE(
            parse_operand(operands[0])?,
        )),
        "BRA" => Ok(Instruction::BRA(
            parse_operand(operands[0])?,
        )),
        "JCC" => Ok(Instruction::JCC(
            parse_operand(operands[0])?,
        )),
        "JCS" => Ok(Instruction::JCS(
            parse_operand(operands[0])?,
        )),
        "JEQ" => Ok(Instruction::JEQ(
            parse_operand(operands[0])?,
        )),
        "JNE" => Ok(Instruction::JNE(
            parse_operand(operands[0])?,
        )),
        "JLT" => Ok(Instruction::JLT(
            parse_operand(operands[0])?,
        )),
        "JGT" => Ok(Instruction::JGT(
            parse_operand(operands[0])?,
        )),
        "JGE" => Ok(Instruction::JGE(
            parse_operand(operands[0])?,
        )),
        "JMP" => Ok(Instruction::JMP(
            parse_operand(operands[0])?,
        )),
        "BSR" => Ok(Instruction::BSR(
            parse_operand(operands[0])?,
        )),
        "JSR" => Ok(Instruction::JSR(
            parse_operand(operands[0])?,
        )),
        "RTS" => Ok(Instruction::RTS(
            parse_operand(operands[0])?,
        )),
        "TRAP"=> Ok(Instruction::TRAP(
            parse_operand(operands[0])?,
        )),
        "RTE" => Ok(Instruction::RTE(
            parse_operand(operands[0])?,
        )),
        _ => panic!("Unknown opcodzaeazeae : [{}]", opcode),
    }
}

// todo: add flags
fn get_operand_value(operand: &Operand, cpu_state: &mut CPUState) -> u16 {
    match operand {
        Operand::Register(id) => cpu_state.registers[*id as usize],
        Operand::MemoryAddress(addr) => {
            let addr = *addr as usize;
            ((cpu_state.memory[addr] as u16) << 8) | (cpu_state.memory[addr + 1] as u16)
        },
        Operand::IndirectAddress(addr) => {
            let indirect_addr: u8 = get_operand_value(&Operand::MemoryAddress(*addr), cpu_state).try_into().unwrap();
            get_operand_value(&Operand::MemoryAddress(indirect_addr), cpu_state)
        },

        Operand::ImmediateValueDEC(val) => *val,
        Operand::ImmediateValueBIN(val) => *val,
        Operand::ImmediateValueHEX(val) => *val,
        Operand::Label(val) => {
            match cpu_state.labels.get(val) {
                Some(value) => get_operand_value(&Operand::MemoryAddress(*value), cpu_state),
                None => {
                    err(format!("Failed {:?}", val).as_str());
                    return 0 as u16;
                }
            }
        },
        Operand::PostIncrementedRegister(id) => {
            let value = cpu_state.registers[*id as usize];
            cpu_state.registers[*id as usize] = cpu_state.registers[*id as usize].wrapping_add(1);
            value
        },
        Operand::PreDecrementedRegister(id) => {
            cpu_state.registers[*id as usize] = cpu_state.registers[*id as usize].wrapping_sub(1);
            cpu_state.registers[*id as usize]
        },
    }
}

// fn get_operand_value(operand: &Operand, cpu_state: &mut CPUState) -> Result<(u16, Operand), String> {
//     match operand {
//         Operand::Register(id) => {
//             Ok((cpu_state.registers[*id as usize], Operand::Register(*id)))
//         },
//         Operand::MemoryAddress(addr) => {
//             let addr_2 = *addr as usize;
//             Ok((((cpu_state.memory[addr_2] as u16) << 8) | (cpu_state.memory[addr_2 + 1] as u16), Operand::MemoryAddress(*addr)))
//         },
//         Operand::IndirectAddress(addr) => {
//             match get_operand_value(&Operand::MemoryAddress(*addr), &mut cpu_state) {
//                 Ok((value, operand_type)) => {
//                     let indirect_addr: u8 = value.try_into().unwrap();
//                     // Ok((get_operand_value(&Operand::MemoryAddress(indirect_addr), cpu_state), Operand::IndirectAddress(*addr)))
//                     match get_operand_value(&Operand::MemoryAddress(indirect_addr), cpu_state) {
//                         Ok((value, _)) => {
//                             Ok((value, Operand::IndirectAddress(*addr)))
//                         },
//                         Err(emsg) => {
//                             err(format!("get_operand_value {}", emsg).as_str());
//                             panic!("");
//                         }
//                     }
//                 },
//                 Err(emsg) => {
//                     err(format!("get_operand_value {}", emsg).as_str());
//                     panic!("");
//                 }
//             }
//         },
//         Operand::ImmediateValueDEC(val) => Ok((*val, Operand::ImmediateValueDEC(*val))),
//         Operand::ImmediateValueBIN(val) => Ok((*val, Operand::ImmediateValueBIN(*val))),
//         Operand::ImmediateValueHEX(val) => Ok((*val, Operand::ImmediateValueHEX(*val))),
//         Operand::Label(val) => {
//             match cpu_state.labels.get(val) {
//                 Some(value) => {
//                     match get_operand_value(&Operand::MemoryAddress(*value), cpu_state) {
//                         Ok((value, operand_type)) => {
//                             Ok((value, operand_type))
//                         },
//                         Err(_) => {
//                             panic!("zaea")
//                         }
//                     }
//                 },
//                 None => {
//                     err(format!("Failed {:?}", val).as_str());
//                     panic!("ezaaez!!");
//                 }
//             }
//         },
//         Operand::PostIncrementedRegister(id) => {
//             let value = cpu_state.registers[*id as usize];
//             cpu_state.registers[*id as usize] = cpu_state.registers[*id as usize].wrapping_add(1);
//             value
//         },
//         Operand::PreDecrementedRegister(id) => {
//             cpu_state.registers[*id as usize] = cpu_state.registers[*id as usize].wrapping_sub(1);
//             cpu_state.registers[*id as usize]
//         },
//     }
// }

fn set_operand_value(operand: &Operand, cpu_state: &mut CPUState, value: u16) {
    match operand {
        Operand::Register(id) => cpu_state.registers[*id as usize] = value,
        Operand::MemoryAddress(addr) => {
            let addr = *addr as usize;
            cpu_state.memory[addr] = (value >> 8) as u8;
            cpu_state.memory[addr + 1] = (value & 0xFF) as u8;
        },
        Operand::ImmediateValueDEC(_) => panic!("Cannot write to an immediate value"),
        Operand::ImmediateValueBIN(_) => panic!("Cannot write to an immediate value"),
        Operand::ImmediateValueHEX(_) => panic!("Cannot write to an immediate value"),
        Operand::Label(_) => panic!("Cannot write to a Label"),
        Operand::IndirectAddress(addr) => {
            let indirect_addr: u8 = get_operand_value(&Operand::MemoryAddress(*addr), cpu_state).try_into().unwrap();
            set_operand_value(&Operand::MemoryAddress(indirect_addr), cpu_state, value);
        },
        Operand::PostIncrementedRegister(id) => {
            let addr = cpu_state.registers[*id as usize] as usize;
            cpu_state.memory[addr] = (value >> 8) as u8;
            cpu_state.memory[addr + 1] = (value & 0xFF) as u8;
            cpu_state.registers[*id as usize] = cpu_state.registers[*id as usize].wrapping_add(1);
        },
        Operand::PreDecrementedRegister(id) => {
            let addr = cpu_state.registers[*id as usize] as usize;
            cpu_state.memory[addr] = (value >> 8) as u8;
            cpu_state.memory[addr + 1] = (value & 0xFF) as u8;
        },
    }
}

fn _execute_instruction(instruction: Instruction, cpu_state: &mut CPUState) {
    match instruction {
        Instruction::MOVE(source, destination) => { },
        Instruction::PUSH(source) => { },
        Instruction::POP(source) => { },
        Instruction::ADD(source, destination) => {
            let source_type: u8 = source.cardinal();
            match source_type {
                0 | 1 | 2 | 4 | 5 | 6 |7 => { }
                anything => {
                    err(format!("{} is not allow as SOURCE in ADD", ordinalToRegister(anything)).as_str());
                    panic!("")
                }
            };

            let destination_type: u8 = destination.cardinal();
            match destination_type {
                0 => { }
                anything => {
                    err(format!("{} is not allow as DESTINATION in ADD", ordinalToRegister(anything)).as_str());
                    panic!("")
                }
            };

            let val1: u16 = get_operand_value(&source, cpu_state);
            let val2: u16 = get_operand_value(&destination, cpu_state);
            let result: u16 = val1.wrapping_add(val2);
            set_operand_value(&destination, cpu_state, result);
        },
        Instruction::CMP(source, destination) => { },
        Instruction::SUB(source, destination) => {
            let val1 = get_operand_value(&source, cpu_state);
            let val2 = get_operand_value(&destination, cpu_state);
            // val1.wrapping_sub(val2)
        },
        Instruction::LSL(source, destination) => { },
        Instruction::LSR(source, destination) => { },
        Instruction::AND(source, destination) => {
            let val1 = get_operand_value(&source, cpu_state);
            let val2 = get_operand_value(&destination, cpu_state);
            // (val1 & val2)
        },
        Instruction::OR(source, destination) => {
            let val1 = get_operand_value(&source, cpu_state);
            let val2 = get_operand_value(&destination, cpu_state);
            // (val1 | val2)
        },
        Instruction::XOR(source, destination) => {
            let val1 = get_operand_value(&source, cpu_state);
            let val2 = get_operand_value(&destination, cpu_state);
            // (val1 ^ val2)
        },
        Instruction::NOT(source) => {
            let val1 = get_operand_value(&source, cpu_state);
            // (!val1)
        },
        Instruction::BCC(source) => { },
        Instruction::BGT(source) => { },
        Instruction::BCS(source) => { },
        Instruction::BLT(source) => { },
        Instruction::BEQ(source) => { },
        Instruction::BNE(source) => { },
        Instruction::BLE(source) => { },
        Instruction::BGE(source) => { },
        Instruction::BRA(source) => { },
        Instruction::JCC(source) => { },
        Instruction::JCS(source) => { },
        Instruction::JEQ(source) => { },
        Instruction::JNE(source) => { },
        Instruction::JLT(source) => { },
        Instruction::JGT(source) => { },
        Instruction::JGE(source) => { },
        Instruction::JMP(source) => { },
        Instruction::BSR(source) => { },
        Instruction::JSR(source) => { },
        Instruction::RTS(source) => { },
        Instruction::TRAP(source) => { },
        Instruction::RTE(source) => { },
    }
}

fn main() -> Result<(),  Box<dyn Error>> {
    println!("Take an input asm..");
    let file_path: &str = "input.asm";
    let file: File = File::open(file_path)?;

    let reader: BufReader<File> = BufReader::new(file);

    // let mut nb_line = 0;

    for line in reader.lines() {
        let mut line: String = line?;

        // Regex for remove comments
        let re = Regex::new(r"^(.*?)(?:;|$)").unwrap();
            
        if let Some(tokens) = re.captures(&line) {
            if let Some(without_comment) = tokens.get(1) {
                line = without_comment.as_str().to_string();
            }        
        }

        match parse_instruction(&line) {
            Ok(instruction) => {
                log(format!("Successfull {:?}", instruction).as_str());
            },
            Err(error) => {
                err(format!("Failed {:?}", error).as_str());
            },
        }
        shape(" ------ ")
    }

    Ok(())
}

