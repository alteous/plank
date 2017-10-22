use std::rc::Rc;
pub use plank_syntax::ast::{BinaryOp, FunctionType, Literal, Number, Signedness, Size, UnaryOp};
use plank_syntax::position::{Span, Spanned};
pub use ast::resolved::Symbol;


#[derive(Debug, Clone)]
pub enum Expr {
    Binary(TypedExpr, Spanned<BinaryOp>, TypedExpr),
    Unary(Spanned<UnaryOp>, TypedExpr),
    Call(TypedExpr, Vec<TypedExpr>),
    Field(TypedExpr, Spanned<usize>),
    Name(Spanned<Symbol>, Vec<Spanned<Type>>),
    Literal(Literal),
    Error,
}

#[derive(Debug, Clone)]
pub struct TypedExpr {
    pub expr: Box<Expr>,
    pub span: Span,
    pub typ: Type,
}

#[derive(Debug, Clone)]
pub enum Statement {
    If(
        TypedExpr,
        Box<Spanned<Statement>>,
        Option<Box<Spanned<Statement>>>,
    ),
    Loop(Box<Spanned<Statement>>),
    While(TypedExpr, Box<Spanned<Statement>>),
    Break,
    Continue,
    Return(TypedExpr),
    Let(Spanned<Symbol>, Spanned<Type>, TypedExpr),
    Block(Vec<Spanned<Statement>>),
    Expr(TypedExpr),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Copy, Clone)]
pub struct TypeVar(pub u32);

#[derive(Debug, Clone)]
pub enum Type {
    Var(TypeVar),
    Bool,
    Int(Signedness, Size),
    Concrete(Symbol, Rc<[Type]>),
    Pointer(Rc<Type>),
    Function(Rc<[Type]>, Rc<Type>),
    Error,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub fn_type: FunctionType,
    pub name: Symbol,
    pub type_params: Vec<Symbol>,
    pub params: Vec<Var>,
    pub return_type: Type,
    pub body: Option<Spanned<Statement>>,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Symbol,
    pub typ: Type,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub name: Symbol,
    pub type_params: Vec<Symbol>,
    pub fields: Vec<Var>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub structs: Vec<Struct>,
    pub functions: Vec<Function>,
}
