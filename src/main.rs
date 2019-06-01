#![feature(test)]

extern crate test;
use std::collections::VecDeque;
use std::fs;
use std::env;

#[derive(Debug)]
enum Command {
    IncPointer,
    DecPointer,
    IncValue,
    DecValue,
    PutChar,
    GetChar,
    Loop(Vec<Command>),
}

#[derive(Debug)]
struct State {
    memory: VecDeque<u8>,
    pointer: usize,
}

impl State {
    fn new() -> State {
        let mut s = State {
            memory: VecDeque::new(),
            pointer: 0,
        };
        s.memory.push_back(0);
        s
    }
}

fn parse(program: &mut std::str::Chars) -> Vec<Command> {
    let mut result: Vec<Command> = Vec::new();
    while let Some(cmd) = program.next() {
        if let Some(pcmd) = match cmd {
            '>' => Some(Command::IncPointer),
            '<' => Some(Command::DecPointer),
            '+' => Some(Command::IncValue),
            '-' => Some(Command::DecValue),
            '.' => Some(Command::PutChar),
            ',' => Some(Command::GetChar),
            '[' => Some(Command::Loop(parse(program))),
            ']' => return result,
            _ => None,
        } {
            result.push(pcmd);
        };
    }
    result
}

fn execute(state: &mut State, commands: &Vec<Command>) {
    //println!(">> {:?}", commands);
    for cmd in commands {
        match cmd {
            Command::IncPointer => {
                if state.pointer == state.memory.len() - 1 {
                    state.memory.push_back(0);
                }
                state.pointer = state.pointer.wrapping_add(1);
            }
            Command::DecPointer => {
                if state.pointer == 0 {
                    state.memory.push_front(0);
                } else {
                    state.pointer = state.pointer.wrapping_sub(1);
                }
            }
            Command::IncValue => {
                state.memory[state.pointer] = state.memory[state.pointer].wrapping_add(1);
            }
            Command::DecValue => {
                state.memory[state.pointer] = state.memory[state.pointer].wrapping_sub(1);
            }
            Command::PutChar => {
                print!("{}", state.memory[state.pointer] as char);
            }
            Command::GetChar => {
                unimplemented!();
            }
            Command::Loop(subprogram) => {
                while state.memory[state.pointer] != 0 {
                    //println!("LOOP {}", *state.memory.get(&state.pointer).unwrap_or(&0));
                    execute(state, &subprogram);
                }
            }
        };
        //println!("{:?}", state);
    };
}

fn main() {
    //let commands = parse(&mut "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.".chars());
    let args: Vec<_> = env::args().collect();
    let contents = fs::read_to_string(&args[1])
        .expect("Something went wrong reading the file");
    let commands = parse(&mut contents.chars());
    let mut state = State::new();
    execute(&mut state, &commands);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    /*
    #[test]
    fn test_a() {
        let mut output = Vec::new();
        let commands = parse(&mut "++++++++[>++++++++<-]>+.".chars());
        let mut state = State::new();
        execute(&mut output, &mut state, &commands);
        assert_eq!("A", output);
    }

    #[test]
    fn test_hello_world() {
        let mut output = Vec::new();
        let commands =
            parse(&mut "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.".chars());
        let mut state = State::new();
        let result = execute(&mut output, &mut state, &commands);
        //assert_eq!("hello world", result);
    }
    */

    #[bench]
    fn bench_hello_world(b: &mut Bencher) {
        let commands =
            parse(&mut "+[-[<<[+[--->]-[<<<]]]>>>-]>-.---.>..>.<<<<-.<+.>>>>>.>.<<.<-.".chars());
        let mut state = State::new();
        //let result = execute(&mut state, &commands);
        b.iter(|| execute(&mut state, &commands));
    }

    #[bench]
    fn bench_bottles(b: &mut Bencher) {
        let commands = parse(
            &mut "99 Bottles of Beer in Urban Mueller's BrainF*** (The actual
name is impolite)

by Ben Olmstead

ANSI C interpreter available on the internet; due to
constraints in comments the address below needs to have the
stuff in parenthesis replaced with the appropriate symbol:

http://www(dot)cats(dash)eye(dot)com/cet/soft/lang/bf/

Believe it or not this language is indeed Turing complete!
Combines the speed of BASIC with the ease of INTERCAL and
the readability of an IOCCC entry!

>+++++++++[<+++++++++++>-]<[>[-]>[-]<<[>+>+<<-]>>[<<+>>-]>>>
[-]<<<+++++++++<[>>>+<<[>+>[-]<<-]>[<+>-]>[<<++++++++++>>>+<
-]<<-<-]+++++++++>[<->-]>>+>[<[-]<<+>>>-]>[-]+<<[>+>-<<-]<<<
[>>+>+<<<-]>>>[<<<+>>>-]>[<+>-]<<-[>[-]<[-]]>>+<[>[-]<-]<+++
+++++[<++++++<++++++>>-]>>>[>+>+<<-]>>[<<+>>-]<[<<<<<.>>>>>-
]<<<<<<.>>[-]>[-]++++[<++++++++>-]<.>++++[<++++++++>-]<++.>+
++++[<+++++++++>-]<.><+++++..--------.-------.>>[>>+>+<<<-]>
>>[<<<+>>>-]<[<<<<++++++++++++++.>>>>-]<<<<[-]>++++[<+++++++
+>-]<.>+++++++++[<+++++++++>-]<--.---------.>+++++++[<------
---->-]<.>++++++[<+++++++++++>-]<.+++..+++++++++++++.>++++++
++[<---------->-]<--.>+++++++++[<+++++++++>-]<--.-.>++++++++
[<---------->-]<++.>++++++++[<++++++++++>-]<++++.-----------
-.---.>+++++++[<---------->-]<+.>++++++++[<+++++++++++>-]<-.
>++[<----------->-]<.+++++++++++..>+++++++++[<---------->-]<
-----.---.>>>[>+>+<<-]>>[<<+>>-]<[<<<<<.>>>>>-]<<<<<<.>>>+++
+[<++++++>-]<--.>++++[<++++++++>-]<++.>+++++[<+++++++++>-]<.
><+++++..--------.-------.>>[>>+>+<<<-]>>>[<<<+>>>-]<[<<<<++
++++++++++++.>>>>-]<<<<[-]>++++[<++++++++>-]<.>+++++++++[<++
+++++++>-]<--.---------.>+++++++[<---------->-]<.>++++++[<++
+++++++++>-]<.+++..+++++++++++++.>++++++++++[<---------->-]<
-.---.>+++++++[<++++++++++>-]<++++.+++++++++++++.++++++++++.
------.>+++++++[<---------->-]<+.>++++++++[<++++++++++>-]<-.
-.---------.>+++++++[<---------->-]<+.>+++++++[<++++++++++>-
]<--.+++++++++++.++++++++.---------.>++++++++[<---------->-]
<++.>+++++[<+++++++++++++>-]<.+++++++++++++.----------.>++++
+++[<---------->-]<++.>++++++++[<++++++++++>-]<.>+++[<----->
-]<.>+++[<++++++>-]<..>+++++++++[<--------->-]<--.>+++++++[<
++++++++++>-]<+++.+++++++++++.>++++++++[<----------->-]<++++
.>+++++[<+++++++++++++>-]<.>+++[<++++++>-]<-.---.++++++.----
---.----------.>++++++++[<----------->-]<+.---.[-]<<<->[-]>[
-]<<[>+>+<<-]>>[<<+>>-]>>>[-]<<<+++++++++<[>>>+<<[>+>[-]<<-]
>[<+>-]>[<<++++++++++>>>+<-]<<-<-]+++++++++>[<->-]>>+>[<[-]<
<+>>>-]>[-]+<<[>+>-<<-]<<<[>>+>+<<<-]>>>[<<<+>>>-]<>>[<+>-]<
<-[>[-]<[-]]>>+<[>[-]<-]<++++++++[<++++++<++++++>>-]>>>[>+>+
<<-]>>[<<+>>-]<[<<<<<.>>>>>-]<<<<<<.>>[-]>[-]++++[<++++++++>
-]<.>++++[<++++++++>-]<++.>+++++[<+++++++++>-]<.><+++++..---
-----.-------.>>[>>+>+<<<-]>>>[<<<+>>>-]<[<<<<++++++++++++++
.>>>>-]<<<<[-]>++++[<++++++++>-]<.>+++++++++[<+++++++++>-]<-
-.---------.>+++++++[<---------->-]<.>++++++[<+++++++++++>-]
<.+++..+++++++++++++.>++++++++[<---------->-]<--.>+++++++++[
<+++++++++>-]<--.-.>++++++++[<---------->-]<++.>++++++++[<++
++++++++>-]<++++.------------.---.>+++++++[<---------->-]<+.
>++++++++[<+++++++++++>-]<-.>++[<----------->-]<.+++++++++++
..>+++++++++[<---------->-]<-----.---.+++.---.[-]<<<]
"
            .chars(),
        );
        let mut state = State::new();
        //let result = execute(&mut state, &commands);
        b.iter(|| execute(&mut state, &commands));
    }
}
