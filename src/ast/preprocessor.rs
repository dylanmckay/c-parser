
use ast::Expr;
use ast::expressions::Identifier;

/// A #define func(a,b,c)
#[deriving(Show)]
pub struct Function
{
    pub name: Identifier,
    pub args: Vec<Identifier>,
    pub expr: Option<Expr>
}

/// A `#define ABCD` or `#define ABCD 1`
#[deriving(Show)]
pub struct Constant
{
    pub name: Identifier,
    pub expr: Option<Expr>,
}
