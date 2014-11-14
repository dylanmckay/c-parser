
use ast;
use util;
use std;

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
    pub fn new_line() -> Token { Token(KindNewLine, "new-line".to_string()) }
    pub fn left_parenthesis() -> Token { Token(KindSymbol, "(".to_string()) }
    pub fn hash() -> Token { Token(KindSymbol, "#".to_string()) }
    
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
    pub it: IteratorPeeker<char, I>,
    stack: Vec<Token>,
    
    // the possible symbols.
    symbol_tokens: Vec<&'static str>,
}

impl<I: Iterator<char>> Tokenizer<I>
{
    /// Creates a new tokenizer.
    pub fn new(it: I) -> Tokenizer<I>
    {
        let mut symbol_tokens = vec![
            ";", "#", ":", 
            "(", ")", "[", "]",
            
            // arithmetic operators.
            "+", "-", "*", "/",
            
            // arithmetic assignment operators.
            "+=", "-=", "*=", "/="
        ];
        
        // sort the symbol tokens by length, so that the longer symbols are at the beginning.
        // this way our symbol finding code works (we check += before +).
        symbol_tokens.sort_by(|&e1,&e2| e2.len().cmp(&e1.len()));
        
        Tokenizer {
            it: IteratorPeeker::new(it),
            stack: Vec::new(),
            
            symbol_tokens: symbol_tokens,
        }
    }
    
    /// Peeks at the next token.
    pub fn peek(&mut self) -> Option<Token>
    {
        let val = match self.stack.pop() {
            Some(val) => val.clone(),
            None => {
                match self.next() {
                    Some(val) => {
                        val
                    },
                    None => { return None; },
                }
            }
        };
        
        self.stack.push(val.clone());
        Some(val)
    }
    
    /// Peeks at the n'th token from the current index.
    pub fn peek_n(&mut self, n: uint) -> Option<Token>
    {
        let mut read_elems = Vec::new();
        
        for _ in range(0,n+1) {
        
            match self.next() {
                Some(e) => {
                    read_elems.push(e);
                },
                None => {
                    break;
                },
            }
        }

        for read_char in read_elems.iter().rev() {
            self.stack.push(read_char.clone());
        }
        
        read_elems.last().map(|a| a.clone())
    }

    fn read_identifier(&mut self) -> Option<Token>
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

        Some(Token(KindWord, String::from_chars(chars.as_slice())))
    }
    
    fn read_possible_symbol(&mut self) -> Option<Token>
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
            self.it.eat(sym.len());
            
            // we have found a symbol match.
            return Some(Token(KindSymbol, sym.to_string()));
        }
        
        // no matches.
        None
    }
    
    /// Checks if the next token is the specified token.
    pub fn expect(&mut self, expected: &Token) -> Result<Token,String>
    {
        self.internal_expect(expected, || ())
    }
    
    /// Checks if the next token is the specified token, calling `panic!` if it isn't.
    pub fn expect_assert(&mut self, expected: &Token) -> Token
    {
        self.internal_expect(expected, || panic!(format!("expected {}", expected))).unwrap()
    }
    
    /// Checks if the next token is the specified token, calling `on_fail` if it isn't.
    pub fn internal_expect(&mut self, expected: &Token, on_fail: ||) -> Result<Token,String>
    {
        match self.next() {
            Some(token) => {
                if &token == expected {
                    return Ok(token);
                }
            },
            _ => ()
        }
        
        on_fail();
        Err(format!("expected {}", expected))
    }
    
    /// Checks that the next token is of a set of kinds.
    pub fn expect_kinds<I: Iterator<TokenKind>>(&mut self, expected_kinds: I) -> Result<Token,String>
    {
        self.internal_expect_kinds(expected_kinds, |_| ())
    }
    
    /// Checks that the next token is of a set of kinds, calling `on_fail` if it is not.
    pub fn expect_assert_kinds<I: Iterator<TokenKind>>(&mut self, expected_kinds: I) -> Token
    {
        self.internal_expect_kinds(expected_kinds, |kind_list| panic!(format!("expected one of: {}", util::build_list_str(kind_list.iter())))).unwrap()
    }
    
    /// Checks that the next token is of a set of kinds, calling `on_fail` if it is not.
    pub fn internal_expect_kinds<I: Iterator<TokenKind>>(&mut self, mut expected_kinds: I, on_fail: |&Vec<TokenKind>|) -> Result<Token,String>
    {
        let mut kind_list = Vec::new();
        
        match self.next() {
            Some(token) => match token {
                Token(read_kind, _) => {
                
                    // iterate through the expected kinds and check.
                    for expected_kind in expected_kinds {
                        kind_list.push(expected_kind);
                        
                        // check if we found a match.
                        if read_kind == expected_kind {
                            return Ok(token);
                        }
                    }
                }
            },
            None => ()
        }
        
        on_fail(&kind_list);
        Err(format!("expected one of: {}", util::build_list_str(kind_list.iter())))
    }
    
    pub fn expect_kind(&mut self, kind: TokenKind) -> Result<Token,String>
    {
        self.internal_expect_kind(kind, || ())
    }
    
    pub fn expect_assert_kind(&mut self, kind: TokenKind) -> Token
    {
        self.internal_expect_kind(kind, || panic!(format!("expected {}", kind))).unwrap()
    }
    
    /// Reads the next token, giving an Ok(Token) if it is of the specified kind, or Err(String) otherwise.
    pub fn internal_expect_kind(&mut self, kind: TokenKind, on_fail: ||) -> Result<Token,String>
    {
        match self.next() {
            Some(token) => if token.is(kind) {
                return Ok(token);
            },
            _ => ()
        }
        
        on_fail();
        Err(format!("expected {}", kind))
    }
    
    /// Expects a token out of a set of tokens.
    pub fn expect_one_of<I: Iterator<Token>>(&mut self, tokens: I) -> Result<Token,String>
    {
        self.internal_expect_one_of(tokens, |_|())
    }
    
    /// Asserts that the next token is one of a set.
    pub fn expect_assert_one_of<I: Iterator<Token>>(&mut self, tokens: I) -> Token
    {
        self.internal_expect_one_of(tokens, |token_list| panic!(format!("expected one of: {}", util::build_list_str(token_list.iter())))).unwrap()
    }
    
    /// Expects a token out of a set of tokens, calling a closure if the token isn't matched.
    fn internal_expect_one_of<I: Iterator<Token>>(&mut self, mut tokens: I, on_fail: |&Vec<Token>|) -> Result<Token,String>
    {
        let mut token_list = Vec::new();
        
        match self.next() {
            Some(next_tok) => {
                token_list.push(next_tok.clone());
                
                match tokens.find(|a| a == &next_tok) {
                    Some(tok) => {
                        return Ok(tok);
                    },
                    None => (),
                }
            },
            None => (),
        }
        
        on_fail(&token_list);
        Err(format!("expected one of: {}", util::build_list_str(token_list.iter())))
    }
}

impl<I: Iterator<char>> Iterator<Token> for Tokenizer<I>
{
    fn next(&mut self) -> Option<Token>
    {
        match self.stack.pop() {
            Some(tok) => { return Some(tok); },
            None => (),
        };
        
        self.it.eat_whitespace_but_line();
        
        let first_char = match self.it.peek() {
            Some(first_char) => first_char,
            None => panic!("unexpected end of file"),
        };
        
        if first_char == '\n' {
            self.it.next();
            return Some(Token::new_line());
        } else if first_char == '\r' {
            match self.it.peek_n(1) {
                Some('\n') => {
                    self.it.next(); // skip '\r'.
                    self.it.next(); // skip '\n'.
                    
                    return Some(Token::new_line());
                },
                Some(..) | None => ()
            }
        }
        
        if ast::expressions::identifier::is_valid_first_char(first_char) {
            self.read_identifier()
        } else {
            self.read_possible_symbol()
        }
    }
}

/// An iterator which can peek.
pub struct IteratorPeeker<T, U: Iterator<T>>
{
    it: U,
    stack: Vec<T>,
}

impl<T: Clone, U: Iterator<T>> IteratorPeeker<T, U>
{
    pub fn new(it: U) -> IteratorPeeker<T,U>
    {
        IteratorPeeker {
            it: it,
            stack: Vec::new(),
        }
    }
    
    pub fn peek(&mut self) -> Option<T>
    {
        let val = match self.stack.pop() {
            Some(val) => val.clone(),
            None => {
                match self.it.next() {
                    Some(val) => {
                        val
                    },
                    None => { return None; },
                }
            }
        };
        
        self.stack.push(val.clone());
        Some(val)
    }
    
    /// Peeks at the n'th character from the current index.
    pub fn peek_n(&mut self, n: uint) -> Option<T>
    {
        let mut read_elems = Vec::new();
        
        for _ in range(0,n+1) {
        
            match self.next() {
                Some(e) => {
                    read_elems.push(e);
                },
                None => {
                    break;
                },
            }
        }

        for read_char in read_elems.iter().rev() {
            self.stack.push(read_char.clone());
        }
        
        read_elems.last().map(|a| a.clone())
    }
    
    pub fn eat(&mut self, n: uint)
    {
        for _ in range(0, n) {
            match self.next() {
                Some(..) => (),
                None => { break }, // we reached the end, might as well stop.
            }
        }
    }
    
}

impl<T: std::fmt::Show + Clone, U: Iterator<T>> IteratorPeeker<T, U>
{
    
}

impl<T, U: Iterator<T>> Iterator<T> for IteratorPeeker<T, U>
{
    fn next(&mut self) -> Option<T>
    {
        match self.stack.pop() {
            Some(val) => Some(val),
            None => self.it.next()
        }
    }
}

impl<U: Iterator<char>> IteratorPeeker<char, U>
{
    pub fn eat_whitespace_but_line(&mut self)
    {
        loop {
            match self.next() {
                Some(val) => {
                    if val.is_whitespace() {
                    
                        if val == '\r' {
                            match self.peek() {
                                Some('\n') => {
                                    self.stack.push(val);
                                    break;
                                },
                                _ => (),
                            }
                        } else if val == '\n' {
                            self.stack.push(val);
                            break;
                        }
                    
                        continue;
                    } else {
                        self.stack.push(val);
                        break;
                    }
                },
                None => ()
            }
        }
    }
}

