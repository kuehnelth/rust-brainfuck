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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cat_a() {
        let mut output = Vec::new();
        let mut input = "A".as_bytes();
        let program = " <,.".parse().unwrap();
        let mut state = State::new();
        state.execute(&program, &mut input, &mut output);
        assert_eq!("A", std::str::from_utf8(&output).unwrap());
    }

    #[test]
    fn test_a() {
        let mut output = Vec::new();
        let program = "++++++++[>++++++++<-]>+.".parse().unwrap();
        let mut state = State::new();
        state.execute(&program, &mut std::io::empty(), &mut output);
        assert_eq!("A", std::str::from_utf8(&output).unwrap());
    }

    #[test]
    fn test_hello_world() {
        let mut output = Vec::new();
        let program = "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-."
            .parse()
            .unwrap();
        let mut state = State::new();
        state.execute(&program, &mut std::io::empty(), &mut output);
        assert_eq!("hello world", std::str::from_utf8(&output).unwrap());
    }
}
