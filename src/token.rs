
use ast;
use std;

use identifier;
use Identifier;

use util::IteratorPeeker;

// TODO: seperate keyword and identifier? is it a good idea?


/// The type of a token.
#[deriving(Clone,PartialEq,Eq)]
pub enum Kind
{
    Symbol,
    Word,
    IntegerLiteral,
    StringLiteral,
    NewLine,
}

impl std::fmt::Show for Kind
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        match self {
            &Kind::Symbol => "symbol",
            &Kind::Word => "word",
            &Kind::IntegerLiteral => "integer",
            &Kind::StringLiteral => "string",
            &Kind::NewLine => "new line",
        }.fmt(formatter)
    }
}

/// A token.
#[deriving(Clone,PartialEq,Eq)]
pub struct Token(pub Kind, pub String);

impl Token
{
    // Special characters.
    pub fn new_line() -> Token { Token(Kind::NewLine, "new-line".to_string()) }
    
    // Symbols.
    pub fn left_parenthesis() -> Token { Token(Kind::Symbol, "(".to_string()) }
    pub fn right_parenthesis() -> Token { Token(Kind::Symbol, ")".to_string()) }
    pub fn hash() -> Token { Token(Kind::Symbol, "#".to_string()) }
    pub fn comma() -> Token { Token(Kind::Symbol, ",".to_string()) }
    pub fn semicolon() -> Token { Token(Kind::Symbol, ";".to_string()) }
    pub fn forward_slash() -> Token { Token(Kind::Symbol, "/".to_string()) }
    pub fn forward_slash_asterix() -> Token { Token(Kind::Symbol, "/*".to_string()) }
    pub fn asterix_forward_slash() -> Token { Token(Kind::Symbol, "*/".to_string()) }
    pub fn forward_slash_slash() -> Token { Token(Kind::Symbol, "//".to_string()) }
    pub fn asterix() -> Token { Token(Kind::Symbol, "*".to_string()) }
    
    // Keywords.
    pub fn define() -> Token { Token(Kind::Word, "define".to_string()) }
    
    pub fn move_value(mut self) -> String
    {
        match self {
            Token(_, val) => val,
        }
    }
    
    pub fn value<'a>(&'a self) -> &'a str
    {
        match self {
            &Token(_, ref val) => val.as_slice(),
        }
    }
    
    /// Checks if the token is of a given kind.
    pub fn is(&self, kind: Kind) -> bool
    {
        match self {
            &Token(k, _) => k == kind,
        }
    }
}

impl std::fmt::Show for Token
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(),std::fmt::Error>
    {
        match self {
            &Token(_, ref s) => s.fmt(formatter),
        }
    }
}

/// A tokenizer.
#[deriving(Clone)]
pub struct Tokenizer<I: Iterator<char>>
{
    it: IteratorPeeker<char, I>,
    stack: Vec<Token>,
    finished: bool,
    
    // the possible symbols.
    symbol_tokens: Vec<&'static str>,
}

impl<I: Iterator<char>> Tokenizer<I>
{
    /// Creates a new tokenizer.
    pub fn new(it: I) -> Tokenizer<I>
    {
        let mut symbol_tokens = vec![
            ";", "#", ":", ",",
            "(", ")", "[", "]",
            "/", "*", "&",
            
            // comment tokens.
            "/*", "*/", "//",
            
            // arithmetic operators.
            "+", "-", "*", "/",
            
            // arithmetic assignment operators.
            "+=", "-=", "*=", "/=",
            
            // comparison operators.
            "<", "<=", ">", ">=",
            
        ];
        
        // sort the symbol tokens by length, so that the longer symbols are at the beginning.
        // this way our symbol finding code works (we check += before +).
        symbol_tokens.sort_by(|&e1,&e2| e2.len().cmp(&e1.len()));
        
        Tokenizer {
            it: IteratorPeeker::new(it),
            stack: Vec::new(),
            finished: false,
            
            symbol_tokens: symbol_tokens,
        }
    }
    
    /// Peeks at the next token.
    pub fn peek(&mut self) -> Option<Result<Token,String>>
    {
        let val: Token = match self.stack.pop() {
            Some(val) => val.clone(),
            None => {
                match self.next() {
                    Some(val) => {
                        match val {
                            Ok(a) => a,
                            err => { return Some(err); }
                        }
                    },
                    None => { return None; },
                }
            }
        };

        self.stack.push(val.clone());
        Some(Ok(val))
    }
    
    /// Eats the next token, disregarding it.
    pub fn eat(&mut self)
    {
        self.next();
    }
    
    /// Peeks at the n'th token from the current index.
    pub fn peek_n(&mut self, n: uint) -> Option<Result<Token,String>>
    {
        let mut read_elems = Vec::new();
        
        for _ in range(0,n+1) {
        
            match self.next() {
                Some(e) => {
                    match e {
                        Ok(tok) => read_elems.push(tok),
                        err => { return Some(err); }
                    }
                },
                None => {
                    break;
                },
            }
        }

        for read_char in read_elems.iter().rev() {
            self.stack.push(read_char.clone());
        }

        match read_elems.last() {
            Some(a) => Some(Ok(a.clone())),
            None => None
        }
    }

    fn parse_identifier(&mut self) -> Result<Token,String>
    {
        let mut chars = vec![ self.it.next().unwrap() ];
        
        loop {
            let c = match self.it.peek() {
                Some(c) => c,
                None => break,
            };
            
            if identifier::is_valid_char(c) {

                chars.push(c);
                
                // eat the character.
                self.it.next();
            } else {
                break;
            }
        }

        Ok(Token(Kind::Word, String::from_chars(chars.as_slice())))
    }
    
    fn parse_numeric_literal(&mut self) -> Result<Token,String>
    {
        // we should be at the first digit.
        assert!(self.it.peek().unwrap().is_digit(10));
        
        let mut result = String::new();
        result.push(self.it.next().unwrap());
        
        loop {
            match self.it.peek() {
                // check if it is hexadecimal..
                Some(c) if (c.is_digit(16)) | (c == 'x') => {
                    self.it.eat();
                    result.push(c);
                },
                Some(..) | None => { break; }
            }
        }
        
        Ok(Token(Kind::IntegerLiteral, result))
    }
    
    fn parse_possible_symbol(&mut self) -> Result<Token,String>
    {
        'symbol_loop: for sym in self.symbol_tokens.iter() {
            for (index,symbol_char) in sym.chars().enumerate() {
                
                let peeked_char = match self.it.peek_n(index) {
                    Some(c) => c,
                    None => { continue 'symbol_loop; },
                };
                
                // check the character againsts the symbol.
                if peeked_char != symbol_char {
                    continue 'symbol_loop;
                }
            }
            
            // eat the symbol characters.
            self.it.eat_several(sym.len());
            
            // we have found a symbol match.
            return Ok(Token(Kind::Symbol, sym.to_string()));
        }
        
        // no matches.
        Err("unknown token".to_string())
    }
}

impl<I: Iterator<char>> Iterator<Result<Token,String>> for Tokenizer<I>
{
    /// Gets the next token.
    /// The last token retrived by this function will always be a new line.
    fn next(&mut self) -> Option<Result<Token,String>>
    {
        
        
        // if we have peeked data on the stack, retrieve it.
        match self.stack.pop() {
            Some(tok) => { return Some(Ok(tok)); },
            None => (),
        };
        
        // note that this must be below 'match self.stack.pop()' because
        // self.peek() calls this function and then pushes result. if this
        // is above aforementioned block, self.peek() == Some(NewLine) & self.next() == None
        if self.finished {
            return None;
        }
        
        self.it.eat_whitespace_but_line();
        
        let first_char = match self.it.peek() {
            Some(first_char) => first_char,
            
            // we reached the EOF.
            None => {
                self.finished = true;
                
                return Some(Ok(Token::new_line()));
            }
        };
        
        if first_char == '\n' {
            self.it.eat();
            return Some(Ok(Token::new_line()));
        } else if first_char == '\r' {
        
            match self.it.peek_n(1) {
                Some('\n') => {
                    self.it.next(); // skip '\r'.
                    self.it.next(); // skip '\n'.
                    
                    return Some(Ok(Token::new_line()));
                },
                Some(..) | None => ()
            }
        }
        
        if identifier::is_valid_first_char(first_char) {
            Some(self.parse_identifier())
        } else if first_char.is_digit(10) {
            Some(self.parse_numeric_literal())
        } else {
            Some(self.parse_possible_symbol())
        }
    }
}


/// A collection of internal methods for token checking.
#[allow(dead_code)]
pub mod expect
{
    use token::{Token,Kind};
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
    pub fn kind(opt: Option<Result<Token,String>>, expected_kind: Kind) -> Result<Token,String>
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
    pub fn assert_kind(opt: Option<Result<Token,String>>, expected_kind: Kind) -> Token
    {
        self::assert_result(self::kind(opt, expected_kind))
    }
    
    /// Checks that a token is an element of a set of kinds.
    pub fn kinds<I: Iterator<Kind>>(opt: Option<Result<Token,String>>, mut expected_kinds: I) -> Result<Token,String>
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
    pub fn assert_kinds<I: Iterator<Kind>>(opt: Option<Result<Token,String>>, expected_kinds: I) -> Token
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
