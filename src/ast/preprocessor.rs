
use ast::Expr;
use ast::expressions::Identifier;

/// A preprocessor function. For example:
/// `#define _SFR_IO8(addr) (addr+12)`
/// `#define do_nothing(a)`
#[deriving(Show)]
pub struct Function
{
    pub name: Identifier,
    pub params: Vec<Identifier>,
    pub expr: Option<Expr>
}

/// A `#define ABCD` or `#define ABCD 1`
#[deriving(Show)]
pub struct Constant
{
    pub name: Identifier,
    pub expr: Option<Expr>,
}
