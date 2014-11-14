
pub mod ast;
pub mod parser;
pub mod tokenizer;


fn main()
{
    let text = "#define _SFR_IO8(addr) addr";
    
    //test_tokenizer(text);
    test_parser(text);
}

#[allow(dead_code)]
fn test_tokenizer(text: &'static str)
{
    let mut tk = tokenizer::Tokenizer::new(text.chars());
    
    for token in tk {
        println!("{}", token);
    }
}

#[allow(dead_code)]
fn test_parser(text: &'static str)
{
    let tk = tokenizer::Tokenizer::new(text.chars());
    let mut parser = parser::Parser::new();
    
    match parser.parse(tk) {
        Ok(..) => { println!("parse success!"); },
        Err(msg) => { println!("parsing failed: {}", msg); },
    }
    
    println!("{}", parser.ast);
}
