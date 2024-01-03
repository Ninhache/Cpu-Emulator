use core::fmt;
use std::ops::Deref;

#[derive(Clone, Debug, Copy)]
pub struct BitInt<const N: usize>(u16);

impl<const N: usize> fmt::Display for BitInt<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = if N < 16 {
            self.0 & ((1 << N) - 1)
        } else {
            self.0
        };

        let binary_str = format!("{:0width$b}", value, width = N);

        write!(f, "{}", binary_str)
    }
}

impl<const N: usize> BitInt<N> {
    pub fn new(value: u16) -> Option<Self> {
        if N > 16 {
            panic!("BitInt cannot represent more than 16 bits");
        }
        if N == 16 {
            Some(BitInt::<N>(value))
        } else if value < (1 << N) {
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

impl<const N: usize> PartialEq for BitInt<N> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<const N: usize> PartialOrd for BitInt<N> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
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

