
use ast;

use ast::Expr;
use ast::expressions::Identifier;



/// A preprocessor #define
#[deriving(Show)]
pub enum Kind
{
    /// A statement defining a preprocessor function.
    KindFunction(self::Function),
    
    /// A statement defining a preprocessor constant.
    KindConstant(self::Constant),
}

#[deriving(Show)]
pub struct Define
{
    name: Identifier,
    kind: Kind,
}

impl Define
{
    pub fn function(name: Identifier, params: Vec<Identifier>, expr: Option<Expr>) -> Define
    {
        Define {
            name: name,
            
            kind: KindFunction(Function {
                params: params,
                expr: expr,
            }),
        }
    }
    
    pub fn constant(name: Identifier, expr: Option<Expr>) -> Define
    {
        Define {
            name: name,
            
            kind: KindConstant(Constant {
                expr: expr,
            })
        }
    }
}

impl ast::Statement for Define
{
    fn to_stmt(self) -> ast::Stmt
    {
        ast::StmtDefine(self)
    }
}

/// A preprocessor function. For example:
/// `#define _SFR_IO8(addr) (addr+12)`
/// `#define do_nothing(a)`
#[deriving(Show)]
pub struct Function
{
    pub params: Vec<Identifier>,
    pub expr: Option<Expr>
}

/// A `#define ABCD` or `#define ABCD 1`
#[deriving(Show)]
pub struct Constant
{
    pub expr: Option<Expr>,
}
