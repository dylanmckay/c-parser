
use ast;

/// Specifies the kind of a comment.
#[deriving(Show)]
pub enum Kind
{
    KindBlock,
    KindLine,
}

/// A comment.
#[deriving(Show)]
pub struct Comment(pub Kind, pub String);

impl ast::Statement for Comment
{
    fn to_stmt(self) -> ast::Stmt
    {
        ast::Stmt::Comment(self)
    }
}
