
use ast;
use std;

use util::IteratorPeeker;

// NOTES:
// Perhaps we should add a 'fork()' method to tokenizer.
// The forked tokenizer would then read tokens, and could be used for lookahead.
// And then we could destroy the forked tokenizer and then pass the original tokenizer
// to the appropriate parse function.
//
// TODO: seperate keyword and identifier?


/// The type of a token.
#[deriving(Clone,PartialEq,Eq)]
pub enum TokenKind
{
    KindSymbol,
    KindWord,
    KindIntegerLiteral,
    KindStringLiteral,
    KindNewLine,
}

impl std::fmt::Show for TokenKind
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(), std::fmt::FormatError>
    {
        match self {
            &KindSymbol => "symbol",
            &KindWord => "word",
            &KindIntegerLiteral => "integer",
            &KindStringLiteral => "string",
            &KindNewLine => "new line",
        }.fmt(formatter)
    }
}

/// A token.
#[deriving(Clone,PartialEq,Eq)]
pub struct Token(pub TokenKind, pub String);

impl Token
{
    // Special characters.
    pub fn new_line() -> Token { Token(KindNewLine, "new-line".to_string()) }
    
    // Symbols.
    pub fn left_parenthesis() -> Token { Token(KindSymbol, "(".to_string()) }
    pub fn right_parenthesis() -> Token { Token(KindSymbol, ")".to_string()) }
    pub fn hash() -> Token { Token(KindSymbol, "#".to_string()) }
    pub fn comma() -> Token { Token(KindSymbol, ",".to_string()) }
    pub fn semicolon() -> Token { Token(KindSymbol, ";".to_string()) }
    pub fn forward_slash() -> Token { Token(KindSymbol, "/".to_string()) }
    pub fn asterix() -> Token { Token(KindSymbol, "*".to_string()) }
    
    // Keywords.
    pub fn define() -> Token { Token(KindWord, "define".to_string()) }
    
    /// Checks if the token is of a given kind.
    pub fn is(&self, kind: TokenKind) -> bool
    {
        match self {
            &Token(k, _) => k == kind,
        }
    }
}

impl std::fmt::Show for Token
{
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> Result<(),std::fmt::FormatError>
    {
        match self {
            &Token(_, ref s) => s.fmt(formatter),
        }
    }
}

/// A tokenizer.
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
            
            if ast::expressions::identifier::is_valid_char(c) {

                chars.push(c);
                
                // eat the character.
                self.it.next();
            } else {
                break;
            }
        }

        Ok(Token(KindWord, String::from_chars(chars.as_slice())))
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
            return Ok(Token(KindSymbol, sym.to_string()));
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
        
        if ast::expressions::identifier::is_valid_first_char(first_char) {
            Some(self.parse_identifier())
        } else {
            Some(self.parse_possible_symbol())
        }
    }
}


