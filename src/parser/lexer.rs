use std::{fs, io};

/// Used for params, braces, and brackets
/// to identify if one is open or close
pub enum OpenClose {
    Open,
    Close,
}

/// Tokens for lexing
pub enum Token {
    /// Left brace for starting a block
    LeftBrace,
    /// Right brace for ending a block
    RightBrace,
    /// LT used in starting a generic
    LessThan,
    /// GT used in ending a generic
    GreaterThan,
    /// Primitive data types, consists of
    /// ```
    /// byte, short, int, long, float, double, char, boolean
    /// ```
    Primitives(String),
    /// Identifier is class name, variable name.
    /// This will ignore primitive data types.
    Identifier(String),
    /// Keyword is like identifier but, for our case,
    /// only contains:
    /// ```
    /// abstract, class, const, default, enum,
    /// extends, final, implements, import, interface,
    /// package, public, private, static, synchronized, transient,
    /// ```
    Keyword(String),
    Semicolon,
    /// Assignment token includes `=`, `-=`, `+=`, `*=`, `/=`,
    /// `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`
    Assignment,

    /// Denotes file ended
    EOF,
}
pub struct Lexer {
    file: Vec<char>,
    pub curr_ind: usize,
    pub current_lexeme: Vec<char>,
    pub current_char: char,
}

impl Lexer {
    /// Create a new Lexer
    pub fn new(file: String) -> io::Result<Self> {
        Ok(Self {
            file: fs::read_to_string(file)?.chars().collect(),
            curr_ind: 0,
            current_lexeme: vec![],
            current_char: '\0',
        })
    }

    /// Get the next char of the file, then move the file ptr up
    fn get_next_char(&mut self) -> Option<char> {
        match self.get_char_at(self.curr_ind) {
            Some(val) => {
                self.curr_ind += 1;
                Some(val)
            }
            None => None,
        }
    }

    // Get the next char of the file
    fn peek_next_char(&self) -> Option<char> {
        self.get_char_at(self.curr_ind)
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
            match self.get_next_char() {
                None => return Some(Token::EOF),
                Some(';') => return Some(Token::Semicolon),
                Some('=') => {
                    // Since we are not
                }
                Some('/') => {
                    // Here, we are either going to see:
                    // - /=, which is assignment,
                    // - //, which is inline comment,
                    // - /*, which signifies block comment.
                    // - Anything else is syntax error.
                    // On the comment cases, we iterate until end of comment,
                    // then go on the next iteration, essentially skipping the comment block
                    match self.get_char_at(self.curr_ind) {
                        Some('*') => {
                            // Block comment case
                        }
                        Some('/') => {
                            // Inline comment case
                        }
                        Some('=') => {
                            // Assignment case
                            return;
                        }
                        _ => return None,
                    }
                }
                // Unexpected token, return None
                _ => return None,
            }
        }
    }

    fn pass_inline_comment(&mut self) {}
}
