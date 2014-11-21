
use ast;

#[deriving(Show)]
pub struct Block
{
    statements: Vec<ast::Stmt>,
}

impl ast::Statement for Block
{
    fn to_stmt(self) -> ast::Stmt
    {
        ast::StmtBlock(self)
    }
}
