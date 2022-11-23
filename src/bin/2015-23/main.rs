use core::panic;

use logos::{Lexer, Logos};

#[derive(Logos, PartialEq)]
enum Token {
    #[token("hlf")]
    Hlf,

    #[token("tpl")]
    Tpl,

    #[token("inc")]
    Inc,

    #[token("jmp")]
    Jmp,

    #[token("jie")]
    Jie,

    #[token("jio")]
    Jio,

    #[token("a")]
    RegisterA,

    #[token("b")]
    RegisterB,

    #[regex(r"(\+|-)\d+")]
    Offset,

    #[error]
    #[regex(r"[ \t\n\f,]+", logos::skip)]
    Error,
}

#[derive(Debug)]
enum Register {
    A,
    B,
}

#[derive(Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i32),
    Jie(Register, i32),
    Jio(Register, i32),
}

struct ProgramLoader<'a> {
    lexer: Lexer<'a, Token>,
}

impl<'a> ProgramLoader<'a> {
    fn load_program(input: &str) -> Vec<Instruction> {
        let mut pl = ProgramLoader {
            lexer: Token::lexer(input),
        };
        pl.parse()
    }

    fn parse(&mut self) -> Vec<Instruction> {
        let mut program = Vec::new();
        while let Some(t) = self.lexer.next() {
            match t {
                Token::Hlf => program.push(Instruction::Hlf(self.consume_register())),
                Token::Tpl => program.push(Instruction::Tpl(self.consume_register())),
                Token::Inc => program.push(Instruction::Inc(self.consume_register())),
                Token::Jmp => program.push(Instruction::Jmp(self.consume_offset())),
                Token::Jie => program.push(Instruction::Jie(
                    self.consume_register(),
                    self.consume_offset(),
                )),
                Token::Jio => program.push(Instruction::Jio(
                    self.consume_register(),
                    self.consume_offset(),
                )),
                Token::RegisterA | Token::RegisterB | Token::Offset => {
                    panic!("Unexpected token: {}", self.lexer.slice());
                }
                Token::Error => (),
            }
        }
        program
    }

    fn consume_register(&mut self) -> Register {
        if let Some(t) = self.lexer.next() {
            match t {
                Token::RegisterA => Register::A,
                Token::RegisterB => Register::B,
                _ => panic!("Unexpected token"),
            }
        } else {
            panic!("Unexpected end");
        }
    }

    fn consume_offset(&mut self) -> i32 {
        if let Some(t) = self.lexer.next() {
            if t == Token::Offset {
                self.lexer.slice().parse().unwrap()
            } else {
                panic!("Unexpected token: {}", self.lexer.slice());
            }
        } else {
            panic!("Unexpected end");
        }
    }
}

#[derive(Debug)]
struct Computer {
    a: u32,
    b: u32,

    pc: usize,

    program: Vec<Instruction>,
}

impl Computer {
    fn new() -> Computer {
        Computer {
            a: 0,
            b: 0,
            pc: 0,
            program: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.a = 0;
        self.b = 0;
        self.pc = 0;
    }

    fn load_program(&mut self, input: &str) {
        self.program = ProgramLoader::load_program(input);
    }

    fn run(&mut self) {
        while self.pc < self.program.len() {
            match &self.program[self.pc] {
                Instruction::Hlf(register) => match register {
                    Register::A => self.a /= 2,
                    Register::B => self.b /= 2,
                },
                Instruction::Tpl(register) => match register {
                    Register::A => self.a *= 3,
                    Register::B => self.b *= 3,
                },
                Instruction::Inc(register) => match register {
                    Register::A => self.a += 1,
                    Register::B => self.b += 1,
                },
                Instruction::Jmp(offset) => {
                    self.move_pc(*offset);
                    continue;
                }
                Instruction::Jie(register, offset) => {
                    if match register {
                        Register::A => self.a % 2 == 0,
                        Register::B => self.b % 2 == 0,
                    } {
                        self.move_pc(*offset);
                        continue;
                    }
                }
                Instruction::Jio(register, offset) => {
                    if match register {
                        Register::A => self.a == 1,
                        Register::B => self.b == 1,
                    } {
                        self.move_pc(*offset);
                        continue;
                    }
                }
            }
            self.pc += 1;
        }
    }

    fn move_pc(&mut self, offset: i32) {
        let new_pc = self.pc as i32 + offset;
        if new_pc >= 0 {
            self.pc = new_pc as usize;
        } else {
            panic!(
                "Moved pc out of bounds!\n{:?}\nOffset {} would have moved pc to {}.",
                self, offset, new_pc
            );
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/2015/23.txt").expect("input should exist");

    let mut comp = Computer::new();
    comp.load_program(&input);
    comp.run();

    println!("The value of register b is {}.", comp.b);

    comp.reset();
    comp.a = 1;
    comp.run();

    println!(
        "If a starts with the value 1, the value of register b is {}.",
        comp.b
    );
}
