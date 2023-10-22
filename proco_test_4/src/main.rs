extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufReader, BufRead};

mod parser;
mod utils;

use utils::BitInt;


use std::collections::HashMap;
use lazy_static::lazy_static;
lazy_static! {
pub static ref INSTRUCTION_SET: HashMap<&'static str, BitInt::<5>> = [
    ("MOVE", BitInt::<5>::new(0x00).unwrap()),
    ("PUSH", BitInt::<5>::new(0x01).unwrap()),
    ("POP" , BitInt::<5>::new(0x02).unwrap()),
    ("ADD" , BitInt::<5>::new(0x03).unwrap()),
    ("CMP" , BitInt::<5>::new(0x04).unwrap()),
    ("SUB" , BitInt::<5>::new(0x05).unwrap()),
    ("LSL" , BitInt::<5>::new(0x06).unwrap()),
    ("LSR" , BitInt::<5>::new(0x07).unwrap()),
    ("AND" , BitInt::<5>::new(0x08).unwrap()),
    ("OR"  , BitInt::<5>::new(0x09).unwrap()),
    ("XOR" , BitInt::<5>::new(0x0A).unwrap()),
    ("NOT" , BitInt::<5>::new(0x0B).unwrap()),
    ("BCC" , BitInt::<5>::new(0x0C).unwrap()),
    ("BGT" , BitInt::<5>::new(0x0C).unwrap()),
    ("BCS" , BitInt::<5>::new(0x0D).unwrap()),
    ("BLT" , BitInt::<5>::new(0x0D).unwrap()),
    ("BEQ" , BitInt::<5>::new(0x0E).unwrap()),
    ("BNE" , BitInt::<5>::new(0x0F).unwrap()),
    ("BLE" , BitInt::<5>::new(0x10).unwrap()),
    ("BGE" , BitInt::<5>::new(0x11).unwrap()),
    ("BRA" , BitInt::<5>::new(0x12).unwrap()),
    ("BSR" , BitInt::<5>::new(0x13).unwrap()),
    ("JCC" , BitInt::<5>::new(0x14).unwrap()),
    ("JGT" , BitInt::<5>::new(0x14).unwrap()),
    ("JCS" , BitInt::<5>::new(0x15).unwrap()),
    ("JLT" , BitInt::<5>::new(0x15).unwrap()),
    ("JEQ" , BitInt::<5>::new(0x16).unwrap()),
    ("JNE" , BitInt::<5>::new(0x17).unwrap()),
    ("JLE" , BitInt::<5>::new(0x18).unwrap()),
    ("JGE" , BitInt::<5>::new(0x19).unwrap()),
    ("JMP" , BitInt::<5>::new(0x1A).unwrap()),
    ("JSR" , BitInt::<5>::new(0x1B).unwrap()),
    ("RTS" , BitInt::<5>::new(0x1C).unwrap()),
    ("TRAP", BitInt::<5>::new(0x1D).unwrap()), 
    ("RTE" , BitInt::<5>::new(0x1E).unwrap())
].iter().cloned().collect();
}


fn main() -> std::io::Result<()> {
    // https://stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust

    println!("Take an input asm..");
    
    let file_path: &str = "input.asm";
    let file: File = File::open(file_path)?;
    
    let reader = BufReader::new(file);
    let mut i = 0;
    for line in reader.lines() {
        
        let line = line?;

        print!("{i} | {line} | ");

        let tokens: Vec<&str> = parser::tokenize(&line);

        // print!("{} -> ", tokens[0].to_string());
        print!("{} -> ", tokens[0].to_string());

        let instruction: Vec<parser::InstructionFormat> = parser::parse(tokens, &INSTRUCTION_SET).unwrap();
        
        instruction.iter()
            .for_each(|instr| {
                println!("{}", instr.to_string())
            });
        
        i = i + 1;

    }

    // println!("{}", .join("\n"));

    Ok(())
}

