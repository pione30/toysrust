use std::fs;
use std::str;

fn main() {
    let contents = fs::read("examples/factorial.toys").unwrap();
    let contents = str::from_utf8(&contents).unwrap();

    let (_, program) = toysrust::parser::program(contents).unwrap();
    let mut interpreter = toysrust::interpreter::Interpreter::new();
    interpreter.call_main(program).unwrap();
}
