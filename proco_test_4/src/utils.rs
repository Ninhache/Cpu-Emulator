use std::ops::Deref;

#[derive(Clone)]
pub struct BitInt<const N: usize>(u16);

impl<const N: usize> BitInt<N> {
    pub fn new(value: u16) -> Option<Self> {
        if N > 16 {
            panic!("Cannot have more than 16 bits for u16");
        }
        if value < (1 << N) {
            Some(BitInt::<N>(value))
        } else {
            None
        }
    }
}

impl<const N: usize> Deref for BitInt<N> {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}






use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn write_color(str: &str, color: Color) -> io::Result<()> {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(color)))?;
    writeln!(&mut stdout, "{}", str)?;
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)))
}

pub fn log(str: &str) {
    match write_color(str, Color::Green) {
        Ok(()) => {}
        Err(err) => eprintln!("{}", err)
    }
}

pub fn alert(str: &str) {
    match write_color(str, Color::Red) {
        Ok(()) => {}
        Err(err) => eprintln!("{}", err)
    }
}

pub fn info(str: &str) {
    match write_color(str, Color::White) {
        Ok(()) => {}
        Err(err) => eprintln!("{}", err)
    }
}

pub fn shape(str: &str) {
    match write_color(str, Color::Rgb(35,35,35)) {
        Ok(()) => {}
        Err(err) => eprintln!("{}", err)
    }
}

