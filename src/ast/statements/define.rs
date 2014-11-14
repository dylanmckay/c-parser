
use ast;
use ast::preprocessor;



/// A preprocessor #define
#[deriving(Show)]
pub enum Define
{
    DefineFunction(preprocessor::Function),
    DefineConstant(preprocessor::Constant),
}

impl ast::Statement for Define
{
    fn to_stmt(self) -> ast::Stmt
    {
        ast::StmtDefine(self)
    }
}
