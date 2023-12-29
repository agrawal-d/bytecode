use crate::scanner::{Scanner, Token, TokenType};
use anyhow::*;
use std::rc::Rc;

pub fn compile(source: Rc<String>) -> Result<()> {
    let line: usize = 0;
    let mut scanner = Scanner::new(source);

    loop {
        let token = scanner.scan_token();
        let scanner_line = scanner.line;
        trace(&token, scanner_line);

        match token.typ {
            TokenType::EOF => break,
            _ => todo!(),
        };
    }

    Ok(())
}

fn trace(token: &Token, scanner_line: usize) {
    if token.line != scanner_line {
        print!("{:4 }", token.line);
    } else {
        print!("   | ");
    }

    println!("{:12} '{}'", token.typ, token.source);
}
