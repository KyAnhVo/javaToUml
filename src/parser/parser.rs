use crate::parser::{lexer::Lexer, token::Token};

#[derive(Debug)]
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
                    imported_items: vec![],
                    package: String::new(),
                });
            }
            Err(e) => return Err(ParseError::IoError(e)),
        }
    }

    /// Parse the file
    pub fn parse(&mut self) -> Result<(), ParseError> {
        self.program()
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

// Actual parsing with the grammar
impl Parser {
    fn program(&mut self) -> Result<(), ParseError> {
        if let Err(e) = self.package() {
            return Err(e);
        }
        if let Err(e) = self.import() {
            return Err(e);
        }
        Ok(())
    }
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

        loop {
            match (self.peek_token_at(0), self.peek_token_at(1)) {
                (Some(Token::Dot), Some(Token::Identifier(s))) => {
                    self.get_next_token();
                    self.get_next_token();
                    self.package.push_str(format!(".{}", s).as_str());
                }
                (Some(Token::Semicolon), _) => {
                    self.get_next_token();
                    break;
                }
                (Some(Token::EOF), _) => {
                    return Err(ParseError::UnexpectedEOF);
                }
                (None, _) | (_, None) => return Err(ParseError::IndexOutOfBounds),
                (Some(tok1), Some(tok2)) => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "DOT IDENTIFIER or SEMICOLON".to_string(),
                        got: vec![tok1, tok2],
                    });
                }
            }
        }

        Ok(())
    }

    fn import(&mut self) -> Result<(), ParseError> {
        loop {
            match self.peek_next_token() {
                Some(Token::Keyword(s)) if s == "import" => {}
                _ => break,
            }

            let mut is_static = false;
            let mut import_stuff = String::new();
            self.get_next_token();
            match self.get_next_token() {
                Some(Token::Identifier(s)) => {
                    import_stuff.push_str(s.as_str());
                }
                Some(Token::Keyword(s)) if s == "static" => {
                    is_static = true;
                    match self.get_next_token() {
                        Some(Token::Identifier(s)) => {
                            import_stuff.push_str(s.as_str());
                        }
                        Some(token) => {
                            return Err(ParseError::UnexpectedToken {
                                expected: "IDENTIFIER".to_string(),
                                got: vec![token],
                            });
                        }
                        None => return Err(ParseError::IndexOutOfBounds),
                    }
                }
                Some(token) => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "[static] IDENTIFIER".to_string(),
                        got: vec![token],
                    });
                }
                None => return Err(ParseError::IndexOutOfBounds),
            }
            loop {
                match (self.get_next_token(), self.peek_next_token()) {
                    (Some(Token::Dot), Some(Token::Identifier(s))) => {
                        import_stuff.push_str(format!(".{}", s).as_str());
                        self.get_next_token();
                    }
                    (Some(Token::Dot), Some(Token::Op(s))) if s == "*" => {
                        import_stuff.push_str(".*");
                        self.get_next_token();
                        match self.get_next_token() {
                            Some(Token::Semicolon) => {
                                break;
                            }
                            Some(token) => {
                                return Err(ParseError::UnexpectedToken {
                                    expected: "SEMICOLON".to_string(),
                                    got: vec![token],
                                });
                            }
                            None => return Err(ParseError::IndexOutOfBounds),
                        }
                    }
                    (Some(Token::Semicolon), _) => break,
                    (None, _) | (_, None) => return Err(ParseError::IndexOutOfBounds),
                    (Some(token1), Some(token2)) => {
                        return Err(ParseError::UnexpectedToken {
                            expected: "SEMICOLON | DOT * | DOT IDENTIFIER".to_string(),
                            got: vec![token1, token2],
                        });
                    }
                }
            }

            if !is_static {
                self.imported_items.push(import_stuff);
            }
        }

        Ok(())
    }

    fn type_decl(&mut self) -> Result<(), ParseError> {
        // loop over modifiers, store them
        let mut modifiers: Vec<Token> = vec![];
        loop {
            match self.peek_next_token() {
                Some(token) => {
                    if Self::is_modifier(token.clone()) {
                        self.get_next_token();
                        modifiers.push(token.clone());
                    } else {
                        break;
                    }
                }
                None => return Err(ParseError::IndexOutOfBounds),
            }
        }

        // TODO: Implement each of the type declarations

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

//------------------------------------------------------------------
//----------------------- TEST -------------------------------------
//------------------------------------------------------------------

#[cfg(test)]
mod test {
    use crate::parser::{lexer::Lexer, parser::Parser, token::Token};

    fn create_parser_from_valid_string(s: &str) -> Parser {
        let mut lexer: Lexer = Lexer {
            file: s.chars().collect(),
            curr_ind: 0,
            curr_char: Some('\0'),
        };
        let mut tokens: Vec<Token> = vec![];
        loop {
            let token = lexer.get_next_token().unwrap();
            tokens.push(token);
            if tokens[tokens.len() - 1] == Token::EOF {
                break;
            }
        }
        Parser {
            tokens,
            ind: 0,
            curr_token: None,
            imported_items: vec![],
            package: String::new(),
        }
    }

    #[test]
    fn test_package() {
        let mut parser = create_parser_from_valid_string("public class A {}");
        parser.package().unwrap();
        assert_eq!(parser.package, "default");

        parser = create_parser_from_valid_string("package com;");
        parser.package().unwrap();
        assert_eq!(parser.package, "com");

        parser = create_parser_from_valid_string("package com.util;");
        parser.package().unwrap();
        assert_eq!(parser.package, "com.util");
    }

    #[test]
    fn test_import() {
        let mut parser = create_parser_from_valid_string(
            "import com.example.Vector; import com.example.*; import com; import static com.example.Vector;",
        );
        parser.import().unwrap();
        assert_eq!(
            parser.imported_items,
            vec!["com.example.Vector", "com.example.*", "com"]
        );
    }
}
