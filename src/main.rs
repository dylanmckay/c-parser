
pub mod ast;
pub mod parser;
pub mod token;
pub mod util;


fn main()
{
    let text = "#define asdf 2321\n";
    
    //test_tokenizer(text);
    test_parser(text);
}

#[allow(dead_code)]
fn test_tokenizer(text: &'static str)
{
    let mut tk = token::Tokenizer::new(text.chars());
    
    for token in tk {
        println!("{}", token);
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
