
use Identifier;
use token::Token;

/// Specifies the kind of a define.
#[deriving(Show)]
pub enum Kind
{
    Constant,
    Function {
        params: Vec<Identifier>,
    },
}

/// A preprocessor `#define~ block.
#[deriving(Show)]
pub struct Define
{
    pub name: Identifier,
    pub body: Option<Vec<Token>>,
    
    pub kind: Kind,
}

impl Define
{
    pub fn constant(name: Identifier, body: Option<Vec<Token>>) -> Define
    {
        Define {
            name: name,
            body: body,
            
            kind: Kind::Constant,
        }
    }
    
    pub fn function(name: Identifier, params: Vec<Identifier>, body: Option<Vec<Token>>) -> Define
    {
        Define {
            name: name,
            body: body,
            
            kind: Kind::Function {
                params: params,
            },
        }
    }
}
