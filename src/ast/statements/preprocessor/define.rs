
use ast;
use ast::preprocessor;



/// A preprocessor #define
#[deriving(Show)]
pub enum Define
{
    /// A statement defining a preprocessor function.
    DefineFunction(preprocessor::Function),
    
    /// A statement defining a preprocessor constant.
    DefineConstant(preprocessor::Constant),
}

impl ast::Statement for Define
{
    fn to_stmt(self) -> ast::Stmt
    {
        ast::StmtDefine(self)
    }
}
