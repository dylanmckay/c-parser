
use std;

pub mod statements;
pub mod expressions;
pub mod preprocessor;




/// A statement.
pub trait Statement : std::fmt::Show
{
    fn to_stmt(self) -> Stmt;
}

#[deriving(Show)]
pub enum Stmt
{
    StmtDefine(statements::Define),
}

pub trait Expression : std::fmt::Show
{
    fn to_expr(self) -> Expr;
}

/// An expression.
#[deriving(Show)]
pub enum Expr
{
    ExprIdentifier(expressions::Identifier),
}

/// An abstract syntax tree.
#[deriving(Show)]
pub struct Ast
{
    pub nodes: Vec<Stmt>,
}

impl Ast
{
    pub fn new() -> Ast
    {
        Ast {
            nodes: Vec::new(),
        }
    }
}
