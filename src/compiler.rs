use crate::{
    chunk::Chunk,
    scanner::{Scanner, Token, TokenType},
};
use anyhow::*;
use log::error;
use std::rc::Rc;

struct Parser {
    pub scanner: Scanner,
    pub current: Token,
    pub previous: Token,
    pub had_error: bool,
    pub panic_mode: bool,
}

impl Parser {
    fn new(scanner: Scanner) -> Parser {
        Parser {
            scanner,
            current: Token::new(),
            previous: Token::new(),
            had_error: false,
            panic_mode: false,
        }
    }

    fn error_at_current(&mut self, message: &str) {
        self.error_at(true, message);
    }

    fn error_at_previous(&mut self, message: &str) {
        self.error_at(false, message);
    }

    fn error_at(&mut self, current: bool, message: &str) {
        let current = if current {
            &self.current
        } else {
            &self.previous
        };

        if self.panic_mode {
            return;
        }

        self.panic_mode = true;
        eprint!("[line {}] Error", current.line);

        if current.typ == TokenType::EOF {
            eprint!(" at end");
        } else if current.typ == TokenType::Error {
            // Nothing.
        } else {
            eprint!(" at '{}'", current.source);
        }

        eprintln!(": {}", message);
        self.had_error = true;
    }

    fn consume(&mut self, typ: TokenType, message: &str) {
        if self.current.typ == typ {
            self.advance();
            return;
        }

        self.error_at_current(message);
    }

    fn advance(&mut self) {
        self.previous = self.current.clone();

        loop {
            self.current = self.scanner.scan_token();
            if self.current.typ != TokenType::Error {
                break;
            }

            let current_source: Rc<str> = self.current.source.clone();
            self.error_at_current(current_source.as_ref());
        }
    }

    fn expression(&mut self) {}
}

pub struct Compiler {
    compiling_chunk: Chunk,
    line: usize,
    parser: Parser,
}

impl Compiler {
    pub fn compile(source: Rc<str>) -> Result<Chunk> {
        let line: usize = 0;
        let scanner: Scanner = Scanner::new(source);
        let parser = Parser::new(scanner);
        let mut compiler = Compiler {
            compiling_chunk: Chunk::default(),
            line,
            parser,
        };

        compiler.parser.advance();
        compiler.parser.expression();
        compiler
            .parser
            .consume(TokenType::EOF, "Expect end of expression.");
        compiler.end();
        todo!("Which chunk to return?");
    }

    fn current_chunk(&mut self) -> &mut Chunk {
        &mut self.compiling_chunk
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.parser.previous.line;
        self.current_chunk().write_byte(byte, line)
    }

    fn emit_bytes(&mut self, byte1: u8, byte2: u8) {
        self.emit_byte(byte1);
        self.emit_byte(byte2);
    }

    fn trace(&self, token: &Token, scanner_line: usize) {
        if token.line != scanner_line {
            print!("{:4 }", token.line);
        } else {
            print!("   | ");
        }

        println!("{:12} '{}'", token.typ, token.source);
    }

    fn emit_return(&mut self) {
        self.emit_byte(TokenType::Return as u8)
    }

    fn end(&mut self) {
        self.emit_return();
    }
}
