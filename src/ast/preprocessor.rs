
use ast::{Identifier, Expr};

/// A #define func(a,b,c)
pub struct Function
{
    pub name: Identifier,
    pub args: Vec<Identifier>,
}

/// A `#define ABCD` or `#define ABCD 1`
pub struct Constant
{
    pub name: Identifier,
    pub expr: Option<Expr>,
}
