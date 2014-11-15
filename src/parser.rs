
use ast;

use token;
use token::{Tokenizer,Token};
use ast::{Expr,ExprIdentifier,Expression};

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
                    Token(token::KindSymbol, ref symbol) if symbol.as_slice() == "#" => {
                        try!(self.parse_preprocessor(&mut it))
                    },
                    Token(token::KindNewLine,_) => {
                    
                        continue;
                    },
                    _ => { return Err(format!("unknown token: {}", tok)); } // we don't know how to handle this token.
                },
                Some(Err(err)) => { return Err(err); },
                None => { return Ok(()); }, // we reached the end.
            }
        }
    }
    
    /// Parses a preprocessor statement.
    /// The tokenizer should be in a state such that the next read token is TokenSymbol("#").
    fn parse_preprocessor<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        it.expect_assert(&Token::hash());
        
        match it.peek() {
            Some(Ok(Token(token::KindWord, ref word))) if word.as_slice() == "define" => {
                self.parse_preprocessor_define(it)
            },
            Some(Err(err)) => { return Err(err); },
            a => { Err(format!("unknown directive: '{}'", a).to_string()) },
        }
    }
    
    /// Parses a preprocessor `#define` statement.
    fn parse_preprocessor_define<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        it.expect_assert(&Token::define());
        
        match it.peek() {
            Some(Ok(Token(token::KindWord, name_str))) => {
                it.next(); // eat name.
                
                let name = match ast::expressions::Identifier::from_name(name_str) {
                    Some(ident) => ident,
                    None => { return Err("invalid identifier".to_string()); }
                };
                
                let result = match it.peek() {
                    // check if it is a function.
                    Some(Ok(Token(token::KindSymbol, ref sym))) if sym.as_slice() == "(" => {
                        self.parse_preprocessor_function(it, name)
                    },
                    Some(Err(err)) => { return Err(err); },
                    // it is a constant
                    Some(..) | None => {
                        self.parse_preprocessor_constant(it, name)
                    },
                };
                
                // if parsing failed, return the error.
                try!(result);
                
                // a new line should always proceed a #define.
                try!(it.expect(&Token::new_line()));
                
                return result;
                
            },
            Some(Err(err)) => { return Err(err); },
            _ => Err("expected identifier".to_string())
        }
    }
    
    /// Parses a preprocessor #define function.
    /// Examples:
    /// ``` c
    /// #define ABC(b) b
    /// #define _SFR_IO8(addr)
    /// ```
    fn parse_preprocessor_function<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>, name: ast::expressions::Identifier) -> Result<(), String>
    {
        let parameter_list = try!(self.parse_preprocessor_function_parameters(it));
        let expression = try!(self.parse_preprocessor_expression(it));
        
        self.ast.nodes.push(ast::StmtDefine(ast::statements::DefineFunction(ast::preprocessor::Function {
            name: name,
            params: parameter_list,
            expr: expression,
        })));
        
        Ok(())
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
            Ok(FromIterator::from_iter(args.into_iter().map(|a| match a {
                ast::ExprIdentifier(ident) => ident,
                _ => unreachable!(),
            })))
        } else {
            Err("expected identifier".to_string())
        }
    }
    
    /// Parses a preprocessor that is not a function, examples:
    /// ``` c
    /// #define ABC
    /// #define foo bar
    /// #define asdf 1
    /// ```
    fn parse_preprocessor_constant<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>, name: ast::expressions::Identifier) -> Result<(), String>
    {
        let expression = try!(self.parse_preprocessor_expression(it));

        self.ast.nodes.push(ast::StmtDefine(ast::statements::DefineConstant(ast::preprocessor::Constant {
            name: name,
            expr: expression,
        })));
        
        Ok(())
    }
    
    /// Parses an optional preprocessor expression.
    /// `#define asdf [expression]`.
    fn parse_preprocessor_expression<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<Option<ast::Expr>,String>
    {
        match it.peek() {
            // there is no following expression.
            Some(Ok(Token(token::KindNewLine,_))) | None => {
                Ok(None)
            },
            Some(Err(err)) => { return Err(err); },
            Some(..) => {
                Ok(Some(try!(self.parse_expression(it))))
            },
        }
    }
    
    /// Parses an expression.
    fn parse_expression<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match it.next()
        {
            Some(Ok(Token(token::KindWord,word))) => Ok(ast::expressions::Identifier::from_name(word).unwrap().to_expr()),
            Some(Err(err)) => { return Err(err); },
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

            let peeked_token = try!(self::expect_token(it.peek()));
            try!(peeked_token.expect_one_of([Token::comma(), Token::right_parenthesis()].iter()));
        }
        
        it.expect_assert(&Token::right_parenthesis());
        
        Ok(expressions)
    }
    
}

/// Unwraps an `Option<Result<Token,String>>`, giving either Ok(Token) or Err(msg).
fn expect_token(opt: Option<Result<Token,String>>) -> Result<Token,String>
{
    match opt {
        Some(res) => res,
        None => Err("expected a token".to_string()),
    }
}

