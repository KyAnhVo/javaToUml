use crate::parser::{lexer::Lexer, token::Token, types::*};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    ind: usize,
    curr_token: Option<Token>,
    lookahead: Option<Token>,
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
                // Small trick here, so we can spam
                // self.peek_token_at(0)? ... self.peek_token_at(3)?
                tokens.push(Token::EOF);
                tokens.push(Token::EOF);
                tokens.push(Token::EOF);
                return Ok(Self {
                    tokens,
                    ind: 0,
                    curr_token: None,
                    lookahead: None,
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

    fn peek_token_at(&self, ind: usize) -> Result<Token, ParseError> {
        match self.lookahead.clone() {
            Some(token) => {
                if ind == 0 {
                    Ok(token)
                } else {
                    let real_ind: usize = ind + self.ind - 1;
                    if real_ind >= self.tokens.len() {
                        Err(ParseError::IndexOutOfBounds)
                    } else {
                        Ok(self.tokens[real_ind].clone())
                    }
                }
            }
            None => {
                let real_ind: usize = ind + self.ind;
                if real_ind >= self.tokens.len() {
                    return Err(ParseError::IndexOutOfBounds);
                }
                Ok(self.tokens[real_ind].clone())
            }
        }
    }
    fn peek_next_token(&self) -> Result<Token, ParseError> {
        if let Some(token) = self.lookahead.clone() {
            return Ok(token);
        }

        self.tokens
            .get(self.ind)
            .cloned()
            .ok_or(ParseError::IndexOutOfBounds)
    }

    fn get_next_token(&mut self) -> Result<Token, ParseError> {
        if let Some(token) = self.lookahead.take() {
            self.curr_token = Some(token.clone());
            return Ok(token);
        }
        if let Ok(token) = self.peek_next_token() {
            self.curr_token = Some(token);
        } else {
            self.curr_token = None;
        }
        self.ind += 1;
        self.curr_token.clone().ok_or(ParseError::IndexOutOfBounds)
    }
}

// Actual parsing with the grammar
impl Parser {
    /// `<program> ::= <package_decl> <import> {<type_decl>}`
    fn program(&mut self) -> Result<(), ParseError> {
        self.package()?;
        self.import()?;
        loop {
            let token = self.peek_next_token()?;
            if token == Token::EOF {
                break;
            }
            self.type_decl("".to_string())?;
        }

        Ok(())
    }

    /// `<package_decl> ::= [ "package" IDENTIFIER { "." IDENTIFIER } ";" ]`
    fn package(&mut self) -> Result<(), ParseError> {
        match self.peek_next_token()? {
            Token::Keyword(s) if s == "package" => {}
            _ => {
                self.package = "default".to_string();
                return Ok(());
            }
        }

        self.get_next_token()?; // consume "package"
        match self.get_next_token()? {
            Token::Identifier(name) => {
                self.package.push_str(name.as_str());
            }
            Token::EOF => return Err(ParseError::UnexpectedEOF),
            tok => {
                return Err(ParseError::UnexpectedToken {
                    expected: "IDENTIFIER".to_string(),
                    got: vec![(tok).clone()],
                });
            }
        }

        loop {
            match (self.peek_token_at(0)?, self.peek_token_at(1)?) {
                (Token::Dot, Token::Identifier(s)) => {
                    self.get_next_token()?;
                    self.get_next_token()?;
                    self.package.push_str(format!(".{}", s).as_str());
                }
                (Token::Semicolon, _) => {
                    self.get_next_token()?;
                    break;
                }
                (Token::EOF, _) => {
                    return Err(ParseError::UnexpectedEOF);
                }
                (tok1, tok2) => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "DOT IDENTIFIER or SEMICOLON".to_string(),
                        got: vec![tok1.clone(), tok2.clone()],
                    });
                }
            }
        }

        Ok(())
    }

    /// `<import> ::= { "import" [ "static" ] IDENTIFIER { "." IDENTIFIER } [ ".*" ] ";" }`
    fn import(&mut self) -> Result<(), ParseError> {
        loop {
            // "import"
            match self.peek_next_token()? {
                Token::Keyword(s) if s == "import" => {}
                _ => break,
            }

            // ["static"] IDENTIFIER
            let mut is_static = false;
            let mut import_stuff = String::new();
            self.get_next_token()?;
            match self.get_next_token()? {
                Token::Identifier(s) => {
                    import_stuff.push_str(s.as_str());
                }
                Token::Keyword(s) if s == "static" => {
                    is_static = true;
                    match self.get_next_token()? {
                        Token::Identifier(s) => {
                            import_stuff.push_str(s.as_str());
                        }
                        token => {
                            return Err(ParseError::UnexpectedToken {
                                expected: "IDENTIFIER".to_string(),
                                got: vec![token],
                            });
                        }
                    }
                }
                token => {
                    return Err(ParseError::UnexpectedToken {
                        expected: "[static] IDENTIFIER".to_string(),
                        got: vec![token],
                    });
                }
            }

            // {"." IDENTIFIER} [".*"] ";"
            loop {
                match (self.get_next_token()?, self.peek_next_token()) {
                    (Token::Dot, Ok(Token::Identifier(s))) => {
                        import_stuff.push_str(format!(".{}", s).as_str());
                        self.get_next_token()?;
                    }
                    (Token::Dot, Ok(Token::Op(s))) if s == "*" => {
                        import_stuff.push_str(".*");
                        self.get_next_token()?;
                        match self.get_next_token()? {
                            Token::Semicolon => {
                                break;
                            }
                            token => {
                                return Err(ParseError::UnexpectedToken {
                                    expected: "SEMICOLON".to_string(),
                                    got: vec![token],
                                });
                            }
                        }
                    }
                    (Token::Semicolon, _) => break,
                    (token1, Ok(token2)) => {
                        return Err(ParseError::UnexpectedToken {
                            expected: "SEMICOLON | DOT * | DOT IDENTIFIER".to_string(),
                            got: vec![token1, token2],
                        });
                    }
                    (_, Err(e)) => return Err(e),
                }
            }

            if !is_static {
                self.imported_items.push(import_stuff);
            }
        }

        Ok(())
    }

    /// `<type_decl> ::= <modifiers> ( <enum_decl> | <class_decl> | <interface_decl> | <annotation_decl> )`
    fn type_decl(&mut self, name_prefix: String) -> Result<(), ParseError> {
        // {<annotation>}
        loop {
            match self.peek_next_token()? {
                Token::Annotation => self.annotation()?,
                _ => break,
            };
        }

        // <modifiers>
        let (access_modifier, modifiers) = self.modifiers()?;

        // return whatever is returned in this block.
        let res: Result<(), ParseError> = match (self.get_next_token()?, self.peek_next_token()?) {
            (Token::Keyword(s), _) if s == "class" => self.class_decl(name_prefix),
            (Token::Keyword(s), _) if s == "interface" => self.interface_decl(name_prefix),
            (Token::Annotation, Token::Keyword(s)) if s == "interface" => {
                self.get_next_token()?;
                self.annotation_decl(name_prefix)
            }
            (token1, token2) => Err(ParseError::UnexpectedToken {
                expected: "class | interface | @interface".to_string(),
                got: vec![token1, token2],
            }),
        };

        res
    }

    /// `<class_decl> ::= "class" IDENTIFIER [ "extends" <ref_type> ]
    /// [ "implements" <ref_type> { "," <ref_type> } ] "{" <class_body> "}"`
    fn class_decl(&mut self, name_prefix: String) -> Result<(), ParseError> {
        // TODO: implement class_decl

        Err(ParseError::IndexOutOfBounds)
    }

    /// `<interface_decl> ::= "interface" IDENTIFIER [ "extends" <ref_type> { "," <ref_type> } ] "{" <interface_body> "}"`
    fn interface_decl(&mut self, name_prefix: String) -> Result<(), ParseError> {
        // TODO: implement interface_decl

        Err(ParseError::IndexOutOfBounds)
    }

    /// `<annotation_decl> ::= "@interface" IDENTIFIER "{" <annotation_body> "}"`
    fn annotation_decl(&mut self, name_prefix: String) -> Result<(), ParseError> {
        // TODO: implement annotation_decl

        Err(ParseError::IndexOutOfBounds)
    }

    // ----------------------------------------------------------------------------
    // ----------------- util nonterminals ----------------------------------------
    // ----------------------------------------------------------------------------

    /// `<annotation>      ::= "@" IDENTIFIER {"." IDENTIFIER} ["(" <skip_parens> ")"]`
    fn annotation(&mut self) -> Result<(), ParseError> {
        // "@" IDENTIFIER
        match (self.get_next_token()?, self.get_next_token()?) {
            (Token::Annotation, Token::Identifier(_)) => {}
            (token1, token2) => {
                return Err(ParseError::UnexpectedToken {
                    expected: "@ IDENTIFIER".to_string(),
                    got: vec![token1, token2],
                });
            }
        };

        // {"." IDENTIFIER}
        loop {
            match (self.peek_token_at(0)?, self.peek_token_at(1)?) {
                (Token::Dot, Token::Identifier(_)) => {}
                _ => break,
            };
            self.get_next_token()?;
            self.get_next_token()?;
        }

        // ["(" <skip_parens> ")"]
        if self.peek_next_token()? != Token::LeftParen {
            return Ok(());
        }
        let mut paren_count = 1;
        self.get_next_token()?;
        while paren_count > 0 {
            match self.get_next_token()? {
                Token::LeftParen => paren_count += 1,
                Token::RightParen => paren_count -= 1,
                Token::EOF => return Err(ParseError::UnexpectedEOF),
                _ => continue,
            }
        }
        Ok(())
    }

    /// `<type> ::= "void" | <ref_type>`
    fn type_class(&mut self) -> Result<TypeName, ParseError> {
        match self.get_next_token()? {
            Token::Keyword(s) if s == "void" => Ok(TypeName::Void),
            _ => Ok(TypeName::RefType(self.ref_type()?)),
        }
    }

    /// `<type_params>     ::= "<" <type_param> { "," <type_param> } ">"`
    fn type_params(&mut self) -> Result<Vec<RefType>, ParseError> {
        // TODO: implement type_params

        Err(ParseError::IndexOutOfBounds)
    }

    /// `<type_param>      ::= IDENTIFIER [ "extends" <ref_type> { "&" <ref_type> } ]`
    fn type_param(&mut self) -> Result<RefType, ParseError> {
        // TODO: implement type_param

        Err(ParseError::IndexOutOfBounds)
    }

    /// `<modifier> ::= "public" | "private" | "protected" | "abstract" | "static" | "final" | "strictfp"`
    fn modifier(&mut self) -> Result<Token, ParseError> {
        match self.get_next_token()? {
            token if Parser::is_modifier(&token) => Ok(token),
            token => Err(ParseError::UnexpectedToken {
                expected: "MODIFIER".to_string(),
                got: vec![token],
            }),
        }
    }

    /// `<modifiers> ::= { <modifiers> }`
    fn modifiers(&mut self) -> Result<(AccessModifier, Vec<Token>), ParseError> {
        let mut modifiers: Vec<Token> = vec![];
        let mut access_modifier: AccessModifier = AccessModifier::PackagePrivate;
        loop {
            match self.peek_next_token()? {
                token if Parser::is_access_modifier(&token) => {
                    if access_modifier != AccessModifier::PackagePrivate {
                        return Err(ParseError::MultipleAccessModifiers);
                    }
                    access_modifier = match &token {
                        Token::Keyword(s) if s == "public" => AccessModifier::Public,
                        Token::Keyword(s) if s == "private" => AccessModifier::Private,
                        Token::Keyword(s) if s == "protected" => AccessModifier::Protected,
                        _ => AccessModifier::PackagePrivate, // shouldn't happen, look at is_access_modifier
                    };
                    modifiers.push(token.clone());
                }
                token if Parser::is_modifier(&token) => {
                    modifiers.push(token.clone());
                }
                _ => break,
            };
            self.get_next_token()?;
        }
        return Ok((access_modifier, modifiers));
    }

    /// `<ref_type> ::= IDENTIFIER { "." IDENTIFIER } [ "<" <type_arg_lst> ">" ] { "[]" }`
    fn ref_type(&mut self) -> Result<RefType, ParseError> {
        let mut ref_type: RefType = RefType {
            name: String::new(),
            type_args: vec![],
            array_depth: 0,
        };

        // IDENTIFIER
        match self.get_next_token()? {
            Token::Identifier(s) => ref_type.name.push_str(s.as_str()),
            other => {
                return Err(ParseError::UnexpectedToken {
                    expected: "IDENTIFIER".to_string(),
                    got: vec![other],
                });
            }
        };

        // { "." IDENTIFIER }
        loop {
            match (self.peek_token_at(0)?, self.peek_token_at(1)?) {
                (Token::Dot, Token::Identifier(s)) => {
                    self.get_next_token()?;
                    self.get_next_token()?;
                    ref_type.name.push_str(format!(".{}", s).as_str());
                }
                _ => break,
            }
        }

        // ["<" <type_arg_list> ">"]
        match self.peek_next_token()? {
            Token::LessThan => {
                self.get_next_token()?;
                ref_type.type_args = self.type_arg_list()?;
                match self.get_next_token()? {
                    Token::GreaterThan => {}
                    Token::Op(s) if s == ">>" => {
                        self.lookahead = Some(Token::GreaterThan);
                    }
                    Token::Op(s) if s == ">>>" => {
                        self.lookahead = Some(Token::Op(">>".to_string()));
                    }
                    token => {
                        return Err(ParseError::UnexpectedToken {
                            expected: "> | >> | >>>".to_string(),
                            got: vec![token],
                        });
                    }
                }
            }
            _ => {}
        }

        // { "[]" }
        loop {
            match (self.peek_token_at(0)?, self.peek_token_at(1)?) {
                (Token::LeftBracket, Token::RightBracket) => {
                    self.get_next_token()?;
                    self.get_next_token()?;
                    ref_type.array_depth += 1;
                }
                _ => break,
            }
        }

        Ok(ref_type)
    }

    /// `<type_arg_list> ::= <type_arg> { "," <type_arg> }`
    fn type_arg_list(&mut self) -> Result<Vec<RefType>, ParseError> {
        let mut ref_types: Vec<RefType> = vec![];

        // <type_arg>
        if let Some(x) = self.type_arg()? {
            ref_types.push(x);
        }

        // { "," <type_arg> }
        while self.peek_next_token()? == Token::Comma {
            self.get_next_token()?;
            if let Some(x) = self.type_arg()? {
                ref_types.push(x);
            }
        }

        Ok(ref_types)
    }

    /// `<type_arg> ::= <ref_type> | "?" [ ( "extends" | "super" ) <ref_type> ]`
    fn type_arg(&mut self) -> Result<Option<RefType>, ParseError> {
        match self.peek_next_token()? {
            Token::QuestionMark => {
                self.get_next_token()?;
                match self.peek_next_token()? {
                    Token::Keyword(s) if s == "extends" || s == "super" => {
                        self.get_next_token()?;
                        Ok(Some(self.ref_type()?))
                    }
                    _ => Ok(None),
                }
            }
            Token::Identifier(_) => Ok(Some(self.ref_type()?)),
            token => Err(ParseError::UnexpectedToken {
                expected: "IDENTIFIER | ?".to_string(),
                got: vec![token],
            }),
        }
    }
}

/// util
impl Parser {
    fn is_modifier(token: &Token) -> bool {
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

    fn is_access_modifier(token: &Token) -> bool {
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
    use crate::parser::{lexer::Lexer, parser::Parser, token::Token, types::*};

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
        tokens.push(Token::EOF);
        tokens.push(Token::EOF);
        tokens.push(Token::EOF);
        tokens.push(Token::EOF);
        Parser {
            tokens,
            ind: 0,
            curr_token: None,
            lookahead: None,
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

    #[test]
    fn test_ref_type() {
        println!();

        // simple identifier
        let mut p = create_parser_from_valid_string("String");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(
            r,
            RefType {
                name: "String".to_string(),
                type_args: vec![],
                array_depth: 0
            }
        );

        // qualified name
        let mut p = create_parser_from_valid_string("java.lang.String");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(
            r,
            RefType {
                name: "java.lang.String".to_string(),
                type_args: vec![],
                array_depth: 0
            }
        );

        // simple generic
        let mut p = create_parser_from_valid_string("ArrayList<String>");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(
            r,
            RefType {
                name: "ArrayList".to_string(),
                type_args: vec![RefType {
                    name: "String".to_string(),
                    type_args: vec![],
                    array_depth: 0
                }],
                array_depth: 0,
            }
        );

        // two-arg generic
        let mut p = create_parser_from_valid_string("Map<String, Integer, Value>");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(r.name, "Map");
        assert_eq!(r.type_args.len(), 3);
        assert_eq!(r.type_args[0].name, "String");
        assert_eq!(r.type_args[1].name, "Integer");
        assert_eq!(r.type_args[2].name, "Value");

        // wildcard bare — ? contributes nothing
        let mut p = create_parser_from_valid_string("List<?>");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(r.name, "List");
        assert_eq!(r.type_args, vec![]);

        // wildcard extends
        let mut p = create_parser_from_valid_string("List<? extends Number>");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(r.type_args[0].name, "Number");

        // wildcard super
        let mut p = create_parser_from_valid_string("List<? super Integer>");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(r.type_args[0].name, "Integer");

        // array
        let mut p = create_parser_from_valid_string("String[]");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(r.name, "String");
        assert_eq!(r.array_depth, 1);

        // multi-dimensional array
        let mut p = create_parser_from_valid_string("String[][]");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(r.array_depth, 2);

        // nested generic — tests >> lookahead
        let mut p = create_parser_from_valid_string("Map<String, List<Integer>>");
        let r = p.ref_type().unwrap();
        println!("{:#?}", r);
        assert_eq!(r.name, "Map");
        assert_eq!(r.type_args[1].name, "List");
        assert_eq!(r.type_args[1].type_args[0].name, "Integer");
    }
}
