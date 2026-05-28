use std::{fs, io};

/// Used for params, braces, and brackets
/// to identify if one is open or close
pub enum OpenClose {
    Open,
    Close,
}

/// Token for lexing
pub enum Token {
    /// Names (for methods, variables, functions)
    Identifier(String),
    /// Keywords (e.g. class, interface, implement, new)
    Keyword(String),
    /// Unary ops
    UnaryOp(String),
    /// Binary ops
    BinaryOp(String),
    /// Assignment ops
    AssignmentOp(String),

    /// Parenthesis ()
    Parenthesis(OpenClose),
    /// Bracket []
    Bracket(OpenClose),
    /// Brace {}
    Brace(OpenClose),
    /// Less than op '<'
    ///
    /// seperate this from binary op to implement generics more easily
    LessThan,
    /// More than op '>'
    ///
    /// seperate this from binary op to implement generics more easily
    MoreThan,
    /// For function calls '.'
    Dot,
    /// Colon ':'
    Colon,
    /// Method reference '::'
    DoubleColon,
    /// Question mark
    QuestionMark,
    /// Any whitespace
    Whitespace,
    /// All comments, // and /* */
    Comment,
    /// Signals that the file has ended
    EOF,
}
pub struct Lexer {
    file: Vec<char>,
    pub curr_ind: usize,
    pub current_lexeme: Vec<char>,
    pub current_char: char,
}

impl Lexer {
    /// A lexeme can only have length maximum at 64
    const MAX_LEXEME_LEN: usize = 64;

    /// Create a new Lexer
    fn new(file: String) -> io::Result<Self> {
        Ok(Self {
            file: fs::read_to_string(file)?.chars().collect(),
            curr_ind: 0,
            current_lexeme: vec![],
            current_char: '\0',
        })
    }

    /// Get the next byte of the file
    fn get_next_char(&mut self) -> Option<char> {
        match self.get_char_at(self.curr_ind) {
            Some(val) => {
                self.curr_ind += 1;
                Some(val)
            }
            None => None,
        }
    }

    /// Get byte at some index in the file
    fn get_char_at(&self, ind: usize) -> Option<char> {
        if ind >= self.file.len() {
            return None;
        }

        Some(self.file[ind])
    }

    /// Get the next token
    pub fn get_next_token(&mut self) -> Option<Token> {
        loop {
            if self.curr_ind >= self.file.len() {
                return Some(Token::EOF);
            }
            let next_char: char = self.get_next_char()?;
            match next_char {
                // Do not care about whitespace
                '\t' | '\n' | '\r' | ' ' => continue,

                // Trivial stuffs
                '[' => return Some(Token::Bracket(OpenClose::Open)),
                ']' => return Some(Token::Bracket(OpenClose::Close)),
                '{' => return Some(Token::Brace(OpenClose::Open)),
                '}' => return Some(Token::Brace(OpenClose::Close)),
                '(' => return Some(Token::Parenthesis(OpenClose::Open)),
                ')' => return Some(Token::Parenthesis(OpenClose::Close)),
                '.' => return Some(Token::Dot),
                '?' => return Some(Token::QuestionMark),

                // Other tokens
                '+' => {
                    let second_next_char: Option<char> = self.get_char_at(self.curr_ind);
                    match second_next_char {
                        Some(byte) => {
                            if byte == '+' {
                                // consume the next token
                                self.get_next_char();
                                return Some(Token::UnaryOp("++".to_string()));
                            }
                            if byte == '=' {
                                self.get_next_char();
                                return Some(Token::AssignmentOp("+=".to_string()));
                            }
                            return Some(Token::BinaryOp("+".to_string()));
                        }
                        None => return Some(Token::BinaryOp("+".to_string())),
                    }
                }
                '-' => {
                    let second_next_char: Option<char> = self.get_char_at(self.curr_ind);
                    match second_next_char {
                        Some(byte) => {
                            if byte == '-' {
                                // consume the next token
                                self.get_next_char();
                                return Some(Token::UnaryOp("--".to_string()));
                            }
                            if byte == '=' {
                                self.get_next_char();
                                return Some(Token::AssignmentOp("-=".to_string()));
                            }
                            return Some(Token::BinaryOp("-".to_string()));
                        }
                        None => return Some(Token::BinaryOp("-".to_string())),
                    }
                }
                '*' => {
                    let second_next_char: Option<char> = self.get_char_at(self.curr_ind);
                    match second_next_char {
                        Some(byte) => {
                            if byte == '=' {
                                self.get_next_char();
                                return Some(Token::AssignmentOp("*=".to_string()));
                            }
                            return Some(Token::BinaryOp("*".to_string()));
                        }
                        None => return Some(Token::BinaryOp("*".to_string())),
                    }
                }
                '/' => {
                    let second_next_char: Option<char> = self.get_char_at(self.curr_ind);
                    match second_next_char {
                        Some(byte) => {
                            if byte == '=' {
                                self.get_next_char();
                                return Some(Token::AssignmentOp("/=".to_string()));
                            } else if byte == '/' {
                                // Inline comment. Ignore the whole line.
                                self.get_next_char();
                                loop {
                                    match self.get_char_at(self.curr_ind) {
                                        Some(c) => {
                                            if c == '\n' {
                                                // so we break out of this loop,
                                                // then break out of the if,
                                                // then we break out of the match,
                                                // thus go to the next big loop's iteration
                                                self.get_next_char();
                                                break;
                                            }
                                            self.get_next_char();
                                        }
                                        None => {
                                            return Some(Token::EOF);
                                        }
                                    }
                                }
                            } else if byte == '*' {
                                // comment block. Read until "*/" then run the next iteration
                                self.get_next_char();
                                loop {
                                    match self.get_next_char() {
                                        Some('*') => match self.get_char_at(self.curr_ind) {
                                            Some('/') => {
                                                self.get_next_char();
                                                break;
                                            }
                                            None => {
                                                return Some(Token::EOF);
                                            }
                                            _ => {}
                                        },
                                        None => {
                                            return Some(Token::EOF);
                                        }
                                        _ => {}
                                    }
                                }
                            } else {
                                return Some(Token::BinaryOp("/".to_string()));
                            }
                        }
                        None => return Some(Token::BinaryOp("/".to_string())),
                    }
                }
                '%' => {
                    let second_next_char: Option<char> = self.get_char_at(self.curr_ind);
                    match second_next_char {
                        Some(byte) => {
                            if byte == '=' {
                                self.get_next_char();
                                return Some(Token::AssignmentOp("%=".to_string()));
                            }
                            return Some(Token::BinaryOp("%".to_string()));
                        }
                        None => return Some(Token::BinaryOp("%".to_string())),
                    }
                }
                ':' => {
                    let second_next_char: Option<char> = self.get_char_at(self.curr_ind);
                    match second_next_char {
                        Some(byte) => {
                            if byte as char == ':' {
                                // consume the next token
                                self.get_next_char();
                                return Some(Token::DoubleColon);
                            }
                            return Some(Token::Colon);
                        }
                        None => return Some(Token::Colon),
                    }
                }
                _ => {}
            }
        }
    }
}
