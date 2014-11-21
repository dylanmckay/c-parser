
use std;

pub mod statements;
pub mod expressions;
pub mod preprocessor;




/// A statement.
pub trait Statement : std::fmt::Show
{
    /// Converts the statement into a more abstract Stmt.
    fn to_stmt(self) -> Stmt;
}

/// Abstracts over a Statement.
#[deriving(Show)]
pub enum Stmt
{
    StmtDefine(statements::preprocessor::Define),
    StmtComment(statements::Comment),
    StmtBlock(statements::Block),
}

/// An expression.
pub trait Expression : std::fmt::Show
{
    /// Converts the expression into a more abstract Expr.
    fn to_expr(self) -> Expr;
}

/// Abstracts over an Expression.
#[deriving(Show)]
pub enum Expr
{
    ExprIdentifier(expressions::Identifier),
    
    ExprIntegerLiteral(expressions::IntegerLiteral),
    
    // temporary. I put this here because we pattern match against an Expr,
    // and I wanted to ignore all other cases. This is an error if there are no other
    // cases, so please delete this once there is more than one Expr variant.
    ExprTmp,
}

/// An [abstract syntax tree](http://en.wikipedia.org/wiki/Abstract_syntax_tree).
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
