
pub mod statements;
pub mod preprocessor;

/// Checks whether a character is a valid first character for an identifier.
pub fn is_valid_first_identifier_char(c: char) -> bool
{
    match c {
        '_' => true,
        _ => return c.is_alphabetic(),
    }
}

/// Checks whether a character is allowed to exist inside an identifier (not including first character).
pub fn is_valid_identifier_char(c: char) -> bool
{
    match c {
        '_' => true,
        _ => return c.is_alphanumeric(),
    }
}

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
