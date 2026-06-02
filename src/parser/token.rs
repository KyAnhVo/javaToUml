/// Tokens for lexing
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LessThan,
    GreaterThan,
    Dot,
    Annotation,
    Comma,
    /// Literals are `true`, `false`, numeric, string, char literals.
    Literal(String),
    /// Identifier is a catch-all for all types (class, primitive)
    /// and variable names and properties.
    Identifier(String),
    /// Keyword is like identifier but, for our case,
    /// contains:
    /// ```
    /// abstract, class, const, default, enum,
    /// extends, final, implements, import, interface,
    /// package, public, private, static, synchronized, transient,
    /// return, void, etc.
    /// ```
    Keyword(String),
    Semicolon,
    /// Assignment token includes `=`, `-=`, `+=`, `*=`, `/=`,
    /// `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`, `>>>=`
    Assignment(String),
    /// An operator can be 1-ary or 2-ary.
    Op(String),

    /// Denotes file ended
    EOF,
}
