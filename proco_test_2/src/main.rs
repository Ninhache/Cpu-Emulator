extern crate maplit;

mod instructions;
mod opcode;


fn main() {
    println!("Hello, world!");

    for (opcode, function) in opcode::OPCODES.iter() {
        println!("Opcode: {:?}, Function: {:?}", opcode, function);
    }
}