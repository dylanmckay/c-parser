
pub mod ast;
pub mod parser;
pub mod tokenizer;


fn main()
{
    let text = "#define hello #define asdf";
    let tk = tokenizer::Tokenizer::new(text.chars());
    let mut parser = parser::Parser::new();
    
    parser.parse(tk);
    
}
