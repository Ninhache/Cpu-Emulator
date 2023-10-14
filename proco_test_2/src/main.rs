extern crate maplit;
extern crate regex;

mod instructions;
mod opcode;

use once_cell::sync::Lazy;
use regex::{Regex};
use std::fmt::{self};
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::error::Error;



struct Registers {
    /*
    Define the "register" from R0 to R7
    R0 - R5 | Designed for ?
    R6 (PC) | Booked for the Program Counter, Will store the next adress of the next instruction
    R7 (SP) | Booked for the Service Processor, provides remote management capabilities, including console redirection, logging and power control
    (Look the instruction's pdf )
    */
    r: [u8; 8], // U8 choosed because adress are 
    // current_register: u8; // Useless ??
    
    /*
    Define the status register, usefull for ..
    16 bits array
    sr[13] => Carry Flag, known as C, provides information to know if an operand has a carry
    sr[14] => Zero Flag, known as Z, will be set to 1 if operand result is 0
    sr[15] => Negative Flag, known as N, will be set to 1 if operand result is negative
    */
    sr: [bool; 16],
}

impl Registers {
    pub fn new() -> Self {
        Registers {
            r: [0; 8], // Initialize all registers to 0
            sr: [false; 16],
        }
    }
}


fn main() -> Result<(),  Box<dyn Error>> {
    // for (opcode, function) in opcode::OPCODES.iter() {
    //     println!("Opcode: {:?}, Function: {:?}", opcode, function);
    // }    
    
    let regex: Regex = Regex::new(r"[A-Z]+\s").unwrap();

    println!("Take an input asm..");
    
    let file_path: &str = "input.asm";
    let file: File = File::open(file_path)?;

    let reader: BufReader<File> = BufReader::new(file);

    let mut result = String::new();

    let mut nb_line = 0;

    for line in reader.lines() {
        let mut line: String = line?;
        print!("ANALYZE THIS LINE [{}]\n", line);

        let re = Regex::new(r"^(.*?)(?:;|$)").unwrap();
            
        if let Some(tokens) = re.captures(&line) {
            if let Some(without_comment) = tokens.get(1) {
                line = without_comment.as_str().to_string();
            }        
        }
        
        let parts: Vec<&str> = line
                                .split(|c| c == ' ' || c == ',')
                                .filter(|part| part.len() > 0)
                                .collect();

        let resultOpCode: String = match parts.len() {
            0 => continue,
            1 => parse_0op(parts),
            2 => parse_1op(parts),
            3 => parse_2op(parts),
            _ => "ckoa ce merdier".to_owned(),
        };

        nb_line += 1;
        result.push_str(&format!("{nb_line}\t{}\n", resultOpCode));
        
        println!("END ANALYZIS\n");
    }

    println!("Final opcode : \n{}", result);

    Ok(())
}


// struct _Format0opLayout {
//     opcode: [bool; 5],
//     op_reserved : [bool; 11],
// }
fn parse_0op(parts: Vec<&str>) -> String {
    let instruction: &str = parts[0];
    format!("{}{:011b}", get_instruction_code(instruction), 0)
}

impl OpType {
    fn get_type_value(&self) -> String {
        match self {
            OpType::Register() => format!("{:03b}", 0),
            OpType::PreDecrementedRegister() => format!("{:03b}", 1),
            OpType::IndirectAddress() => format!("{:03b}", 2),
            OpType::PostIncrementRegister() => format!("{:03b}", 3),
            OpType::ImmediateValueDEC() => format!("{:03b}", 4),
            OpType::ImmediateValueHEX() => format!("{:03b}", 4),
            OpType::ImmediateValueBIN() => format!("{:03b}", 4),
            OpType::Address() => format!("{:03b}", 5),
            OpType::Unknown() => "UNKNOWN".to_owned()
        }
    }

    fn get_operand_value(&self, part: &str) -> String {
        match self {
            OpType::Register() => panic!("z"),
            OpType::PreDecrementedRegister() => panic!("z"),
            OpType::IndirectAddress() => panic!("z"),
            OpType::PostIncrementRegister() => panic!("z"),
            OpType::ImmediateValueDEC() => panic!("z"),
            OpType::ImmediateValueHEX() => panic!("z"),
            OpType::ImmediateValueBIN() => panic!("z"),
            OpType::Address() => panic!("z"),
            OpType::Unknown() => panic!("z"),
        }
    }
}

impl fmt::Display for OpType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_type_value())
    }
}

// struct _Format1opLayout {
//     opcode: [bool; 5],
//     op_type: [bool; 3],
//     op_value: [bool; 8],
// }
// S or D
fn parse_1op(parts: Vec<&str>) -> String {
    let instruction: &str = parts[0];
    let operand: &str = parts[1];

    let opcode: String = get_instruction_code(instruction);
    let op_type: OpType = get_op_type(operand);
    let op_value: String = get_op_value(operand, op_type);
    
    println!("PARSE_1p => {} -> {}", op_type, op_value);
    format!("{}{}{:08b}", opcode, op_type, op_value.as_str().parse::<u8>().unwrap())
}

#[derive(Copy, Clone)]
enum OpType {
    Register(),               // Rn
    PreDecrementedRegister(), // -(Rn)
    PostIncrementRegister(),  // +(Rn)
    Address(),                // @v
    IndirectAddress(),        // (Rn)
    ImmediateValueDEC(),      // #v
    ImmediateValueHEX(),      // #0xv
    ImmediateValueBIN(),      // #bv
    Unknown(),                // ???
}

struct OneThing {
    name: &'static str,
    regex: Regex,
    op_type: OpType,
}

struct ManyThings {
    register: OneThing,
    valeur_x: OneThing,
    valeur_oxv: OneThing,
    valeur_obv: OneThing,
    valeur_texte: OneThing,
    adresse_d: OneThing,
    register_i: OneThing,
    register_i_post: OneThing,
    register_i_pre: OneThing,
}

impl ManyThings {
    fn get(&self, name: &str) -> Option<&OneThing> {
        Some(match name {
            "REGISTER" => &self.register,
            "VALEUR_#V" => &self.valeur_x,
            "VALEUR_OxV" => &self.valeur_oxv,
            "VALEUR_0bV" => &self.valeur_obv,
            "VALEUR_TEXTE" => &self.valeur_texte,
            "ADRESSE_D" => &self.adresse_d,
            "REGISTER_I" => &self.register_i,
            "REGISTER_I_POST" => &self.register_i_post,
            "REGISTER_I_PRE" => &self.register_i_pre,
            _ => return None,
        })
    }
}

impl std::ops::Index<&str> for ManyThings {
    type Output = OneThing;
    fn index(&self, key: &str) -> &Self::Output {
        self.get(key).expect("Regex Not Found")
    }
}



static RE_MAP: Lazy<ManyThings> = Lazy::new(|| ManyThings {
    register: OneThing {
        name: "REGISTER",
        regex: Regex::new(r"^R([0-7])$").unwrap(),
        op_type: OpType::Register(),
    },
    valeur_x: OneThing {
        name: "VALEUR_#V",
        regex: Regex::new(r"^#((6553[0-5])|(655[0-2][0-9])|(65[0-4][0-9]{2})|(6[0-4][0-9]{3})|([1-5][0-9]{4})|([0-5]{0,5})|([0-9]{1,4}))$").unwrap(),
        op_type: OpType::ImmediateValueDEC(),
    },
    valeur_oxv: OneThing {
        name: "VALEUR_OxV",
        regex: Regex::new(r"^#0x([0-9A-F]{1,4})$").unwrap(),
        op_type: OpType::ImmediateValueHEX(),
    },
    valeur_obv: OneThing {
        name: "VALEUR_0bV",
        regex: Regex::new(r"^#b([0|1]{1,16})$").unwrap(),
        op_type: OpType::ImmediateValueBIN(),
    },
    valeur_texte: OneThing {
        name: "VALEUR_TEXTE",
        regex: Regex::new(r"^#([a-zA-Z]+)$").unwrap(),
        op_type: OpType::Unknown(),
    },
    adresse_d: OneThing {
        name: "ADRESSE_D",
        regex: Regex::new(r"^@0x([0-9A-F]{1,4})$").unwrap(),
        op_type: OpType::Address(),
    },
    register_i: OneThing {
        name: "REGISTER_I",
        regex: Regex::new(r"^\(R([0-7])\)$").unwrap(),
        op_type: OpType::IndirectAddress(),
    },
    register_i_post: OneThing {
        name: "REGISTER_I_POST",
        regex: Regex::new(r"^\(R([0-7])\)\+$").unwrap(),
        op_type: OpType::PostIncrementRegister(),
    },
    register_i_pre: OneThing {
        name: "REGISTER_I_PRE",
        regex: Regex::new(r"^\-\(R([0-7])\)$").unwrap(),
        op_type: OpType::PreDecrementedRegister(),
    }
});



fn get_op_type(opcode: &str) -> OpType {
    // We're looking over the first char to not apply all the regex
    let mut chars: std::str::Chars<'_> = opcode.chars();

    match chars.nth(0) {
        Some('R') => OpType::Register(),
        Some('-') => OpType::PreDecrementedRegister(),
        Some('(') => {
            match chars.last() {
                Some(')') => OpType::IndirectAddress(),
                Some('+') => OpType::PostIncrementRegister(),
                None => OpType::Unknown(),
                _ => OpType::Unknown()
            }
        },
        Some('#') => {
            
            let patterns = [
                &RE_MAP["VALEUR_#V"],
                &RE_MAP["VALEUR_OxV"],
                &RE_MAP["VALEUR_0bV"],
                &RE_MAP["VALEUR_TEXTE"],
            ];
            
            
            for ot in patterns {
                let re = &ot.regex;
                if re.captures(&format!("{}", opcode)).is_some() {
                    return ot.op_type;
                }
            }

            OpType::Unknown()
        },
        Some('@') => OpType::Address(),
        None => OpType::Unknown(),
        _ => OpType::Unknown()
    }
}



fn get_op_value(opcode: &str, optype: OpType) -> String {
    let undefined_str = "0".to_string();
    match optype {
        OpType::Register() => {
            let re = &RE_MAP["REGISTER"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().to_owned();
                }        
            }

            undefined_str
        },
        OpType::IndirectAddress() => {
            let re = &RE_MAP["REGISTER_I"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().to_owned();
                }        
            }

            undefined_str
        },
        OpType::PreDecrementedRegister() => {
            print!("GHJKLGHJKLGHJKLMGHJKLMHJKL [{}]", opcode);
            let re = &RE_MAP["REGISTER_I_PRE"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().to_owned();
                }        
            }

            undefined_str
        },
        OpType::PostIncrementRegister() => {
            let re = &RE_MAP["REGISTER_I_POST"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().to_owned();
                }        
            }

            undefined_str
        },
        OpType::Address() => {
            let re = &RE_MAP["ADRESSE_D"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(hex_string) = captures.get(1) {
                    match i64::from_str_radix(hex_string.as_str(), 16) {
                        Ok(decimal_value) => return decimal_value.to_string(),
                        Err(e) => return undefined_str,
                    }
                }    
            }

            undefined_str
        },
        OpType::ImmediateValueDEC() => {
            let re = &RE_MAP["VALEUR_#V"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().to_owned();
                }        
            }

            undefined_str
        },
        OpType::ImmediateValueHEX() => {
            let re = &RE_MAP["VALEUR_0xV"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().to_owned();
                }        
            }

            undefined_str
        },
        OpType::ImmediateValueBIN() => {
            let re = &RE_MAP["VALEUR_0bV"].regex;
            
            if let Some(captures) = re.captures(opcode) {
                if let Some(token_value) = captures.get(1) {
                    return token_value.as_str().to_owned();
                }        
            }

            undefined_str
        },
        
        
        _ => undefined_str,
    }
}


// struct _Format2opLayout {
//     opcode: [bool; 5],
//     op_type_dest: [bool; 3], // REGISTER (when it's not move)
//     op_type_source: [bool; 3], // TYPE source (Can't be an OpType::Address())
//     op_value: [bool; 5],
// }
// ADD S D
fn parse_2op(parts: Vec<&str>) -> String {
    
    // specific instruction
    let instruction: &str = parts[0];

    // if move .. do something else..

    let source: &str = parts[1];
    let destination: &str = parts[2];
    
    let op_type_dest = get_op_type(destination);
    let op_type_source = get_op_type(source);


    let opcode = get_instruction_code(instruction);
    let op_value = get_op_value(source, op_type_source);

    format!("{}{}{}{:05b}", opcode, op_type_dest, op_type_source, op_value.as_str().parse::<u8>().unwrap())
}

fn parse_move(parts: Vec<&str>) -> String {
    "null".to_owned()
}


fn get_instruction_code(part: &str) -> String {
    format!("{:05b}", opcode::opcode::OPCODES.get(part).unwrap())
}
