use std::io;

use crate::parser::{lexer::Lexer, token::Token};

pub struct Parser {
    lexer: Lexer,
    current_statement: Vec<Token>,
    imported_items: Vec<String>,
}

impl Parser {
    pub fn new(file: String) -> io::Result<Self> {
        Ok(Self {
            lexer: Lexer::new(file)?,
            current_statement: vec![],
            imported_items: vec![],
        })
    }
}
