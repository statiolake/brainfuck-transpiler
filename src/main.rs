use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct Machine<'a> {
    program: &'a [u8],
    pc: usize,
    outfile: BufWriter<File>,
}

impl<'a> Drop for Machine<'a> {
    fn drop(&mut self) {
        writeln!(self.outfile, "    putchar('\\n');").unwrap();
        writeln!(self.outfile, "    return 0;").unwrap();
        writeln!(self.outfile, "}}").unwrap();
    }
}

impl<'a> Machine<'a> {
    pub fn new(program: &'a [u8]) -> Machine {
        let mut outfile =
            BufWriter::new(File::create("output.cpp").expect("failed to create output.cpp"));
        writeln!(outfile, "#include <bits/stdc++.h>").unwrap();
        writeln!(outfile, "int main() {{").unwrap();
        writeln!(outfile, "    std::vector<int> buffer;").unwrap();
        writeln!(outfile, "    size_t ptr = 0;").unwrap();
        Machine {
            program: program,
            pc: 0,
            outfile: outfile,
        }
    }

    pub fn forward_ptr(&mut self) {
        writeln!(self.outfile, "    ++ptr;").unwrap();
    }

    pub fn backward_ptr(&mut self) {
        writeln!(self.outfile, "    --ptr;").unwrap();
    }

    pub fn inc(&mut self) {
        self.realloc();
        writeln!(self.outfile, "    buffer[ptr]++;").unwrap();
    }

    pub fn dec(&mut self) {
        self.realloc();
        writeln!(self.outfile, "    buffer[ptr]--;").unwrap();
    }

    pub fn output(&mut self) {
        writeln!(self.outfile, "    putchar(buffer[ptr]);").unwrap()
    }

    pub fn input(&mut self) {
        self.realloc();
        writeln!(self.outfile, "    buffer[ptr] = getchar();").unwrap();
    }

    pub fn loop_begin(&mut self) {
        writeln!(self.outfile, "    while (buffer[ptr]) {{").unwrap();
    }

    pub fn loop_end(&mut self) {
        writeln!(self.outfile, "}}").unwrap();
    }

    pub fn step(&mut self) -> bool {
        if let Some(ch) = self.eat() {
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
        writeln!(self.outfile, "    if (ptr >= buffer.size()) {{").unwrap();
        writeln!(self.outfile, "        buffer.resize(ptr + 1);").unwrap();
        writeln!(self.outfile, "    }}").unwrap();
    }

    fn eat(&mut self) -> Option<char> {
        if self.pc >= self.program.len() {
            None
        } else {
            let res = self.program[self.pc] as char;
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
    drop(machine);

    Command::new("g++")
        .arg("-o")
        .arg("output")
        .arg("output.cpp")
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .unwrap();
}
