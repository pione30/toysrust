use std::fs;
use std::str;
use toysrust::{interpreter, parser};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents = fs::read("examples/factorial.toys")?;
    let contents = str::from_utf8(&contents)?;

    let (_, program) = parser::program(contents).unwrap();
    let mut interpreter = interpreter::Interpreter::default();
    interpreter.call_main(program)?;

    Ok(())
}
