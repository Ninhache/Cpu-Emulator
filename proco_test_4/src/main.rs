use std::fs::File;
use std::io::prelude::*;


fn main() -> std::io::Result<()> {
    // https://stackoverflow.com/questions/53826371/how-to-create-a-binary-file-with-rust
    {
        let mut file = File::create("test")?;
        file.write_all(&[0b01101101,0b00001101,0b10001101])?;
    }

    {
        let mut file = File::open("test")?;
        
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
    
        buffer.iter().for_each(|part| {
            println!("{:#010b}", part);
        });
        
    }

    Ok(())
}