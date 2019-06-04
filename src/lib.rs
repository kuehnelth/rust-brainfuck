use std::collections::VecDeque;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    IncPointer,
    DecPointer,
    IncValue,
    DecValue,
    PutChar,
    GetChar,
    Loop(Program),
}

#[derive(Debug)]
pub enum ParseError {
    MissingClosingBracket(u32),
}

#[derive(Debug, Default)]
pub struct State {
    memory: VecDeque<u8>,
    pointer: usize,
}

#[derive(Debug, Default)]
pub struct Program {
    commands: Vec<Command>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            commands: Vec::new(),
        }
    }

    fn parse(program: &mut std::str::Chars) -> Result<Program, ParseError> {
        Program::parse_depth(program, 0)
    }

    fn parse_depth(program: &mut std::str::Chars, depth: u32) -> Result<Program, ParseError> {
        let mut result: Program = Program::new();
        while let Some(cmd) = program.next() {
            if let Some(pcmd) = match cmd {
                '>' => Some(Command::IncPointer),
                '<' => Some(Command::DecPointer),
                '+' => Some(Command::IncValue),
                '-' => Some(Command::DecValue),
                '.' => Some(Command::PutChar),
                ',' => Some(Command::GetChar),
                '[' => Some(Command::Loop(Program::parse_depth(program, depth + 1)?)),
                ']' => return Ok(result),
                _ => None,
            } {
                result.commands.push(pcmd);
            };
        }
        if depth == 0 {
            Ok(result)
        } else {
            Err(ParseError::MissingClosingBracket(depth))
        }
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Program, ParseError> {
        let mut chars = s.chars();
        Program::parse(&mut chars)
    }
}

impl State {
    pub fn new() -> State {
        let mut s = State {
            memory: VecDeque::new(),
            pointer: 0,
        };
        s.memory.push_back(0);
        s
    }

    fn inc_pointer(&mut self) {
        if self.pointer == self.memory.len() - 1 {
            self.memory.push_back(0);
        }
        self.pointer = self.pointer.wrapping_add(1);
    }

    fn dec_pointer(&mut self) {
        if self.pointer == 0 {
            self.memory.push_front(0);
        } else {
            self.pointer = self.pointer.wrapping_sub(1);
        }
    }

    fn add_val(&mut self, val: u8) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_add(val);
    }

    fn sub_val(&mut self, val: u8) {
        self.memory[self.pointer] = self.memory[self.pointer].wrapping_sub(val);
    }

    fn get_char(&mut self, reader: &mut dyn std::io::Read) {
        self.memory[self.pointer] = reader.bytes().next().unwrap_or(Ok(0)).unwrap();
    }

    fn put_char(&self, writer: &mut dyn std::io::Write) {
        write!(writer, "{}", self.memory[self.pointer] as char).unwrap();
    }

    fn run_loop(
        &mut self,
        subprogram: &Program,
        reader: &mut dyn std::io::Read,
        writer: &mut dyn std::io::Write,
    ) {
        while self.memory[self.pointer] != 0 {
            self.execute(&subprogram, reader, writer);
        }
    }

    pub fn execute(
        &mut self,
        program: &Program,
        reader: &mut dyn std::io::Read,
        writer: &mut dyn std::io::Write,
    ) {
        for cmd in &program.commands {
            match cmd {
                Command::IncPointer => self.inc_pointer(),
                Command::DecPointer => self.dec_pointer(),
                Command::IncValue => self.add_val(1),
                Command::DecValue => self.sub_val(1),
                Command::GetChar => self.get_char(reader),
                Command::PutChar => self.put_char(writer),
                Command::Loop(subprogram) => self.run_loop(&subprogram, reader, writer),
            };
        }
    }
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
    fn test_empty() {
        let mut output = Vec::new();
        let program = "".parse().unwrap();
        let mut state = State::new();
        state.execute(&program, &mut std::io::empty(), &mut output);
        assert_eq!("", std::str::from_utf8(&output).unwrap());
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
