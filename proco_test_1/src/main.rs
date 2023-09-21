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
    fn new() -> Self {
        Registers {
            r: [0; 8], // Initialize all registers to 0
            sr: [false; 16],
        }
    }
}

fn main() {
    println!("Hello, world!");
}
