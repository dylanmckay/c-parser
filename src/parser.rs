
use ast;
use std;

use tokenizer::{Tokenizer,Token,TokenSymbol,TokenWord,TokenNewLine};
use ast::Expression;

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
            Some(TokenWord(name)) => {
                it.next(); // eat name.
                
   
                self.parse_preprocessor_constant(it, name)
                
                
            },
            _ => Err("expected identifier".to_string())
        }
    }
    
    fn parse_preprocessor_function<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<(), String>
    {
        unimplemented!();
        Ok(())
    }
    
    fn parse_preprocessor_constant<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>, name: String) -> Result<(), String>
    {
        let expr = match it.peek() {
            // there is no following expression.
            Some(TokenNewLine) | None => {
                None
            },
            Some(t) => {
                Some(try!(self.parse_expression(it)))
            },
        };

        self.ast.nodes.push(ast::StmtDefine(ast::statements::DefineConstant(ast::preprocessor::Constant {
            name: match ast::expressions::Identifier::from_name(name) {
                Some(ident) => ident,
                None => { return Err("invalid identifier".to_string()); }
            },
            expr: expr,
        })));
        
        Ok(())
    }
    
    fn parse_expression<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match it.next()
        {
            Some(TokenWord(word)) => Ok(ast::expressions::Identifier::from_name(word).unwrap().to_expr()),
            Some(..) | None => unimplemented!()
        }
    }
    
}

