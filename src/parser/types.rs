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
        extends: Option<RefType>,
        implements: Vec<RefType>,
        composites: Vec<RefType>,
    },
    Interface {
        extends: Vec<RefType>,
    },
    Annotation,
    Enum {
        implements: Vec<RefType>,
        composites: Vec<RefType>,
    },
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
    pub modifiers: Vec<String>,
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

impl RefType {
    pub(crate) fn flatten(&self) -> Vec<Self> {
        let mut v: Vec<Self> = vec![Self {
            name: self.name.clone(),
            type_args: vec![],
            array_depth: self.array_depth,
        }];
        for arr in &self.type_args {
            v.extend(arr.flatten());
        }
        v
    }
}
