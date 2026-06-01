use std::{fs, io};

/// Used for params, braces, and brackets
/// to identify if one is open or close
pub enum OpenClose {
    Open,
    Close,
}

/// Tokens for lexing
#[derive(Debug)]
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
    /// Binary ops include operations where there are 2 inputs, such as
    /// `+`, `-`, `*`, `/`, `==`, etc.
    BinaryOp(String),
    /// Unary ops include operations where is 1 input, such as
    /// `--`, `++`, `!`, etc.
    UnaryOp(String),

    /// Denotes file ended
    EOF,
}
pub struct Lexer {
    pub file: Vec<char>,
    pub curr_ind: usize,
    pub curr_char: Option<char>,
}

impl Lexer {
    /// Create a new Lexer
    pub fn new(file: String) -> io::Result<Self> {
        Ok(Self {
            file: fs::read_to_string(file)?.chars().collect(),
            curr_ind: 0,
            curr_char: Some('\0'),
        })
    }

    /// Get the next char of the file, then move the file ptr up
    fn get_next_char(&mut self) -> Option<char> {
        self.curr_char = match self.get_char_at(self.curr_ind) {
            Some(val) => {
                self.curr_ind += 1;
                Some(val)
            }
            None => None,
        };
        self.curr_char
    }

    // Get the next char of the file
    fn peek_next_char(&self) -> Option<char> {
        self.get_char_at(self.curr_ind)
    }

    fn peek_next_2(&self) -> (Option<char>, Option<char>) {
        (self.peek_next_char(), self.get_char_at(self.curr_ind + 1))
    }

    fn peek_next_3(&self) -> (Option<char>, Option<char>, Option<char>) {
        (
            self.peek_next_char(),
            self.get_char_at(self.curr_ind + 1),
            self.get_char_at(self.curr_ind + 2),
        )
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
                // Whitespace stuffs -> skip
                Some(' ') | Some('\t') | Some('\n') | Some('\r') => continue,
                Some(';') => return Some(Token::Semicolon),
                Some('=') => {
                    // There are 2 cases:
                    // - =, assignment.
                    // - ==, binary op.
                    // - Anything else is syntax error
                    match self.peek_next_char() {
                        Some('=') => {
                            self.get_next_char();
                            return Some(Token::BinaryOp("==".to_string()));
                        }
                        _ => return Some(Token::Assignment),
                    }
                }
                Some('/') => {
                    // Here, we are either going to see:
                    // - /=, which is assignment,
                    // - //, which is inline comment,
                    // - /*, which signifies block comment.
                    // - Anything else is syntax error.
                    match self.get_char_at(self.curr_ind) {
                        Some('*') => {
                            self.pass_block_comment();
                        }
                        Some('/') => {
                            self.pass_inline_comment();
                        }
                        Some('=') => {
                            // Assignment case
                            self.get_next_char();
                            return Some(Token::Assignment);
                        }
                        _ => return None,
                    }
                }
                Some('a'..='z') | Some('A'..='Z') => {
                    // This is either Identifier or Keyword, depends.
                    let s: String = self.get_alphabet_chain()?;
                    match s.as_str() {
                        "abstract" | "class" | "const" | "default" | "enum" | "extends"
                        | "final" | "implements" | "import" | "interface" | "package"
                        | "public" | "private" | "static" | "synchronized" | "transient"
                        | "true" | "false" => {
                            return Some(Token::Keyword(s));
                        }
                        _ => return Some(Token::Identifier(s)),
                    }
                }
                // Unexpected token, return None
                _ => return None,
            }
        }
    }

    // iterate until new line
    fn pass_inline_comment(&mut self) {
        let mut curr_char = self.get_next_char();
        while curr_char != None && curr_char != Some('\n') {
            curr_char = self.get_next_char();
        }
    }

    // iterate until see */ or end of file
    fn pass_block_comment(&mut self) {
        loop {
            let c1 = self.get_next_char();
            if c1 == None {
                return;
            }
            if c1 == Some('*') {
                let c2 = self.get_next_char();
                if c2 == Some('/') {
                    break;
                }
                if c2 == None {
                    break;
                }
            }
        }
    }

    // iterate to get all a-zA-Z0-9.
    fn get_alphabet_chain(&mut self) -> Option<String> {
        let mut s: String = "".to_string();
        s.push(self.curr_char?);
        loop {
            match self.peek_next_char() {
                Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') => {
                    s.push(self.get_next_char()?);
                }
                _ => return Some(s),
            }
        }
    }
}

// -----------------------------------
// -------------- TESTS --------------
// -----------------------------------

#[test]
fn test_inline_comment() {
    let mut lexer: Lexer = Lexer {
        curr_ind: 0,
        file: "// Ola!\nHello World!".chars().collect(),
        curr_char: Some('\0'),
    };
    lexer.pass_inline_comment();
    assert!(lexer.peek_next_char() == Some('H'));
}

#[test]
fn test_block_comment() {
    let mut lexer: Lexer = Lexer {
        curr_ind: 0,
        file: "askdjf\nsladfnasdf\t * * * */Hello world\n"
            .chars()
            .collect(),
        curr_char: Some('\0'),
    };
    lexer.pass_block_comment();
    assert!(lexer.peek_next_char() == Some('H'));
}

#[test]
fn test_lex_simple_string() {
    let s: String = "boolean meta = true;".to_string();
    println!("Test string: {s}");
    let mut lexer: Lexer = Lexer {
        curr_ind: 0,
        file: s.chars().collect(),
        curr_char: Some('\0'),
    };

    for _ in 0..5 {
        println!("{:?}", lexer.get_next_token());
    }
}
