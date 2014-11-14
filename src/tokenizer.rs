
use ast;
use std;


/// A token.
#[deriving(Show,Clone,PartialEq,Eq)]
pub enum Token
{
    TokenSymbol(String),
    TokenWord(String),
    TokenIntegerLiteral(String),
    TokenStringLiteral(String),
    TokenNewLine,
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
    pub fn new(it: I) -> Tokenizer<I>
    {
        let mut symbol_tokens = vec![
            ";",
            "(", ")",
            
            // arithmetic operators.
            "+", "-", "*", "/",
            
            // arithmetic assignment operators.
            "+=", "-=", "*=", "/="
        ];
        
        // sort the symbol tokens by length, so that the longer symbols are at the beginning.
        symbol_tokens.sort_by(|&e1,&e2| e2.len().cmp(&e1.len()));
        
        println!("{}", symbol_tokens);
        
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
        
        Some(TokenWord(String::from_chars(chars.as_slice())))
    }
    
    fn read_possible_symbol(&mut self) -> Option<Token>
    {
        match self.it.next() {
            Some(c) => {
                let mut read_chars = String::from_char(1, c);
                
                for sym in self.symbol_tokens.iter() {
                    for (index,symbol_char) in sym.chars().enumerate() {
                        
                    }
                }
            },
            None => unreachable!(),
        };
        
        None
        /*
        match self.it.next() {
            Some(c) => match c {
                '#' => Some(TokenSymbol("#".to_string())),
                t => panic!(format!("unknown token: '{}'", t)),
            },
            None => None
        }*/
    }
    
    pub fn expect_assert(&mut self, tok: &Token)
    {
        match self.next() {
            Some(next_tok) => {
                if &next_tok != tok {
                    panic!(format!("expected {}", tok))
                }
            },
            None => (),
        }
    }
    
    pub fn next_word(&mut self) -> Result<String,String>
    {
        match self.next() {
            Some(tok) => match tok {
                TokenWord(word) => Ok(word),
                _ => Err("expected word".to_string())
            },
            None => Err("unexpected end of file".to_string()),
        }
    }
    
    pub fn peek_word(&mut self) -> Result<String,String>
    {
        match self.peek() {
            Some(tok) => match tok {
                TokenWord(word) => Ok(word),
                _ => Err("expected word".to_string())
            },
            None => Err("unexpected end of file".to_string()),
        }
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
            return Some(TokenNewLine);
        } else if first_char == '\r' {
            match self.it.peek_n(1) {
                Some('\n') => {
                    self.it.next(); // skip '\r'.
                    self.it.next(); // skip '\n'.
                    
                    return Some(TokenNewLine);
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
    pub fn peek_n(&mut self, n: u32) -> Option<T>
    {
        let mut read_elems = Vec::new();
        
        for i in range(0,n+1) {
        
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


