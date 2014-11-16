
use std;

/// An iterator which supports peeking.
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
    
    /// Peeks at the n'th object from the current position.
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
    
    pub fn eat(&mut self)
    {
        self.next();
    }
    
    pub fn eat_several(&mut self, n: uint)
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
                None => break,
            }
        }
    }
}
