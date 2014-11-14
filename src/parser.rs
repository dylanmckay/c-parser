
use ast;

use token;
use token::{Tokenizer,Token};
use ast::{Expr,ExprIdentifier,Expression};

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
                Token(token::KindSymbol, ref symbol) => {
                    match symbol.as_slice() {
                        "#" => {
                            self.parse_preprocessor(&mut it)
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
    fn parse_preprocessor<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        it.expect_assert(&Token::hash());
        
        match it.peek() {
            Some(Token(token::KindWord, ref word)) if word.as_slice() == "define" => {
                self.parse_preprocessor_define(it)
            },
            a => { Err(format!("unknown thingy: '{}'", a).to_string()) },
        }
    }
    
    fn parse_preprocessor_define<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        it.expect_assert(&Token::define());
        
        match it.peek() {
            Some(Token(token::KindWord, name)) => {
                it.next(); // eat name.
                
                match it.peek() {
                    // check if it is a function.
                    Some(Token(token::KindSymbol, ref sym)) if sym.as_slice() == "(" => {
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
    
    fn parse_preprocessor_function<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>, name: String) -> Result<(), String>
    {
        let parameter_list = self.parse_preprocessor_function_parameters(it);
        
        unimplemented!();
    }
    
    /// Parses the parameter list of a #define function(a,b,c)
    /// The next token should be '(' at the point of calling this function.
    fn parse_preprocessor_function_parameters<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<Vec<ast::expressions::Identifier>, String>
    {
        // here we abuse this function because a preprocessor parameter list
        // resembles a regular argument list, but with all arguments being identifiers.
        let args = try!(self.parse_argument_list(it));
        
        // whether the argument list is a valid parameter list; that all its expressions are identifiers.
        let is_valid = args.iter().all(|a| match a {
            &ExprIdentifier(..) => true,
            _ => false,
        });
        
        if is_valid {
            Ok(args.map_in_place(|a| match a {
                ast::ExprIdentifier(ident) => ident,
                _ => unreachable!(),
            }))
        } else {
            Err("expected identifier".to_string())
        }
    }
    
    fn parse_preprocessor_constant<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>, name: String) -> Result<(), String>
    {
        let expr = match it.peek() {
            // there is no following expression.
            Some(Token(token::KindNewLine,_)) | None => {
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
    
    fn parse_expression<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match it.next()
        {
            Some(Token(token::KindWord,word)) => Ok(ast::expressions::Identifier::from_name(word).unwrap().to_expr()),
            Some(..) | None => unimplemented!()
        }
    }
    
    
    /// Parses an argument list (a set of expressions, in parentheses, seperated by commas).
    /// For example: "(abc, 123, bvs)".
    fn parse_argument_list<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<Vec<ast::Expr>, String>
    {
        it.expect_assert(&Token::left_parenthesis());
        
        let mut expressions = Vec::new();
        
        loop {
            
            match it.peek() {
                Some(ref token) if token == &Token::right_parenthesis() => {
                    break;
                },
                _ => (),
            }
            
            let expr = try!(self.parse_expression(it));
            expressions.push(expr);
        }
        
        it.expect_assert(&Token::right_parenthesis());
        
        Ok(expressions)
    }
    
}

