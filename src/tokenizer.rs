
use ast;


/// A token.
#[deriving(Show,Clone,PartialEq,Eq)]
pub enum Token
{
    TokenSymbol(String),
    TokenWord(String),
    TokenIntegerLiteral(String),
    TokenStringLiteral(String),
}

/// A tokenizer.
pub struct Tokenizer<I: Iterator<char>>
{
    it: IteratorPeeker<char, I>,
    stack: Vec<Token>,
}

impl<I: Iterator<char>> Tokenizer<I>
{
    pub fn new(it: I) -> Tokenizer<I>
    {
        Tokenizer {
            it: IteratorPeeker::new(it),
            stack: Vec::new(),
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
            
            if ast::is_valid_identifier_char(c) {

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
            Some(c) => match c {
                '#' => Some(TokenSymbol("#".to_string())),
                t => panic!(format!("unknown token: '{}'", t)),
            },
            None => None
        }
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
        
        self.it.eat_whitespace();
        
        let first_char = match self.it.peek() {
            Some(first_char) => first_char,
            None => panic!("unexpected end of file"),
        };
        
        if ast::is_valid_first_identifier_char(first_char) {
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
    pub fn eat_whitespace(&mut self)
    {
        loop {
            match self.next() {
                Some(val) => {
                    if val.is_whitespace() {
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


