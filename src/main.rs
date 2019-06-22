use std::fs::File;
use std::io::{Read, stdin, stdout, Write};

use crate::brainfuck_interpreter::BrainFuck;

mod brainfuck_interpreter;

fn main() {
    // Input Path
    let mut path = String::new();
    print!("Input File Path: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut path).expect("");
    let path = path.trim();

    // get file content
    let mut code = String::with_capacity(50);
    File::open(path)
        .expect(format!("file not found {}", path).as_str())
        .read_to_string(&mut code).unwrap();

    // brainfuck eval
    let mut bf = BrainFuck::new();
    let cb = |x: &BrainFuck| println!("{:?}", x);

    // bf.callback(cb.borrow());
    let output = bf.interpret(&code);
    println!("{}", output);
}
