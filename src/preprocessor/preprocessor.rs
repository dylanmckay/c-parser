
use token;
use token::{expect,Token};

use Identifier;
use token::Tokenizer;

/// A block of code.
#[deriving(Show)]
pub enum Block
{
    BlockRegular,
}

pub struct Preprocessor
{
    pub blocks: Vec<Block>,
}

impl Preprocessor
{
    pub fn new() -> Preprocessor
    {
        Preprocessor {
            blocks: Vec::new(),
        }
    }
    
    pub fn preprocess<I: Iterator<char>>(&mut self, mut tokenizer: Tokenizer<I>) -> Result<(),String>
    {
        loop {
            match tokenizer.peek() {
                Some(Ok(tok)) => match tok {
                    Token(token::Kind::Symbol, ref symbol) if (symbol.as_slice() == "#") => {
                        try!(self.preprocess_directive(&mut tokenizer));
                    },
                    Token(token::Kind::Symbol, ref symbol) if (symbol.as_slice() == "//") => {
                        unimplemented!();
                    },
                    Token(token::Kind::Symbol, ref symbol) if (symbol.as_slice() == "/*") => {
                        unimplemented!();
                    },
                    _ => {
                        return Err(format!("unknown token: {}", tok));
                    }
                },
                Some(Err(err)) => {
                    return Err(err);
                },
                None => {
                    return Ok(());
                }
            }
        }
    }
    
    fn preprocess_directive<I: Iterator<char>>(&mut self, tokenizer: &mut Tokenizer<I>) -> Result<(),String>
    {
        expect::assert_token(tokenizer.next(), &Token::hash());
        
        match try!(expect::kind(tokenizer.next(), token::Kind::Word)) {
            Token(token::Kind::Word, ref word) => match word.as_slice() {
            
                "define" => self.preprocess_define(tokenizer),
                d => { return Err(format!("unknown directive: {}", d)); },
            
            },
            _ => unreachable!(),
        }
    }
    
    fn preprocess_define<I: Iterator<char>>(&mut self, tokenizer: &mut Tokenizer<I>) -> Result<(),String>
    {
        let name_str = try!(expect::kind(tokenizer.next(), token::Kind::Word)).move_value();
        
        let name = match Identifier::from_name(name_str) {
            Some(name) => name,
            None => { return Err("invalid identifier".to_string()); },
        };
        
        match try!(expect::something(tokenizer.peek())) {
            Token(token::Kind::Symbol, ref sym) if sym.as_slice() == "(" => {
                unimplemented!();
            },
            _ => {
                self.preprocess_define_constant(name, tokenizer)
            }
        }
    }
    
    fn preprocess_define_constant<I: Iterator<char>>(&mut self, name: Identifier, tokenizer: &mut Tokenizer<I>) -> Result<(),String>
    {
        unimplemented!();
    }
}

