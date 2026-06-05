use crate::parser::token::Token;
use std::{fs, io};

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
        self.curr_char = match self.peek_char_at(self.curr_ind) {
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
        self.peek_char_at(self.curr_ind)
    }

    fn peek_next_2(&self) -> (Option<char>, Option<char>) {
        (self.peek_next_char(), self.peek_char_at(self.curr_ind + 1))
    }

    fn peek_next_3(&self) -> (Option<char>, Option<char>, Option<char>) {
        (
            self.peek_next_char(),
            self.peek_char_at(self.curr_ind + 1),
            self.peek_char_at(self.curr_ind + 2),
        )
    }

    /// Get byte at some index in the file
    fn peek_char_at(&self, ind: usize) -> Option<char> {
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
                Some('{') => return Some(Token::LeftBrace),
                Some('}') => return Some(Token::RightBrace),
                Some('(') => return Some(Token::LeftParen),
                Some(')') => return Some(Token::RightParen),
                Some('[') => return Some(Token::LeftBracket),
                Some(']') => return Some(Token::RightBracket),
                Some(',') => return Some(Token::Comma),
                Some('@') => return Some(Token::Annotation),
                Some('?') => return Some(Token::QuestionMark),
                Some('.') => match self.peek_next_2() {
                    (Some('.'), Some('.')) => {
                        self.get_next_char();
                        self.get_next_char();
                        return Some(Token::Op("...".to_string()));
                    }
                    _ => return Some(Token::Dot),
                },
                Some('~') => return Some(Token::Op("~".to_string())),
                Some('>') => match self.peek_next_3() {
                    (Some('>'), Some('>'), Some('=')) => {
                        self.get_next_char();
                        self.get_next_char();
                        self.get_next_char();
                        return Some(Token::Assignment(">>>=".to_string()));
                    }
                    (Some('>'), Some('>'), _) => {
                        self.get_next_char();
                        self.get_next_char();
                        return Some(Token::Op(">>>".to_string()));
                    }
                    (Some('>'), Some('='), _) => {
                        self.get_next_char();
                        self.get_next_char();
                        return Some(Token::Assignment(">>=".to_string()));
                    }
                    (Some('>'), _, _) => {
                        self.get_next_char();
                        return Some(Token::Op(">>".to_string()));
                    }
                    (Some('='), _, _) => {
                        self.get_next_char();
                        return Some(Token::Op(">=".to_string()));
                    }
                    _ => return Some(Token::GreaterThan),
                },
                Some('<') => match self.peek_next_2() {
                    (Some('<'), Some('=')) => {
                        self.get_next_char();
                        self.get_next_char();
                        return Some(Token::Assignment("<<=".to_string()));
                    }
                    (Some('<'), _) => {
                        self.get_next_char();
                        return Some(Token::Op("<<".to_string()));
                    }
                    (Some('='), _) => {
                        self.get_next_char();
                        return Some(Token::Op("<=".to_string()));
                    }
                    _ => return Some(Token::LessThan),
                },
                Some('^') | Some('%') | Some('!') => match self.peek_next_char() {
                    Some('=') => {
                        let mut s: String = self.curr_char.unwrap().to_string();
                        s.push(self.get_next_char().unwrap());
                        return Some(Token::Assignment(s));
                    }
                    _ => return Some(Token::Op(self.curr_char?.to_string())),
                },
                Some(':') => match self.peek_next_char() {
                    Some(':') => {
                        self.get_next_char();
                        return Some(Token::Op("::".to_string()));
                    }
                    _ => return Some(Token::Op(":".to_string())),
                },
                Some('=') => match self.peek_next_char() {
                    Some('=') => {
                        self.get_next_char();
                        return Some(Token::Op("==".to_string()));
                    }
                    _ => return Some(Token::Assignment("=".to_string())),
                },
                Some('+') => match self.peek_next_char() {
                    Some('=') => {
                        self.get_next_char();
                        return Some(Token::Assignment("+=".to_string()));
                    }
                    Some('+') => {
                        self.get_next_char();
                        return Some(Token::Op("++".to_string()));
                    }
                    _ => return Some(Token::Op("+".to_string())),
                },
                Some('-') => match self.peek_next_char() {
                    Some('=') => {
                        self.get_next_char();
                        return Some(Token::Assignment("-=".to_string()));
                    }
                    Some('-') => {
                        self.get_next_char();
                        return Some(Token::Op("--".to_string()));
                    }
                    _ => return Some(Token::Op("-".to_string())),
                },
                Some('*') => match self.peek_next_char() {
                    Some('=') => {
                        self.get_next_char();
                        return Some(Token::Assignment("*=".to_string()));
                    }
                    _ => return Some(Token::Op("*".to_string())),
                },
                Some('/') => {
                    match self.peek_char_at(self.curr_ind) {
                        Some('*') => {
                            self.pass_block_comment();
                        }
                        Some('/') => {
                            self.pass_inline_comment();
                        }
                        Some('=') => {
                            // Assignment case
                            self.get_next_char();
                            return Some(Token::Assignment("/=".to_string()));
                        }
                        _ => return Some(Token::Op("/".to_string())),
                    }
                }
                Some('&') => match self.peek_next_char() {
                    Some('&') => {
                        self.get_next_char();
                        return Some(Token::Op("&&".to_string()));
                    }
                    Some('=') => {
                        self.get_next_char();
                        return Some(Token::Assignment("&=".to_string()));
                    }
                    _ => return Some(Token::Op("&".to_string())),
                },
                Some('|') => match self.peek_next_char() {
                    Some('|') => {
                        self.get_next_char();
                        return Some(Token::Op("||".to_string()));
                    }
                    Some('=') => {
                        self.get_next_char();
                        return Some(Token::Assignment("|=".to_string()));
                    }
                    _ => return Some(Token::Op("|".to_string())),
                },
                Some('\'') => return self.get_char_literal(),
                Some('\"') => return self.get_string_literal(),
                Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('$') => {
                    let s: String = self.get_alphabet_chain()?;
                    match s.as_str() {
                        "abstract" | "class" | "const" | "default" | "enum" | "extends"
                        | "final" | "implements" | "import" | "interface" | "package"
                        | "public" | "private" | "static" | "synchronized" | "transient"
                        | "return" | "void" | "if" | "else" | "while" | "for" | "new" | "this"
                        | "super" | "break" | "continue" | "case" | "try" | "catch" | "do"
                        | "finally" | "instanceof" | "native" | "protected" | "switch"
                        | "throw" | "throws" | "volatile" => {
                            return Some(Token::Keyword(s));
                        }
                        "true" | "false" | "null" => return Some(Token::Literal(s)),
                        _ => return Some(Token::Identifier(s)),
                    }
                }
                Some('0'..='9') => return Some(Token::Literal(self.get_numeric()?)),
                _ => return None,
            }
        }
    }

    /// iterate to get string literal
    fn get_string_literal(&mut self) -> Option<Token> {
        let mut s: String = "\"".to_string();
        loop {
            let c: Option<char> = self.get_next_char();
            match c {
                Some('\\') => {
                    s.push(c?);
                    s.push(self.get_next_char()?); // we skip the next char, no matter what it is
                }
                Some('\"') => {
                    s.push(c?);
                    break;
                }
                Some(_) => s.push(c?),
                None => return None, // file ends before we end the string => syntax error
            }
        }
        return Some(Token::Literal(s));
    }

    /// iterate to get the char literla
    fn get_char_literal(&mut self) -> Option<Token> {
        let mut s: String = "\'".to_string();
        match self.peek_next_2() {
            (Some('\\'), Some('u')) => {
                s.push(self.get_next_char()?);
                s.push(self.get_next_char()?);
                for _ in 0..4 {
                    let c = self.get_next_char();
                    match c {
                        Some('a'..='f') | Some('A'..='F') | Some('0'..='9') => s.push(c?),
                        _ => return None,
                    }
                }
                if let Some(c) = self.get_next_char() {
                    if c != '\'' {
                        return None;
                    }
                    s.push(c);
                } else {
                    return None;
                }
            }
            (Some('\\'), _) => {
                s.push(self.get_next_char()?);
                s.push(self.get_next_char()?);
                if self.get_next_char() != Some('\'') {
                    return None;
                }
                s.push('\'');
            }
            (Some(_), Some('\'')) => {
                s.push(self.get_next_char()?);
                s.push(self.get_next_char()?);
            }
            _ => return None,
        }
        Some(Token::Literal(s))
    }

    /// iterate until new line
    fn pass_inline_comment(&mut self) {
        let mut curr_char = self.get_next_char();
        while curr_char != None && curr_char != Some('\n') {
            curr_char = self.get_next_char();
        }
    }

    /// iterate until see */ or end of file
    fn pass_block_comment(&mut self) {
        loop {
            match self.peek_next_2() {
                (Some('*'), Some('/')) => {
                    self.get_next_char();
                    self.get_next_char();
                    return;
                }
                (None, _) | (_, None) => return,
                _ => self.get_next_char(),
            };
        }
    }

    /// iterate to get all a-zA-Z0-9.
    fn get_alphabet_chain(&mut self) -> Option<String> {
        let mut s: String = "".to_string();
        s.push(self.curr_char?);
        loop {
            match self.peek_next_char() {
                Some('a'..='z') | Some('A'..='Z') | Some('0'..='9') | Some('$') | Some('_') => {
                    s.push(self.get_next_char()?);
                }
                _ => return Some(s),
            }
        }
    }

    /// iterate to get a numeric.
    fn get_numeric(&mut self) -> Option<String> {
        let mut s = "".to_string();
        s.push(self.curr_char?);

        match self.curr_char {
            // 0 implies these:
            // - 0x...
            // - 0b(integer of form 0 or 1)
            // - 0(dot)...
            // -
            Some('0') => match self.peek_next_char() {
                Some('l') | Some('L') => {
                    s.push(self.get_next_char()?);
                    return Some(s);
                }
                Some('b') => return self.get_binary_numeric(),
                Some('x') => return self.get_hex_numeric(),
                Some('.') => return self.get_decimal_numeric(),
                _ => return Some(s),
            },
            Some('1'..='9') => return self.get_decimal_numeric(),
            _ => return Some(s),
        }
    }

    fn get_decimal_numeric(&mut self) -> Option<String> {
        let mut can_underscore = true;
        let mut s = "".to_string();
        let mut is_float = false;
        let mut has_exponent = false;
        let mut has_decimal_point = false;
        s.push(self.curr_char?);
        loop {
            match self.peek_next_3() {
                // Only allowed rules are:
                // - Digit followed by something
                // - underscore followed by digit
                (Some('0'..='9'), _, _) => {
                    s.push(self.get_next_char()?);
                    can_underscore = true;
                }
                (Some('_'), Some('0'..='9'), _) => {
                    if can_underscore {
                        s.push(self.get_next_char()?);
                        can_underscore = false;
                    } else {
                        break;
                    }
                }
                (Some('e') | Some('E'), Some('0'..='9'), _) => {
                    if has_exponent {
                        break;
                    }
                    has_exponent = true;
                    is_float = true;
                    s.push(self.get_next_char()?);
                }
                (Some('e') | Some('E'), Some('+') | Some('-'), Some('0'..='9')) => {
                    if has_exponent {
                        break;
                    }
                    has_exponent = true;
                    is_float = true;
                    s.push(self.get_next_char()?);
                    s.push(self.get_next_char()?);
                }
                (Some('.'), _, _) => {
                    if has_decimal_point {
                        break;
                    }
                    has_decimal_point = true;
                    can_underscore = true;
                    is_float = true;
                    s.push(self.get_next_char()?);
                }
                (Some('f') | Some('F'), _, _) => {
                    if is_float {
                        s.push(self.get_next_char()?);
                    }
                    break;
                }
                (Some('l') | Some('L'), _, _) => {
                    if !is_float {
                        s.push(self.get_next_char()?);
                    }
                    break;
                }
                _ => break,
            }
        }
        Some(s)
    }
    fn get_binary_numeric(&mut self) -> Option<String> {
        self.get_next_char();
        let mut s = "0b".to_string();
        let mut can_underscore = false;
        loop {
            match self.peek_next_char() {
                Some('_') => {
                    if can_underscore {
                        can_underscore = false;
                        s.push(self.get_next_char()?);
                    } else {
                        break;
                    }
                }
                Some('0') | Some('1') => {
                    can_underscore = true;
                    s.push(self.get_next_char()?);
                }
                Some('l') | Some('L') => {
                    s.push(self.get_next_char()?);
                    break;
                }
                _ => break,
            }
        }
        if s.len() == 2 {
            // only have 0b, lexical error
            return None;
        }
        Some(s)
    }

    fn get_hex_numeric(&mut self) -> Option<String> {
        self.get_next_char();
        let mut s = "0x".to_string();
        let mut can_underscore = false;
        loop {
            match self.peek_next_char() {
                Some('_') => {
                    if can_underscore {
                        can_underscore = false;
                        s.push(self.get_next_char()?);
                    } else {
                        break;
                    }
                }
                Some('a'..='f') | Some('A'..='F') | Some('0'..='9') => {
                    can_underscore = true;
                    s.push(self.get_next_char()?);
                }
                Some('l') | Some('L') => {
                    s.push(self.get_next_char()?);
                    break;
                }
                _ => break,
            }
        }

        if s.len() == 2 {
            // only have 0x, lexical error
            return None;
        }
        Some(s)
    }
}

// -----------------------------------
// -------------- TESTS --------------
// -----------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    fn test_lexer_no_error(s: String, expected_token: Vec<Token>) {
        let mut lexer: Lexer = Lexer {
            curr_ind: 0,
            file: s.chars().collect(),
            curr_char: Some('\0'),
        };
        let mut token: Option<Token> = None;
        let mut ind: usize = 0;
        while token != Some(Token::EOF) {
            token = lexer.get_next_token();
            assert!(token != None);
            assert!(ind < expected_token.len());
            assert_eq!(token, Some(expected_token[ind].clone()));
            ind += 1;
        }
    }

    #[test]
    fn test_counter_program() {
        test_lexer_no_error(
            "
            public class Counter {
                private int count;

                // constructor
                public Counter() {
                    count = 0;
                }

                // increment counter
                public void increment() {
                    count++;
                }

                // increment counter by a value
                public void add(int value) {
                    count += value;
                }

                // check if has incremented
                public boolean isPositive() {
                    return count > 0;
                }

                // getter
                public int getCount() {
                    return count;
                }
            }"
            .to_string(),
            vec![
                // public class Counter {
                Token::Keyword("public".to_string()),
                Token::Keyword("class".to_string()),
                Token::Identifier("Counter".to_string()),
                Token::LeftBrace,
                // private int count;
                Token::Keyword("private".to_string()),
                Token::Identifier("int".to_string()),
                Token::Identifier("count".to_string()),
                Token::Semicolon,
                // public Counter() {
                Token::Keyword("public".to_string()),
                Token::Identifier("Counter".to_string()),
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                // count = 0;
                Token::Identifier("count".to_string()),
                Token::Assignment("=".to_string()),
                Token::Literal("0".to_string()),
                Token::Semicolon,
                // }
                Token::RightBrace,
                // public void increment() {
                Token::Keyword("public".to_string()),
                Token::Keyword("void".to_string()),
                Token::Identifier("increment".to_string()),
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                // count++;
                Token::Identifier("count".to_string()),
                Token::Op("++".to_string()),
                Token::Semicolon,
                // }
                Token::RightBrace,
                // public void add(int value) {
                Token::Keyword("public".to_string()),
                Token::Keyword("void".to_string()),
                Token::Identifier("add".to_string()),
                Token::LeftParen,
                Token::Identifier("int".to_string()),
                Token::Identifier("value".to_string()),
                Token::RightParen,
                Token::LeftBrace,
                // count += value;
                Token::Identifier("count".to_string()),
                Token::Assignment("+=".to_string()),
                Token::Identifier("value".to_string()),
                Token::Semicolon,
                // }
                Token::RightBrace,
                // public boolean isPositive() {
                Token::Keyword("public".to_string()),
                Token::Identifier("boolean".to_string()),
                Token::Identifier("isPositive".to_string()),
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                // return count > 0;
                Token::Keyword("return".to_string()),
                Token::Identifier("count".to_string()),
                Token::GreaterThan,
                Token::Literal("0".to_string()),
                Token::Semicolon,
                // }
                Token::RightBrace,
                // public int getCount() {
                Token::Keyword("public".to_string()),
                Token::Identifier("int".to_string()),
                Token::Identifier("getCount".to_string()),
                Token::LeftParen,
                Token::RightParen,
                Token::LeftBrace,
                // return count;
                Token::Keyword("return".to_string()),
                Token::Identifier("count".to_string()),
                Token::Semicolon,
                // }
                Token::RightBrace,
                // } (class)
                Token::RightBrace,
                Token::EOF,
            ],
        );
    }
}
