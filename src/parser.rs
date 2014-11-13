
use ast;
use std;

use tokenizer::{Tokenizer,Token,TokenSymbol};

pub struct Parser
{
    pub ast: ast::Ast,
}

impl Parser
{
    pub fn new() -> Parser
    {
        Parser {
            ast: ast::Ast::new(),
        }
    }
    
    /// Parses a tokenizer.
    pub fn parse<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>)
    {
        match it.peek() {
            Some(tok) => match tok {
                TokenSymbol(ref symbol) => {
                    match symbol.as_slice() {
                        "#" => {
                            self.parse_preprocessor(it);
                        },
                        _ => (),
                    }
                },
                _ => ()
            },
            None => (),
        }
    }
    
    /// Parses a preprocessor statement.
    /// The tokenizer should be in a state such that the next read token is TokenSymbol("#").
    pub fn parse_preprocessor<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>)
    {
        it.expect_assert(&TokenSymbol("#".to_string()));
        
        println!("preprocessor!");
    }
}

