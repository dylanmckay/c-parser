
pub mod ast;
pub mod parser;
pub mod tokenizer;


fn main()
{
    let text = "#define _SFR_IO8(addr) addr";
    
    //test_tokenizer(text);
    test_parser(text);
}

fn test_tokenizer(text: &'static str)
{
    let mut tk = tokenizer::Tokenizer::new(text.chars());
    
    for token in tk {
        println!("{}", token);
    }
}

fn test_parser(text: &'static str)
{
    let tk = tokenizer::Tokenizer::new(text.chars());
    let mut parser = parser::Parser::new();
    
    parser.parse(tk);
    
    println!("{}", parser.ast);
}
