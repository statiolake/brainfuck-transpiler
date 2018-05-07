use std::env;
use std::fs::File;
use std::io;
use std::io::Read;

pub struct Machine<'a> {
    buffer: Vec<i64>,
    program: &'a [u8],
    pc: usize,
    ptr: usize,
    loop_pos: Vec<usize>,
}

impl<'a> Machine<'a> {
    pub fn new(program: &[u8]) -> Machine {
        Machine {
            buffer: Vec::new(),
            program: program,
            pc: 0,
            ptr: 0,
            loop_pos: Vec::new(),
        }
    }

    pub fn forward_ptr(&mut self) {
        self.ptr += 1;
    }

    pub fn backward_ptr(&mut self) {
        if self.ptr == 0 {
            panic!("instruction pointer must not be negative!");
        }
        self.ptr -= 1;
    }

    pub fn inc(&mut self) {
        self.realloc();
        self.buffer[self.ptr] += 1;
    }

    pub fn dec(&mut self) {
        self.realloc();
        self.buffer[self.ptr] -= 1;
    }

    pub fn output(&self) {
        print!("{}", self.buffer[self.ptr] as u8 as char);
    }

    pub fn input(&mut self) {
        self.realloc();
        let mut cha = [' ' as u8];
        let stdin = io::stdin();
        stdin
            .lock()
            .read_exact(&mut cha)
            .expect("input not supplied");
        self.buffer[self.ptr] = cha[0] as i64;
    }

    pub fn loop_begin(&mut self) {
        if self.buffer[self.ptr] != 0 {
            self.loop_pos.push(self.pc - 1);
        } else {
            let mut level = 1;
            while level != 0 {
                match self.eat(None).expect("no matching `end of loop`") {
                    '[' => level += 1,
                    ']' => level -= 1,
                    _ => {}
                }
            }
        }
    }

    pub fn loop_end(&mut self) {
        self.pc = self.loop_pos
            .pop()
            .expect("internal error: no more `beginning of loop`.");
    }

    pub fn step(&mut self) -> bool {
        if let Some(ch) = self.eat(None) {
            // eprintln!("current pc is {}, is {}", self.pc, ch);
            match ch {
                '>' => self.forward_ptr(),
                '<' => self.backward_ptr(),
                '+' => self.inc(),
                '-' => self.dec(),
                '.' => self.output(),
                ',' => self.input(),
                '[' => self.loop_begin(),
                ']' => self.loop_end(),
                _ => {}
            }
            true
        } else {
            false
        }
    }

    fn realloc(&mut self) {
        if self.ptr >= self.buffer.len() {
            self.buffer.resize(self.ptr + 1, 0);
        }
    }

    fn eat(&mut self, expect: Option<char>) -> Option<char> {
        if self.pc >= self.program.len() {
            None
        } else {
            let res = self.program[self.pc] as char;
            if let Some(expect) = expect {
                assert_eq!(res, expect);
            }
            self.pc += 1;
            Some(res)
        }
    }
}

fn main() {
    let mut infile = File::open(&env::args()
        .nth(1)
        .expect("please specify program file to run as 1st argument."))
        .expect(
        "failed to open specified file.",
    );
    let mut program = String::new();
    infile
        .read_to_string(&mut program)
        .expect("failed to get contents of program file.");
    let mut machine = Machine::new(program.as_bytes());
    while machine.step() {}
    println!("");
}
