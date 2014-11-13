
use ast::preprocessor;
use ast::Statement;



/// A preprocessor #define
pub enum Define
{
    DefineFunction(preprocessor::Function),
    DefineConstant(preprocessor::Constant),
}

impl Statement for Define { }
