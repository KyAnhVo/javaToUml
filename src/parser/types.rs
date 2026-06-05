use crate::parser::token::Token;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken { expected: String, got: Vec<Token> },
    MultipleAccessModifiers,
    UnexpectedEOF,
    LexError,
    IndexOutOfBounds,
    IoError(std::io::Error),
}

#[derive(Debug, PartialEq)]
pub enum TypeKind {
    Class {
        extends: Option<String>,
        implements: Vec<String>,
    },
    Interface {
        extends: Vec<String>,
    },
    Annotation,
}

#[derive(Debug, PartialEq)]
pub enum AccessModifier {
    Public,
    Private,
    Protected,
    PackagePrivate,
}

#[derive(Debug, PartialEq)]
pub struct Type {
    pub name: String,
    pub kind: TypeKind,
    pub access_modifier: AccessModifier,
}

#[derive(Debug, PartialEq)]
pub enum TypeName {
    Void,
    RefType(RefType),
}

#[derive(Debug, PartialEq)]
pub struct RefType {
    pub name: String,
    pub type_args: Vec<RefType>,
    pub array_depth: usize,
}
