use std::collections::VecDeque;
use std::io::Read;
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
    IncPointer(u8),
    DecPointer(u8),
    IncValue(u8),
    DecValue(u8),
    PutChar,
    GetChar,
    Loop(Program),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    MissingClosingBracket(u32),
}

#[derive(Debug, Default)]
pub struct State {
    memory: VecDeque<u8>,
    pointer: usize,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct Program {
    commands: Vec<Command>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            commands: Vec::new(),
        }
    }

    fn parse(buf: &[u8]) -> Result<Program, ParseError> {
        let mut res: Vec<(u8, char)> = Vec::new();
        if buf.len() > 0 {
            let mut byte = buf[0];
            let mut reps = 1;

            for curr_byte in &buf[1..] {
                if byte == *curr_byte && "+-><".bytes().find(|c| *c == *curr_byte).is_some() {
                    reps += 1;
                } else  {
                    res.push((reps, byte as char));
                    reps = 1;
                    byte = *curr_byte;
                }
            }
            res.push((reps, byte as char));
        }
        let mut iter = res.into_iter();
        Program::parse_depth(&mut iter, 0)
    }

    fn parse_depth(program : &mut std::vec::IntoIter<(u8, char)>, depth: u32) -> Result<Program, ParseError> {
        let mut result: Program = Program::new();
        while let Some(cmd) = program.next() {
            if let Some(pcmd) = match cmd {
                (i, '>') => Some(Command::IncPointer(i)),
                (i, '<') => Some(Command::DecPointer(i)),
                (i, '+') => Some(Command::IncValue(i)),
                (i, '-') => Some(Command::DecValue(i)),
                (1, '.') => Some(Command::PutChar),
                (1, ',') => Some(Command::GetChar),
                (1, '[') => Some(Command::Loop(Program::parse_depth(program, depth + 1)?)),
                (1, ']') => return Ok(result),
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
        let bytes = s.as_bytes();
        Program::parse(bytes)
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

    fn inc_pointer(&mut self, val: usize) {
        self.pointer = self.pointer.wrapping_add(val);
        while self.pointer >= self.memory.len() - 1 {
            self.memory.push_back(0);
        }
    }

    fn dec_pointer(&mut self, val: usize) {
        if val > self.pointer {
            for _ in 0..val - self.pointer {
                self.memory.push_front(0);
            }
        } else {
            self.pointer = self.pointer.wrapping_sub(val);
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
                Command::IncPointer(i) => self.inc_pointer(*i as usize),
                Command::DecPointer(i) => self.dec_pointer(*i as usize),
                Command::IncValue(i) => self.add_val(*i),
                Command::DecValue(i) => self.sub_val(*i),
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

    #[test]
    fn test_missing_bracket() {
        let program : Result<Program, ParseError> = "[".parse();
        assert_eq!(Err(ParseError::MissingClosingBracket(1)), program);
    }
}
