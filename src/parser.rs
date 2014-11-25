
use ast;

use token;
use token::expect;
use Identifier;
use token::{Tokenizer,Token};
use ast::{Expr,Expression,Statement};

/// A parser can read C code and encode it into an AST.
pub struct Parser
{
    pub ast: ast::Ast,
}

impl Parser
{
    /// Creates a new parser.
    pub fn new() -> Parser
    {
        Parser {
            ast: ast::Ast::new(),
        }
    }
    
    /// Parses a tokenizer.
    pub fn parse<I: Iterator<char>>(&mut self, mut it: Tokenizer<I>) -> Result<(), String>
    {
        loop {
            match it.peek() {
                Some(Ok(tok)) => match tok {
                    Token(token::Kind::NewLine,_) => {
                        it.eat();
                        continue;
                    },
                    _ => { return Err(format!("unknown token: {}", tok)); } // we don't know how to handle this token.
                },
                Some(Err(err)) => { return Err(err); },
                None => { return Ok(()); }, // we reached the end.
            }
        }
    }
    
    /// Parses an expression.
    fn parse_expression<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match try!(expect::something(it.peek()))
        {
            Token(token::Kind::Word, _) => self.parse_identifier(it),
            Token(token::Kind::IntegerLiteral, _) => self.parse_integer_literal(it),
            _ => Err("unexpected token".to_string())
        }
    }
    
    fn parse_identifier<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match expect::assert_kind(it.next(), token::Kind::Word) {
            // create a new identifier.
            Token(token::Kind::Word, name) => match Identifier::from_name(name) {
                // the word is a valid identifier.
                Some(ident) => Ok(ident.to_expr()),
                
                // the word is an ill formed identifier.
                None => Err("invalid identifier".to_string()),
            },
            _ => unreachable!(),
        }
    }
    
    fn parse_integer_literal<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match expect::assert_kind(it.next(), token::Kind::IntegerLiteral) {
            // create a new integer literal.
            Token(token::Kind::IntegerLiteral, val) => {
                Ok(ast::expressions::IntegerLiteral(val).to_expr())
            },
            _ => unreachable!(),
        }
    }
    
    
    /// Parses an argument list (a set of expressions, in parentheses, seperated by commas).
    /// For example: "(abc, 123, bvs)".
    fn parse_argument_list<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<Vec<ast::Expr>, String>
    {
        expect::assert_token(it.next(), &Token::left_parenthesis());
        
        let mut expressions = Vec::new();
        
        loop {
            
            match it.peek() {
                Some(Ok(ref token)) if token == &Token::right_parenthesis() => {
                    break;
                },
                // eat comma, we explicitly check for it the previous iteration.
                Some(Ok(ref token)) if token == &Token::comma() => it.eat(),
                Some(Err(err)) => { return Err(err); },
                _ => (),
            }
            
            let expr = try!(self.parse_expression(it));
            expressions.push(expr);

            try!(expect::one_of(it.peek(), [Token::comma(), Token::right_parenthesis()].iter()));
        }
        
        expect::assert_token(it.next(), &Token::right_parenthesis());
        
        Ok(expressions)
    }
    
}

