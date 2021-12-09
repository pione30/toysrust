use std::fs;
use std::str;
use toysrust::{interpreter, parser};

fn main() {
    let contents = fs::read("examples/factorial.toys").unwrap();
    let contents = str::from_utf8(&contents).unwrap();

    let (_, program) = parser::program(contents).unwrap();
    let mut interpreter = interpreter::Interpreter::default();
    interpreter.call_main(program).unwrap();
}
