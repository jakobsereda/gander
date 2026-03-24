use crate::{
    ast::*,
    token::{
        TokenVariant,
        Symbol
    },
    tokenbuffer::TokenBuffer,
    tokenizer::Tokenizer
};

#[derive(Debug)]
struct Parser<'a> {
    tokens: TokenBuffer<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(src: &'a str) -> Self {
        Self {
            tokens: TokenBuffer::new(Tokenizer::new(src)),
        }
    }

    // EBNF:
    //     program = { item };
    //
    // Entry point for the parser
    pub fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();
        while self.tokens.peek().is_some() {
            items.push(self.parse_item());
        }

        Program { items }
    }

    // EBNF:
    //     item = definition
    //          | expression;
    // TODO: this function is ugly
    fn parse_item(&mut self) -> Item {

        // A variable definition can either have an
        // exlicit type or not. i.e.
        //     Int a = 2
        // or
        //     a = 2
        if self.check(TokenVariant::Symbol(Symbol::Equals), 1) || self.check(TokenVariant::Symbol(Symbol::Equals), 2) {
            Item::Definition(self.parse_definition())
        } else if self.check(TokenVariant::Symbol(Symbol::At), 0) {
            let sig = self.parse_function_type_decl();
            Item::Definition(Definition::Function(
                FunctionDef {
                    sig: Some(sig),
                    head: self.parse_function_header(),
                }
            ))
        } else {
            Item::Expression(self.parse_expression())
        }
    }

    // EBNF:
    //     definition = enum_def
    //                | struct_def
    //                | variable_def
    //                | function_def;
    fn parse_definition(&mut self) -> Definition {
        let t = self.tokens.peek_n(2).unwrap();
        match t.var {
            TokenVariant::Symbol(Symbol::Hash) => {
                Definition::Enum(self.parse_enum_def())
            },
            TokenVariant::Symbol(Symbol::Dollar) => {
                Definition::Struct(self.parse_struct_def())
            },
            TokenVariant::Symbol(Symbol::Colon) => {
                Definition::Function(self.parse_function_def())
            },
            _ => {
                // At this point, I think we can assert that
                // the 2nd token in the buffer is Symbol::Equals,
                // So it has to be variable definition
                Definition::Variable(self.parse_variable_def())
            },
        }
    }

    // EBNF:
    //     enum_def = identifier "=" "#" newline
    //              { identifier enum_variant newline };
    // TODO: better error handling in this fn
    // TODO: ebnf dosen't have commas for enums
    // and structs
    fn parse_enum_def(&mut self) -> EnumDef {
        let ident = self.tokens
            .eat()
            .unwrap()
            .lit
            .unwrap();

        // Consume Symbol::Equals
        self.tokens.eat();

        // Consume Symbol::Hash
        self.tokens.eat();

        let mut variants = Vec::new();
        loop {
            let var_id = self.tokens.eat().unwrap();
            variants.push(var_id.lit.unwrap());

            if self.check(TokenVariant::Symbol(Symbol::Comma), 0) {
                break;
            }

            // Consume Symbol::Comma
            self.tokens.eat();
        }

        // TODO: maybe we don't want it to be 
        // mandatory that there is no comma after the
        // last variant, having an optional comma
        // could be nice. But then I would have to 
        // implement the whitespace tokens ughhhhhh

        EnumDef {
            ident,
            variants
        }
    }

    // TODO: similar, error handling
    fn parse_struct_def(&mut self) -> StructDef {
        // TODO: this pattern (extracting a list) is used
        // a lot, maybe it could become a helper function
        let ident = self.tokens
            .eat()
            .unwrap()
            .lit
            .unwrap();

        // Consume Symbol::Equals
        self.tokens.eat();

        // Consume Symbol::Dollar
        self.tokens.eat();

        let mut fields = Vec::new();
        loop {
            fields.push(self.parse_struct_field());

            if !self.check(TokenVariant::Symbol(Symbol::Comma), 0) {
                break;
            }

            // Consume Symbol::Comma
            self.tokens.eat();
        }

        StructDef {
            ident,
            fields
        }
    }

    fn parse_struct_field(&mut self) -> StructField {
        let ident = self.tokens
            .eat()
            .unwrap()
            .lit
            .unwrap();

        // Consume Symbol::Colon
        self.tokens.eat();

        let ftype = self.parse_type();

        StructField {
            ident,
            ftype
        }
    }

    fn parse_variable_def(&mut self) -> VariableDef {
        let mut vtype: Option<Type> = None;
        if self.check(TokenVariant::Symbol(Symbol::Equals), 2) {
            vtype = Some(self.parse_type());
        }

        let ident = self.eat_and_extract_lit();

        // Consume Symbol::Equals
        self.tokens.eat();

        let value = self.parse_expression();

        VariableDef {
            vtype,
            ident,
            value
        }
    }

    fn parse_function_def(&mut self) -> FunctionDef {
        let mut sig: Option<FunctionTypeDecl> = None;
        if self.check(TokenVariant::Symbol(Symbol::At), 0) {
            sig = Some(self.parse_function_type_decl());
        }

        let head = self.parse_function_header();

        FunctionDef {
            sig,
            head
        }
    }

    fn parse_function_type_decl(&mut self) -> FunctionTypeDecl {
        // Consume Symbol::At
        self.tokens.eat();

        let mut params = Vec::new();
        loop {
            params.push(self.parse_type());

            // Consume Symbol::Comma or Symbol::Arrow
            if self.eat_and_match(TokenVariant::Symbol(Symbol::Arrow)) {
                break;
            }
        }

        let ret = self.parse_type();

        FunctionTypeDecl {
            params,
            ret
        }
    }

    fn parse_function_header(&mut self) -> FunctionHeader {
        let ident = self.eat_and_extract_lit();

        // Consume Symbol::Equals
        self.tokens.eat();

        // Consume Symbol::Colon
        self.tokens.eat();

        // Consume Symbol::LParen
        self.tokens.eat();

        let mut params = Vec::new();
        loop {
            let token = self.tokens.eat().unwrap();
            params.push(token.lit.unwrap());

            // Consume Symbol::Comma or Symbol::RParen
            if self.eat_and_match(TokenVariant::Symbol(Symbol::RParen)) {
                break;
            }
        }

        let body = Vec::new();

        FunctionHeader {
            ident,
            params,
            body
        }
    }

    fn parse_type(&mut self) -> Type {
        let t = self.tokens.peek().unwrap();

        match t.var {
            TokenVariant::Identifier => {
                let id = self.eat_and_extract_lit();
                Type::Identifier(id)
            },
            TokenVariant::Symbol(Symbol::LParen) => {
                Type::Function(self.parse_function_type())
            },
            _ => {
                Type::Primitive(self.parse_primitive_type())
            },
        }
    }

    fn parse_primitive_type(&mut self) -> PrimitiveType {
        let t = self.tokens.eat().unwrap();

        match t.var {
            TokenVariant::Symbol(Symbol::IntType) => PrimitiveType::Int,
            TokenVariant::Symbol(Symbol::BoolType) => PrimitiveType::Bool,
            TokenVariant::Symbol(Symbol::FloatType) => PrimitiveType::Float,
            TokenVariant::Symbol(Symbol::StringType) => PrimitiveType::String,
            _ => panic!("that is not a primitive type...")
        }
    }

    fn parse_function_type(&mut self) -> FunctionType {
        // Consume Symbol::LParen
        self.tokens.eat();

        let mut params = Vec::new();
        loop {
            params.push(self.parse_type());

            // Consume Symbol::Comma or Symbol::RParen
            if self.eat_and_match(TokenVariant::Symbol(Symbol::RParen)) {
                break;
            }
        }

        // Consume Symbol::Arrow
        self.tokens.eat();

        let ret = Box::new(self.parse_type());

        FunctionType {
            params,
            ret
        }
    }

    fn parse_expression(&mut self) -> Expression {
        todo!()
    }

    fn check(&mut self, expected: TokenVariant, n: usize) -> bool {
        match self.tokens.peek_n(n) {
            Some(c) => c.var == expected,
            None => false
        }
    }

    fn eat_and_match(&mut self, expected: TokenVariant) -> bool {
        let matches = self.check(expected, 0);
        self.tokens.eat();
        matches
    }

    fn eat_and_extract_lit(&mut self) -> String {
        self.tokens
            .eat()
            .expect("eat_and_extract_lit: expected Token, got None")
            .lit
            .expect("eat_and_extract_lit: should not be invoked when Token has no lit")
    }
}
