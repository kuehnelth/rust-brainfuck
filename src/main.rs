use brainfuck::*;
use std::env;
use std::fs;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        panic!("Please provide a filename to execute");
    }
    let contents = fs::read_to_string(&args[1]).expect("Something went wrong reading the file");
    let program = contents.parse().unwrap();
    let mut state = State::new();
    state.execute(&program, &mut std::io::stdin(), &mut std::io::stdout());
}
