
use ast;
use std;

use tokenizer::{Tokenizer,Token,TokenSymbol,TokenWord};

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
    pub fn parse<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<(), String>
    {
        match it.peek() {
            Some(tok) => match tok {
                TokenSymbol(ref symbol) => {
                    match symbol.as_slice() {
                        "#" => {
                            self.parse_preprocessor(it)
                        },
                        _ => Ok(()), // we don't know what to do with the symbol so just ignore.
                    }
                },
                _ => Ok(()) // we don't know how to handle this token.
            },
            None => Ok(()), // we reached the end.
        }
    }
    
    /// Parses a preprocessor statement.
    /// The tokenizer should be in a state such that the next read token is TokenSymbol("#").
    fn parse_preprocessor<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<(), String>
    {
        it.expect_assert(&TokenSymbol("#".to_string()));
        
        match try!(it.peek_word()).as_slice() {
            "define" => {
                self.parse_preprocessor_define(it)
            },
            a => { Err(format!("unknown thingy: '{}'", a).to_string()) },
        }
    }
    
    fn parse_preprocessor_define<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<(), String>
    {
        it.expect_assert(&TokenWord("define".to_string()));
        
        match it.peek() {
            Some(tok) => match tok {
                TokenSymbol(ref symbol) => {
                    if symbol.as_slice() == "(" {
                        self.parse_preprocessor_function(it)
                    } else {
                        Err("unexpected symbol".to_string())
                    }
                },
                TokenWord(name) => {
                    self.parse_preprocessor_constant(it, name)
                }
                ,
                _ => Err("expected word or '('".to_string())
            },
            None => Err("expected token".to_string())
        }
    }
    
    fn parse_preprocessor_function<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<(), String>
    {
        unimplemented!();
        Ok(())
    }
    
    fn parse_preprocessor_constant<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>, name: String) -> Result<(), String>
    {
        let expression = try!(self.parse_expression(it));
        
        self.ast.nodes.push(ast::StmtDefine(ast::statements::DefineConstant(ast::preprocessor::Constant {
            name: match ast::Identifier::from_name(name) {
                Some(ident) => ident,
                None => { return Err("invalid identifier".to_string()); }
            },
            expr: Some(expression),
        })));
        
        Ok(())
    }
    
    fn parse_expression<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<ast::Expr, String>
    {
        unimplemented!();
    }
    
}

