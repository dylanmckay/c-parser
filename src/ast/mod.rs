
pub mod statements;
pub mod preprocessor;

/// An identifier.
pub struct Identifier
{
    pub name: String,
}

impl Identifier
{
    /// Creates a new identifier from a name.
    /// Returns None if name is an invalid identifier.
    pub fn from_name(name: String) -> Option<Identifier>
    {
        // TODO: validate identifier.
        Some(Identifier {
            name: name,
        })
    }
}

/// A statement.
pub trait Statement { }

pub enum Stmt
{
    StmtDefine(statements::Define),
}

pub trait Expression { }

pub enum Expr
{

}

/// An abstract syntax tree.
pub struct Ast
{
    pub nodes: Vec<Stmt>,
}
