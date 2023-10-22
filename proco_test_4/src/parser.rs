use crate::utils::{BitInt, alert, log};

use regex::{self, Regex};


pub enum InstructionFormat {
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
    registry_dest: BitInt::<3>,
    op_type_source: BitInt::<3>,
    op_value: BitInt::<5>,
}

impl fmt::Display for _Format0opLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode: {}, Reserved: {}", self.opcode, self.op_reserved)
    }
}

impl fmt::Display for _Format1opLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode: {}, Type: {}, Value: {}", self.opcode, self.op_type, self.op_value)
    }
}

impl fmt::Display for _Format2opLayout {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Opcode: {}, Dest: {}, Type Source: {}, Value: {}", self.opcode, self.registry_dest, self.op_type_source, self.op_value)
    }
}

impl fmt::Display for InstructionFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InstructionFormat::Format0op(layout) => write!(f, "Format0op: {}", layout),
            InstructionFormat::Format1op(layout) => write!(f, "Format1op: {}", layout),
            InstructionFormat::Format2op(layout) => write!(f, "Format2op: {}", layout),
        }
    }
}

use core::fmt;
use std::{collections::HashMap, ptr::null};
use lazy_static::lazy_static;
lazy_static! {
pub static ref RE_MAP: HashMap<&'static str, Regex> = [
    ("REGISTER", Regex::new(r"^R([0-7])$").unwrap()),
    ("VALEUR_#V", Regex::new(r"^#((6553[0-5])|(655[0-2][0-9])|(65[0-4][0-9]{2})|(6[0-4][0-9]{3})|([1-5][0-9]{4})|([0-5]{0,5})|([0-9]{1,4}))$").unwrap()),
    ("VALEUR_OxV", Regex::new(r"^#0x([0-9A-F]{1,4})$").unwrap()),
    ("VALEUR_0bV", Regex::new(r"^#b([0|1]{1,16})$").unwrap()),
    ("VALEUR_TEXTE", Regex::new(r"^#([a-zA-Z]+)$").unwrap()),
    ("ADRESS_D", Regex::new(r"^@0x([0-9A-F]{1,4})$").unwrap()),
    ("REGISTER_I", Regex::new(r"^\(R([0-7])\)$").unwrap()),
    ("REGISTER_I_POST", Regex::new(r"^\(R([0-7])\)\+$").unwrap()),
    ("REGISTER_I_PRE", Regex::new(r"^\-\(R([0-7])\)$").unwrap())
].iter().cloned().collect();
}




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


pub fn parse(tokens: Vec<&str>, instruction_set: &HashMap<&str, BitInt<5>>) -> Result<Vec<InstructionFormat>, String> {
    let mut instructions: Vec<InstructionFormat> = Vec::new();

    // let mut iter = tokens.iter().peekable();

    match tokens.len() {
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

            let op_type: Operand = parse_operand_type(operand_1);
            let op_value  = get_op_value(operand_1, op_type);
            
            instructions.push(InstructionFormat::Format1op(_Format1opLayout {
                opcode: BitInt::<5>::new(*opcode).unwrap(),
                op_type: BitInt::<3>::new(op_type.get().into()).unwrap(),
                op_value: BitInt::<8>::new(op_value.into()).unwrap()
            }))

        },
        3 => {
            let instruction = tokens.get(0).unwrap();
            if instruction.to_lowercase() == "MOVE" {
                // todo: parse move instruction
                instructions.push(InstructionFormat::Format2op(_Format2opLayout {
                    opcode: BitInt::<5>::new(0b00000).unwrap(),
                    registry_dest: BitInt::<3>::new(0).unwrap(),
                    op_type_source: BitInt::<3>::new(0).unwrap(),
                    op_value: BitInt::<5>::new(0).unwrap()
                }))
            } else {
                let source = tokens.get(1).unwrap();
                let destination = tokens.get(2).unwrap();
                
                let opcode: BitInt<5> = *instruction_set.get(instruction).unwrap();
                

                let source_type: Operand = parse_operand_type(source);
                let source_value  = get_op_value(source, source_type);

                let destination_type: Operand = parse_operand_type(destination);

                match destination_type {
                    Operand::Register(_) => {
                        let destination_value  = get_op_value(destination, destination_type);

                        instructions.push(InstructionFormat::Format2op(_Format2opLayout {
                            opcode: opcode,
                            registry_dest: BitInt::<3>::new(destination_value.into()).unwrap(),
                            op_type_source: BitInt::<3>::new(source_type.get().into()).unwrap(),
                            op_value: BitInt::<5>::new(source_value.into()).unwrap()
                        }))
                    },
                    _ => {
                        alert(format!("Destination operand isnt a register for\n{} {}, {}", instruction, source, destination).as_str());
                        panic!("")
                    }
                }
            }
        },
        _ => { },
    };

    Ok(instructions)    
}


fn get_op_value(source: &str, operand: Operand) -> u8 {
    match operand {
        Operand::Register(_) => {
            // let re = &RE_MAP["REGISTER"].regex;
            let re: &Regex = RE_MAP.get("REGISTER").unwrap();
            
            if let Some(captures) = re.captures(source) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().parse::<u8>().unwrap();
                }        
            }

            0
        },
        Operand::IndirectAddress(_) => {
            let re: &Regex = RE_MAP.get("REGISTER_I").unwrap();
            
            if let Some(captures) = re.captures(source) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().parse::<u8>().unwrap();
                }        
            }

            0
        },
        Operand::PreDecrementedRegister(_) => {
            let re = RE_MAP.get("REGISTER_I_PRE").unwrap();
            
            if let Some(captures) = re.captures(source) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().parse::<u8>().unwrap();
                }        
            }

            0
        },
        Operand::PostIncrementedRegister(_) => {
            let re = RE_MAP.get("REGISTER_I_POST").unwrap();
            
            if let Some(captures) = re.captures(source) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().parse::<u8>().unwrap();
                }        
            }

            0
        },
        Operand::MemoryAddress(_) => {
            let re = RE_MAP.get("ADRESS_D").unwrap();

            if let Some(captures) = re.captures(source) {
                if let Some(hex_string) = captures.get(1) {
                    return u8::from_str_radix(hex_string.as_str(), 16).unwrap()
                }    
            }

            0
        },
        Operand::ImmediateValueDEC(_) => {
            let re = RE_MAP.get("VALEUR_#V").unwrap();
            
            if let Some(captures) = re.captures(source) {
                if let Some(token_value) = captures.get(1) {
                  return u8::from_str_radix(token_value.as_str(), 16).unwrap()
                }        
            }
            0
        },
        Operand::ImmediateValueHEX(_) => {
            let re = RE_MAP.get("VALEUR_0xV").unwrap();
            
            if let Some(captures) = re.captures(source) {
                if let Some(token_value) = captures.get(1) {
                    return u8::from_str_radix(token_value.as_str(), 16).unwrap()
                }        
            }

            0
        },
        Operand::ImmediateValueBIN(_) => {
            let re = RE_MAP.get("VALEUR_0bV").unwrap();
            
            if let Some(captures) = re.captures(source) {
                if let Some(token_value) = captures.get(1) {
                    match u8::from_str_radix(token_value.as_str(), 16) {
                        Ok(decimal_value) => return decimal_value,
                        Err(_) => { },
                    }
                }        
            }
            0
        },
        
        
        _ => 0,
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
enum Operand {
    Register(u8),
    PreDecrementedRegister(u8),
    PostIncrementedRegister(u8),
    MemoryAddress(u8),
    IndirectAddress(u8),
    ImmediateValueDEC(u16),
    ImmediateValueHEX(u16),
    ImmediateValueBIN(u16),
    Label(u16)
}

impl Operand {
    pub fn get(&self) -> u8 {
        match self {
            Operand::Register(val) => *val,
            Operand::PreDecrementedRegister(val) => *val,
            Operand::PostIncrementedRegister(val) => *val,
            Operand::MemoryAddress(val) => *val,
            Operand::IndirectAddress(val) => *val,
            Operand::ImmediateValueDEC(val) => (*val).try_into().unwrap(),
            Operand::ImmediateValueHEX(val) => (*val).try_into().unwrap(),
            Operand::ImmediateValueBIN(val) => (*val).try_into().unwrap(),
            Operand::Label(val) => (*val).try_into().unwrap(),
        }
    }

}


// Could probably be better lol
fn parse_operand_type(operand: &str) -> Operand {
    if operand.starts_with("R") {
        Operand::Register(0b000)
        // BitInt::<3>::new(0b000).unwrap() // Register
    } else
    if operand.starts_with("-(") && operand.ends_with(")") {
        Operand::PreDecrementedRegister(0b001)
        // BitInt::<3>::new(0b001).unwrap() // Register pre decrement
    } else
    if operand.starts_with("+(") && operand.ends_with(")") {
        Operand::PostIncrementedRegister(0b011)
        // BitInt::<3>::new(0b011).unwrap() // Register post increment
    } else
    if operand.starts_with("@") {
        Operand::MemoryAddress(0b101)
        // BitInt::<3>::new(0b101).unwrap() // Adress
    } else
    
    if operand.starts_with("(") && operand.ends_with(")") {
        Operand::IndirectAddress(0b010)
        // BitInt::<3>::new(0b010).unwrap() // Indirect register
    } else
        
    if operand.starts_with("#") {
        let patterns = [
            &RE_MAP["VALEUR_#V"],
            &RE_MAP["VALEUR_OxV"],
            &RE_MAP["VALEUR_0bV"],
            &RE_MAP["VALEUR_TEXTE"],
        ];
            
        let mut i = 0;
        let mut _result: *const u8 = null();
        for re in patterns {
            if let Some(captures) = re.captures(&format!("{}", operand)) {
                if let Some(token_value) = captures.get(1) {
                    println!("token_value {}", token_value.as_str());
                }
            }
            i = i + 1;
        };

        Operand::ImmediateValueBIN(0b000)
    } else {
        alert("Unknown operand type");
        panic!("")
    }
}