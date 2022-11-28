use std::collections::HashMap;

use logos::{Lexer, Logos};

#[derive(Debug, Logos, PartialEq)]
enum Token {
    #[regex("[a-z]+")]
    Identifier,

    #[regex("[0-9]+")]
    Literal,

    #[token("NOT")]
    Not,

    #[token("AND")]
    And,

    #[token("OR")]
    Or,

    #[token("LSHIFT")]
    Lshift,

    #[token("RSHIFT")]
    Rshift,

    #[token("->")]
    Arrow,

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Debug)]
enum Value {
    Identifier(String),
    Literal(i32),
}

#[derive(Debug)]
enum Instruction {
    Store(Value),
    Not(Value),
    And(Value, Value),
    Or(Value, Value),
    Lshift(Value, Value),
    Rshift(Value, Value),
}

struct Circuit<'a> {
    wires: HashMap<String, Instruction>,
    lexer: Lexer<'a, Token>,
    peeked: Option<Option<Token>>,
}

impl<'a> Circuit<'a> {
    fn new(input: &'a str) -> Self {
        let mut circuit = Circuit {
            wires: HashMap::new(),
            lexer: Token::lexer(input),
            peeked: None,
        };

        circuit.parse();

        circuit
    }

    fn solve(&self, circuit: &mut HashMap<String, i32>) {
        for wire_name in self.wires.keys() {
            if let Some(_) = circuit.get(wire_name) {
                continue;
            }

            self.get_wire(&wire_name, circuit);
        }
    }

    fn get_wire(&self, wire_name: &str, cache: &mut HashMap<String, i32>) -> i32 {
        if let Some(val) = cache.get(wire_name) {
            return *val;
        }

        let val = match self.wires.get(wire_name).unwrap() {
            Instruction::Store(v) => self.resolve_value(v, cache),
            Instruction::Not(v) => !self.resolve_value(v, cache),
            Instruction::And(v, b) => self.resolve_value(v, cache) & self.resolve_value(b, cache),
            Instruction::Or(v, b) => self.resolve_value(v, cache) | self.resolve_value(b, cache),
            Instruction::Lshift(v, b) => {
                self.resolve_value(v, cache) << self.resolve_value(b, cache)
            }
            Instruction::Rshift(v, b) => {
                self.resolve_value(v, cache) >> self.resolve_value(b, cache)
            }
        };
        cache.insert(String::from(wire_name), val);
        val
    }

    fn resolve_value(&self, val: &Value, cache: &mut HashMap<String, i32>) -> i32 {
        match val {
            Value::Literal(l) => *l,
            Value::Identifier(i) => self.get_wire(i, cache),
        }
    }

    fn parse(&mut self) {
        while let Some(t) = self.peek_token() {
            match t {
                Token::Not => self.consume_not(),
                Token::Identifier | Token::Literal => {
                    let left = self.consume_value();
                    if let Some(t) = self.peek_token() {
                        match t {
                            Token::Arrow => self.add_wire(Instruction::Store(left)),
                            Token::And | Token::Or | Token::Lshift | Token::Rshift => {
                                self.consume_infix(left)
                            }
                            _ => panic!("Unexpected token"),
                        }
                    } else {
                        panic!("Unexpected end");
                    }
                }
                _ => panic!("Unexpected case"),
            }
        }
    }

    fn peek_token(&mut self) -> &Option<Token> {
        if self.peeked.is_none() {
            self.peeked = Some(self.lexer.next());
        }
        self.peeked.as_ref().unwrap()
    }

    fn next_token(&mut self) -> Option<Token> {
        if let Some(peeked) = self.peeked.take() {
            peeked
        } else {
            self.lexer.next()
        }
    }

    fn consume_value(&mut self) -> Value {
        if let Some(t) = self.next_token() {
            match t {
                Token::Identifier => Value::Identifier(String::from(self.lexer.slice())),
                Token::Literal => Value::Literal(self.lexer.slice().parse().unwrap()),
                _ => {
                    panic!("Expected value");
                }
            }
        } else {
            panic!("Unexpected end");
        }
    }

    fn consume_not(&mut self) {
        self.next_token();
        let val = self.consume_value();
        let instruction = Instruction::Not(val);
        self.add_wire(instruction);
    }

    fn consume_infix(&mut self, left: Value) {
        if let Some(instruction_token) = self.next_token() {
            let right = self.consume_value();
            let instruction = match instruction_token {
                Token::And => Instruction::And(left, right),
                Token::Or => Instruction::Or(left, right),
                Token::Lshift => Instruction::Lshift(left, right),
                Token::Rshift => Instruction::Rshift(left, right),
                _ => panic!("Expected infix instruction"),
            };
            self.add_wire(instruction);
        }
    }

    fn add_wire(&mut self, instruction: Instruction) {
        if let Some(mut t) = self.next_token() {
            if t == Token::Arrow {
                t = self.next_token().expect("Unexpected end");
            }
            if t != Token::Identifier {
                panic!("Expected identifier");
            }

            self.wires
                .insert(String::from(self.lexer.slice()), instruction);
        } else {
            panic!("Unexpected end");
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input/07.txt").expect("input should exist");
    let circuit = Circuit::new(&input);

    let mut circuit_a = HashMap::new();
    circuit.solve(&mut circuit_a);
    let a_val = circuit_a.get("a").unwrap();

    let mut circuit_b = HashMap::new();
    circuit_b.insert(String::from("b"), *a_val);
    circuit.solve(&mut circuit_b);

    println!(
        "The signal for wire a is {} and if wire b has the value {} a's value is {}.",
        a_val,
        a_val,
        circuit_b.get("a").unwrap(),
    );
}
