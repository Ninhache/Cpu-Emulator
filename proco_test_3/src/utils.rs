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

pub fn err(str: &str) {
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

