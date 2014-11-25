
pub use self::identifier::Identifier;

pub mod ast;
pub mod parser;
pub mod token;
pub mod util;
pub mod preprocessor;
pub mod identifier;




fn main()
{
    let text = "#define asdf 2321\n #if defined(asdf) \n #endif";
    
    //test_tokenizer(text);
    test_preprocessor(text);
    //test_parser(text);
}

#[allow(dead_code)]
fn test_tokenizer(text: &'static str)
{
    let tk1 = token::Tokenizer::new(text.chars());
    
    test_tokenizer_it(tk1);
}

#[allow(dead_code)]
fn test_tokenizer_it<I: Iterator<char>>(mut tk: token::Tokenizer<I>)
{
    for token in tk {
        println!("{}", token);
    }
}

fn test_preprocessor(text: &'static str)
{
    let tokenizer = token::Tokenizer::new(text.chars());
    
    test_preprocessor_tk(tokenizer);
}

fn test_preprocessor_tk<I: Iterator<char>>(tk: token::Tokenizer<I>)
{
    let mut p = preprocessor::Preprocessor::new();
    
    match p.preprocess(tk) {
        Ok(..) => println!("{}", p.blocks),
        Err(err) => println!("error: {}", err),
    }
    
    
}

#[allow(dead_code)]
fn test_parser(text: &'static str)
{
    let tk = token::Tokenizer::new(text.chars());
    let mut parser = parser::Parser::new();
    
    match parser.parse(tk) {
        Ok(..) => { println!("parse success!"); },
        Err(msg) => { println!("parsing failed: {}", msg); },
    }
    
    println!("{}", parser.ast);
}
