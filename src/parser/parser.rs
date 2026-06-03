use crate::parser::{
    lexer::Lexer,
    parser::ParseError::{IoError, LexError},
    token::Token,
};

pub enum ParseError {
    UnexpectedToken { expected: String, got: Vec<Token> },
    UnexpectedEOF,
    LexError,
    IndexOutOfBounds,
    IoError(std::io::Error),
}
pub struct Parser {
    tokens: Vec<Token>,
    ind: usize,
    curr_token: Option<Token>,
    current_statement: Vec<Token>,
    imported_items: Vec<String>,
    package: String,
}

/// initialization and parsing helpers
impl Parser {
    pub fn new(file: String) -> Result<Self, ParseError> {
        match Lexer::new(file) {
            Ok(mut lexer) => {
                let mut tokens: Vec<Token> = vec![];
                loop {
                    if let Some(token) = lexer.get_next_token() {
                        tokens.push(token);
                        if tokens[tokens.len() - 1] == Token::EOF {
                            break;
                        }
                    } else {
                        return Err(ParseError::LexError);
                    }
                }
                return Ok(Self {
                    tokens,
                    ind: 0,
                    curr_token: None,
                    current_statement: vec![],
                    imported_items: vec![],
                    package: String::new(),
                });
            }
            Err(e) => return Err(ParseError::IoError(e)),
        }
    }

    fn peek_token_at(&self, ind: usize) -> Option<Token> {
        let real_ind: usize = ind + self.ind;
        if real_ind >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[real_ind].clone())
    }
    fn peek_next_token(&self) -> Option<Token> {
        if self.ind >= self.tokens.len() {
            return None;
        }
        Some(self.tokens[self.ind].clone())
    }

    fn get_next_token(&mut self) -> Option<Token> {
        self.curr_token = self.peek_next_token();
        self.ind += 1;
        return self.curr_token.clone();
    }
}

impl Parser {
    fn package(&mut self) -> Result<(), ParseError> {
        match self.peek_next_token() {
            Some(Token::Keyword(s)) if s == "package" => {}
            Some(_) => {
                self.package = "default".to_string();
                return Ok(());
            }
            None => return Err(ParseError::IndexOutOfBounds),
        }

        self.get_next_token(); // consume "package"
        match self.get_next_token() {
            Some(Token::Identifier(name)) => {
                self.package.push_str(name.as_str());
            }
            Some(Token::EOF) => return Err(ParseError::UnexpectedEOF),
            Some(tok) => {
                return Err(ParseError::UnexpectedToken {
                    expected: "IDENTIFIER".to_string(),
                    got: vec![tok],
                });
            }
            None => return Err(ParseError::IndexOutOfBounds),
        }

        Ok(())
    }
}

/// util
impl Parser {
    fn is_modifier(token: Token) -> bool {
        match token {
            Token::Keyword(k)
                if k == "public"
                    || k == "private"
                    || k == "protected"
                    || k == "abstract"
                    || k == "static"
                    || k == "final"
                    || k == "strictfp" =>
            {
                true
            }
            _ => false,
        }
    }

    fn is_access_modifier(token: Token) -> bool {
        match token {
            Token::Keyword(k) if k == "public" || k == "private" || k == "protected" => true,
            _ => false,
        }
    }
}
