use crate::{
    ast::*,
    token::{TokenVariant, Symbol},
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

    pub fn parse_program(&mut self) -> Program {
        let mut items = Vec::new();

        while self.tokens.peek().is_some() {
            items.push(self.parse_item());
        }

        Program { items }
    }

    fn parse_item(&mut self) -> Item {
        // TODO: wait, can't we have the equals 2 ahead
        // as well? Like Int a = 3 for example
        if self.check(TokenVariant::Symbol(Symbol::Equals), 1) {
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

    fn parse_definition(&mut self) -> Definition {
        match self.tokens.peek_n(2) {
            TokenVariant::Symbol(Symbol::Hash) => {
                Definition::Enum(self.parse_enum_def())
            },
            TokenVariant::Symbol(Symbol::Dollar) => {
                Defintion::Struct(self.parse_struct_def())
            },
            TokenVariant::Symbol(Symbol::Colon) => {
                Defintion::Function(self.parse_function_def())
            },
            _ => {
                // At this point, I think we can assert that 
                // the 2nd token in the buffer is Symbol::Equals,
                // So it has to be variable definition
                Definition::Variable(self.parse_variable_def())
            },
        }
    }

    // TODO: better error handling in this fn
    // TODO: ebnf dosen't have commas for enums
    // and structs
    fn parse_enum_def(&mut self) -> EnumDef {
        let ident = self.tokens.eat().unwrap();

        // Consume Symbol::Equals
        self.tokens.eat();

        // Consume Symbol::Hash
        self.tokens.eat();

        let mut variants = Vec::new();
        while self.check(TokenVariant::Symbol(Symbol::Comma), 1) {
            let var_id = self.tokens.eat().unwrap();
            variants.push(var_id.lit.unwrap());

            // Consume Symbol::Comma
            self.tokens.eat();
        }

        // There will be one variant remaining after
        // we have consumed all of the ones with a 
        // trailing comma.

        // TODO: maybe we don't want it to be 
        // mandatory that there is no comma after the
        // last variant, having an optional comma
        // could be nice. But then I would have to 
        // implement the whitespace tokens ughhhhhh

        let var_id = self.tokens.eat().unwrap();
        variants.push(var_id.lit.unwrap());

        EnumDef {
            ident,
            variants
        }
    }

    // TODO: similar, error handling
    fn parse_struct_def(&mut self) -> StructDef {
        let ident = self.tokens.eat().unwrap();

        // Consume Symbol::Equals
        self.tokens.eat();

        // Consume Symbol::Dollar
        self.tokens.eat();

        let mut fields = Vec::new();
        while self.check(TokenVariant::Symbol(Symbol::Comma), 3) {
            fields.push(self.parse_struct_field());

            // Consume Symbol::Comma
            self.tokens.eat();
        }

        // Same issue as in parse_enum_def, this
        // requires no trailing comma on the last
        // field, which is less than ideal

        fields.push(self.parse_struct_field());

        StructDef {
            ident,
            fields
        }
    }

    fn parse_struct_field(&mut self) -> StructField {
        let ident = self.tokens.eat().unwrap();

        // Consume Symbol::Colon
        self.tokens.eat();

        let ftype = self.parse_type();

        StructField {
            ident,
            ftype
        }
    }

    fn parse_variable_def(&mut self) -> VariableDef {
        if self.check(TokenVariant::Symbol(Symbol::Equals), 2)
    }

    fn parse_function_def(&mut self) -> FunctionDef {
        todo!()
    } 

    fn parse_function_type_decl(&mut self) -> FunctionTypeDecl {
        todo!()
    }

    fn parse_function_header(&mut self) -> FunctionHeader {
        todo!()
    }

    fn parse_type(&mut self) -> Type {
        match self.tokens.peek() {
            TokenVariant::Identifier => {
                let id = self.tokens.eat().unwrap();
                Type::Identifier(id.lit.unwrap())
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

        match t {
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

        let params = Vec::new();
        while !self.check(TokenVariant::Symbol(Symbol::RParen), 1) {
            params.push(self.parse_type());

            // Consume Symbol::Comma
            self.tokens.eat();
        }

        // Should be one last param
        params.push(self.parse_type());

        // Consume Symbol::RParen
        self.tokens.eat();

        // Consume Symbol::Arrow
        self.tokens.eat();

        FunctionType {
            params,
            Box::new(self.parse_type())
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

    fn eat_if_match(&mut self, expected: TokenVariant) -> bool {
        if self.check(expected, 0) {
            self.tokens.eat();
            true
        } else {
            false
        }
    }
}
