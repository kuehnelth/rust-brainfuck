use std::collections::VecDeque;

#[derive(Debug)]
pub enum Command {
    IncPointer,
    DecPointer,
    IncValue,
    DecValue,
    PutChar,
    GetChar,
    Loop(Vec<Command>),
}

#[derive(Debug)]
pub struct State {
    memory: VecDeque<u8>,
    pointer: usize,
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

    fn put_char(&self, writer: &mut dyn std::io::Write) {
        write!(writer, "{}", self.memory[self.pointer] as char).unwrap();
    }

    fn run_loop(&mut self, subprogram: &[Command], writer: &mut dyn std::io::Write) {
        while self.memory[self.pointer] != 0 {
            self.execute(&subprogram, writer);
        }
    }

    pub fn execute(&mut self, commands: &[Command], writer: &mut dyn std::io::Write) {
        for cmd in commands {
            match cmd {
                Command::IncPointer => self.inc_pointer(),
                Command::DecPointer => self.dec_pointer(),
                Command::IncValue => self.add_val(1),
                Command::DecValue => self.sub_val(1),
                Command::PutChar => self.put_char(writer),
                Command::GetChar => unimplemented!(),
                Command::Loop(subprogram) => self.run_loop(&subprogram, writer),
            };
        };
    }

}

pub fn parse(program: &mut std::str::Chars) -> Vec<Command> {
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
