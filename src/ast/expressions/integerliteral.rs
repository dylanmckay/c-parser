
use ast;

#[deriving(Clone,Show)]
pub struct IntegerLiteral(pub String);

impl ast::Expression for IntegerLiteral
{
    fn to_expr(self) -> ast::Expr
    {
        ast::ExprIntegerLiteral(self)
    }
}

