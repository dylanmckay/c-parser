
use token;
use token::{expect,Token,Tokenizer};

use Identifier;

#[deriving(Show)]
pub enum Block
{
    Directive,
    Token(Token),
}

pub struct Preprocessor<I: Iterator<char>>
{
    it: Tokenizer<I>,
}

impl<I: Iterator<char>> Preprocessor<I>
{
    pub fn new(it: Tokenizer<I>) -> Preprocessor<I>
    {
        Preprocessor {
            it: it,
        }
    }
    
    fn preprocess_directive(&mut self) -> Result<Block,String>
    {
        expect::assert_token(self.it.next(), &Token::hash());
        
        match try!(expect::kind(self.it.next(), token::Kind::Word)) {
            Token(token::Kind::Word, ref word) => match word.as_slice() {
            
                "define" => self.preprocess_define(),
                d => { return Err(format!("unknown directive: {}", d)); },
            
            },
            _ => unreachable!(),
        }
    }
    
    fn preprocess_define(&mut self) -> Result<Block,String>
    {
        let name_str = try!(expect::kind(self.it.next(), token::Kind::Word)).move_value();
        
        let name = match Identifier::from_name(name_str) {
            Some(name) => name,
            None => { return Err("invalid identifier".to_string()); },
        };
        
        match try!(expect::something(self.it.peek())) {
            Token(token::Kind::Symbol, ref sym) if sym.as_slice() == "(" => {
                unimplemented!();
            },
            _ => {
                self.preprocess_define_constant(name)
            }
        }
    }
    
    fn preprocess_define_constant(&mut self, name: Identifier) -> Result<Block,String>
    {
        unimplemented!();
    }
}

impl<I: Iterator<char>> Iterator<Result<Block,String>> for Preprocessor<I>
{
    fn next(&mut self) -> Option<Result<Block,String>>
    {
        match self.it.peek() {
            Some(Ok(tok)) => match tok {
                Token(token::Kind::Symbol, ref symbol) if (symbol.as_slice() == "#") => {
                    Some(self.preprocess_directive())
                },
                Token(token::Kind::Symbol, ref symbol) if (symbol.as_slice() == "//") => {
                    unimplemented!();
                },
                Token(token::Kind::Symbol, ref symbol) if (symbol.as_slice() == "/*") => {
                    unimplemented!();
                },
                // it's just a regular token - pass it on.
                _ => {
                    self.it.eat(); // chew on the token so we don't choke next iteration
                    
                    Some(Ok(Block::Token(tok)))
                }
            },
            Some(Err(err)) => {
                return Some(Err(err));
            },
            None => {
                return None;
            }
        }
    }
}

