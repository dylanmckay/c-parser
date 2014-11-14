use ast;

/// An identifier.
#[deriving(Show)]
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

impl ast::Expression for Identifier
{
    fn to_expr(self) -> ast::Expr
    {
        ast::ExprIdentifier(self)
    }
}

/// Checks whether a character is a valid first character for an identifier.
pub fn is_valid_first_char(c: char) -> bool
{
    match c {
        '_' => true,
        _ => return c.is_alphabetic(),
    }
}

/// Checks whether a character is allowed to exist inside an identifier (not including first character).
pub fn is_valid_char(c: char) -> bool
{
    match c {
        '_' => true,
        _ => return c.is_alphanumeric(),
    }
}
