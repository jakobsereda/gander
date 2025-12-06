// NOTE:
// I have cut the 'identifier' definition from the
// EBNF completely out for now since it is just a 
// wrapper around a String. I doubt it will be needed
// later but this is just here so I remember why I 
// chose to do such a thing...

#[derive(Debug)]
pub struct Program {
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum Item {
    Definition(Definition),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Definition {
    Enum(EnumDef),
    Struct(StructDef),
    Variable(VariableDef),
    Function(FunctionDef),
}

#[derive(Debug)]
pub struct EnumDef {
    pub ident: String,
    pub variants: Vec<String>,
}

#[derive(Debug)]
pub struct StructDef {
    pub ident: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug)]
pub struct StructField {
    pub ident: String,
    pub ftype: Type,
}

#[derive(Debug)]
pub struct VariableDef {
    pub ident: String,
    pub vtype: Option<Type>,
    pub value: Expression, 
}

#[derive(Debug)]
pub struct FunctionDef {
    pub sig: Option<FunctionTypeDecl>,
    pub head: FunctionHeader,
}

#[derive(Debug)]
pub struct FunctionTypeDecl {
    pub params: Vec<Type>,
    pub ret: Type,
}

#[derive(Debug)]
pub struct FunctionHeader {
    pub ident: String,
    pub params: Vec<String>,
    pub body: Vec<Expression>,
}

#[derive(Debug)]
pub enum Type {
    Primitive(PrimitiveType),
    Identifier(String),
    Function(FunctionType),
}

#[derive(Debug)]
pub enum PrimitiveType {
    Int,
    Bool,
    Float,
    String,
}

#[derive(Debug)]
pub struct FunctionType {
    pub params: Vec<Type>,
    pub ret: Box<Type>,
}

#[derive(Debug)]
pub enum Expression {
    Comparison {
        lhs: Box<Expression>,
        op: Option<ComparisonOp>,
        rhs: Option<Box<Expression>>,
    },

    Additive {
        lhs: Box<Expression>,
        op: Option<AdditiveOp>,
        rhs: Option<Box<Expression>>,
    },

    Multiplicative {
        lhs: Box<Expression>,
        op: Option<MultiplicativeOp>,
        rhs: Option<Box<Expression>>,
    },

    Unary {
        op: Option<UnaryOp>,
        body: Box<Expression>,
    },

    Primary(PrimaryExpr),
}

#[derive(Debug)]
pub enum ComparisonOp {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterEquals,
    LessEquals,
}

#[derive(Debug)]
pub enum AdditiveOp {
    Plus,
    Minus,
}

#[derive(Debug)]
pub enum MultiplicativeOp {
    Divide,
    Multiply,
    Modulo,
}

#[derive(Debug)]
pub enum UnaryOp {
    Negative,
}

#[derive(Debug)]
pub enum PrimaryExpr {
    StructAccess(StructAccess),
    EnumAccess(EnumAccess),
    FunctionCall(FunctionCall),
    Identifier(String),
    Literal(Literal),
    ParenExpr(Box<Expression>),
}

#[derive(Debug)]
pub struct StructAccess {
    pub parent: Box<Expression>,
    pub field: String,
}

#[derive(Debug)]
pub struct EnumAccess {
    pub parent: String,
    pub variant: String,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub ident: String,
    pub args: Vec<Expression>,
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}
