
use ast;

#[deriving(Show)]
pub enum Token
{
    TokenSymbol(String),
    TokenWord(String),
    TokenIntegerLiteral(String),
    TokenStringLiteral(String),
}

pub struct Tokenizer<I: Iterator<char>>
{
    it: IteratorPeeker<char, I>,
}

impl<I: Iterator<char>> Tokenizer<I>
{
    pub fn new(it: I) -> Tokenizer<I>
    {
        Tokenizer {
            it: IteratorPeeker::new(it),
        }
    }
    
    pub fn read_token(&mut self) -> Option<Token>
    {
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
        
        Some(TokenWord(chars.to_string()))
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
}

impl<I: Iterator<char>> Iterator<Token> for Tokenizer<I>
{
    fn next(&mut self) -> Option<Token>
    {
        self.read_token()
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
            Some(val) => val,
            None => {
                match self.it.next() {
                    Some(val) => val,
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


