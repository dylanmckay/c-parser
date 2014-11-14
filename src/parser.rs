
use ast;

use token::{Tokenizer,Token,KindSymbol,KindWord,KindNewLine};
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
                Token(KindSymbol, ref symbol) => {
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
        it.expect_assert(&Token(KindSymbol, "#".to_string()));
        
        match try!(it.peek_word()).as_slice() {
            "define" => {
                self.parse_preprocessor_define(it)
            },
            a => { Err(format!("unknown thingy: '{}'", a).to_string()) },
        }
    }
    
    fn parse_preprocessor_define<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<(), String>
    {
        it.expect_assert(&Token(KindWord, "define".to_string()));
        
        match it.peek() {
            Some(Token(KindWord, name)) => {
                it.next(); // eat name.
                
                match it.peek() {
                    // check if it is a function.
                    Some(Token(KindSymbol, ref sym)) if sym.as_slice() == "(" => {
                        self.parse_preprocessor_function(it, name)
                    },
                    // it is a constant
                    Some(..) | None => {
                        self.parse_preprocessor_constant(it, name)
                    },
                }
                
            },
            _ => Err("expected identifier".to_string())
        }
    }
    
    fn parse_preprocessor_function<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>, name: String) -> Result<(), String>
    {
        unimplemented!();
    }
    
    fn parse_preprocessor_constant<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>, name: String) -> Result<(), String>
    {
        let expr = match it.peek() {
            // there is no following expression.
            Some(Token(KindNewLine,_)) | None => {
                None
            },
            Some(..) => {
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
            Some(Token(KindWord,word)) => Ok(ast::expressions::Identifier::from_name(word).unwrap().to_expr()),
            Some(..) | None => unimplemented!()
        }
    }
    
    
    /// Parses an argument list (a set of expressions, in parentheses, seperated by commas).
    /// For example: "(abc, 123, bvs)".
    fn parse_argument_list<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<Vec<ast::Expr>, String>
    {
        unimplemented!();
    }
    
    fn parse_token_parenthesis_list<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<Vec<Token>, String>
    {
        unimplemented!();
    }
    
}

