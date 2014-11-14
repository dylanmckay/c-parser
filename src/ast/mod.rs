
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
    
    // temporary. I put this here because we pattern match against an Expr,
    // and I wanted to ignore all other cases. This is an error if there are no other
    // cases, so please delete this once there is more than one Expr variant.
    ExprTmp,
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
