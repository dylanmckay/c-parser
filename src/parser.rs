
use ast;

use token;
use Identifier;
use token::{Tokenizer,Token};
use ast::{Expr,ExprIdentifier,Expression,Statement};

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
                    Token(token::KindSymbol, ref symbol) if (symbol.as_slice() == "//") => {
                        try!(self.parse_line_comment(&mut it))
                    },
                    Token(token::KindSymbol, ref symbol) if (symbol.as_slice() == "/*") => {
                        try!(self.parse_block_comment(&mut it))
                    },
                    Token(token::KindNewLine,_) => {
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
    /*
    /// Parses a preprocessor statement.
    /// The tokenizer should be in a state such that the next read token is TokenSymbol("#").
    fn parse_preprocessor<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        expect::assert_token(it.next(), &Token::hash());
        
        match try!(expect::kind(it.peek(), token::KindWord)) {
            Token(token::KindWord, ref word) => match word.as_slice() {
                "define" => self.parse_preprocessor_define(it),
                "if" => self.parse_preprocessor_if(it),
                
                _ => { return Err("not a valid directive".to_string()); }
            },
            _ => { // we can only have words.
                unreachable!()
            },
        }
    }
    
    /// Parses a preprocessor `#define` statement.
    fn parse_preprocessor_define<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        expect::assert_token(it.next(), &Token::define());
        
        match it.peek() {
            Some(Ok(Token(token::KindWord, name_str))) => {
                it.next(); // eat name.
                
                let name = match Identifier::from_name(name_str) {
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
                try!(expect::token(it.next(), &Token::new_line()));
                
                return result;
                
            },
            Some(Err(err)) => { return Err(err); },
            _ => Err("expected identifier".to_string())
        }
    }
    
    fn parse_preprocessor_if<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        let precondition = try!(self.parse_preprocessor_expression(it));
        
        unimplemented!();
    }
    
    /// Parses a preprocessor #define function.
    /// Examples:
    /// ``` c
    /// #define ABC(b) b
    /// #define _SFR_IO8(addr)
    /// ```
    fn parse_preprocessor_function<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>, name: Identifier) -> Result<(), String>
    {
        let parameter_list = try!(self.parse_preprocessor_function_parameters(it));
        let expression = try!(self.parse_preprocessor_expression(it));
        
        self.ast.nodes.push(ast::statements::preprocessor::Define::function(name, parameter_list, expression).to_stmt());
        
        Ok(())
    }
    
    /// Parses the parameter list of a #define function(a,b,c)
    /// The next token should be '(' at the point of calling this function.
    fn parse_preprocessor_function_parameters<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<Vec<Identifier>, String>
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
    fn parse_preprocessor_constant<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>, name: Identifier) -> Result<(), String>
    {
        let expression = try!(self.parse_preprocessor_expression(it));

        self.ast.nodes.push(ast::statements::preprocessor::Define::constant(name, expression).to_stmt());
        
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
    }*/
    
    fn parse_block_comment<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        expect::assert_token(it.next(), &Token::forward_slash_asterix());
        
        let body = try!(self.parse_comment_body(it, &Token::asterix_forward_slash()));
        
        self.ast.nodes.push(ast::StmtComment(ast::statements::Comment(ast::statements::comment::KindBlock, body)));
        
        Ok(())
    }
    
    fn parse_line_comment<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<(), String>
    {
        expect::assert_token(it.next(), &Token::forward_slash_slash());
        
        let body = try!(self.parse_comment_body(it, &Token::new_line()));
        
        self.ast.nodes.push(ast::StmtComment(ast::statements::Comment(ast::statements::comment::KindLine, body)));
        
        Ok(())
    }
    
    /// Parses the text from a comment.
    /// Stops when `terminator` is reached.
    /// NOTE: This does not properly handle nested comments, i.e. `/* /* */`, or even the properly terminated `/* /* */ */`.
    /// TODO: This function ignores the body and returns an empty string; fix this/
    fn parse_comment_body<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>, terminator: &Token) -> Result<String, String>
    {
        loop {
            let token = try!(expect::something(it.next()));
            
            if &token == terminator {
                break;
            }
        }
        
        Ok("".to_string())
    }
    
    /// Parses an expression.
    fn parse_expression<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match try!(expect::something(it.peek()))
        {
            Token(token::KindWord, _) => self.parse_identifier(it),
            Token(token::KindIntegerLiteral, _) => self.parse_integer_literal(it),
            _ => Err("unexpected token".to_string())
        }
    }
    
    fn parse_identifier<I: Iterator<char>>(&mut self, it: &mut Tokenizer<I>) -> Result<ast::Expr, String>
    {
        match expect::assert_kind(it.next(), token::KindWord) {
            // create a new identifier.
            Token(token::KindWord, name) => match Identifier::from_name(name) {
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
        match expect::assert_kind(it.next(), token::KindIntegerLiteral) {
            // create a new integer literal.
            Token(token::KindIntegerLiteral, val) => {
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

/// A collection of internal methods for token checking.
#[allow(dead_code)]
mod expect
{
    use token::{Token,TokenKind};
    use util;
    
    /// Checks that there is a token.
    pub fn something(opt: Option<Result<Token,String>>) -> Result<Token,String>
    {
        match opt {
            Some(thing) => thing,
            None => Err("expected a token".to_string()),
        }
    }
    
    /// Asserts that there is a token.
    pub fn assert_something(opt: Option<Result<Token,String>>) -> Token
    {
        self::assert_result(self::something(opt))
    }
    
    /// Checks that a token is equal to a given token.
    pub fn token(opt: Option<Result<Token,String>>, expected_token: &Token) -> Result<Token,String>
    {
        match opt {
            Some(Ok(read_token)) => {
                if &read_token == expected_token {
                    return Ok(read_token);
                }
            },
            Some(Err(err)) => {
                return Err(err);
            },
            _ => (),
        }
        
        let msg = format!("expected {}", expected_token);
        Err(msg)
    }
    
    /// Asserts that a token is equal to a given token.
    pub fn assert_token(opt: Option<Result<Token,String>>, expected_token: &Token) -> Token
    {
        self::assert_result(self::token(opt, expected_token))
    }
    
    /// Checks that a token is of a given kind.
    pub fn kind(opt: Option<Result<Token,String>>, expected_kind: TokenKind) -> Result<Token,String>
    {
        match opt {
            Some(Ok(read_token)) => {
                if read_token.is(expected_kind) {
                    return Ok(read_token);
                }
            },
            Some(Err(err)) => {
                return Err(err);
            },
            _ => (),
        }
        
        let msg = format!("expected {}", expected_kind);
        Err(msg)
    }
    
    /// Checks that a token is of a given kind.
    pub fn assert_kind(opt: Option<Result<Token,String>>, expected_kind: TokenKind) -> Token
    {
        self::assert_result(self::kind(opt, expected_kind))
    }
    
    /// Checks that a token is an element of a set of kinds.
    pub fn kinds<I: Iterator<TokenKind>>(opt: Option<Result<Token,String>>, mut expected_kinds: I) -> Result<Token,String>
    {
        // a list of kinds collected from the expected kind iterator.
        // if the token is not matched, this list will contain all expected kinds.
        let mut kind_list = Vec::new();
        
        match opt {
            Some(Ok(read_token)) => {
                for expected_kind in expected_kinds {
                
                    kind_list.push(expected_kind.clone());
                    
                    if read_token.is(expected_kind) {
                        return Ok(read_token);
                    }
                }
            },
            Some(Err(err)) => {
                return Err(err);
            },
            _ => (),
        }
        
        let msg = format!("expected one of: {}", kind_list);
        Err(msg)
    }
    
    /// Asserts that a token is an element of a set of kinds.
    pub fn assert_kinds<I: Iterator<TokenKind>>(opt: Option<Result<Token,String>>, expected_kinds: I) -> Token
    {
        self::assert_result(self::kinds(opt, expected_kinds))
    }
    
    /// Checks that the next token is one of a set of tokens.
    pub fn one_of<'a, I: Iterator<&'a Token>>(opt: Option<Result<Token,String>>, mut expected_tokens: I) -> Result<Token,String>
    {
        // a list of tokens collected from the expected token iterator.
        // if the token is not matched, this list will contain all expected tokens.
        let mut token_list = Vec::new();
        
        match opt {
            Some(Ok(read_token)) => {
                
                for possible_token in expected_tokens {
                    token_list.push(possible_token.clone());
                    
                    if &read_token == possible_token {
                        return Ok(read_token);
                    }
                }
            },
            Some(Err(err)) => {
                return Err(err);
            },
            _ => (),
        };
        
        let msg = format!("expected one of: {}", util::build_list_str(token_list.iter()));
        Err(msg)
    }
    
    /// Asserts that the next token is one of a set of tokens.
    pub fn assert_one_of<'a, I: Iterator<&'a Token>>(opt: Option<Result<Token,String>>, expected_tokens: I) -> Token
    {
        match self::one_of(opt, expected_tokens) {
            Ok(token) => token,
            Err(err) => panic!(err),
        }
    }
    
    /// Helper method for unwrapping an expect result.
    fn assert_result(opt: Result<Token,String>) -> Token
    {
        match opt {
            Ok(token) => token,
            Err(err) => panic!(err),
        }
    }
        
}

