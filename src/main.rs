
pub mod ast;
pub mod parser;
pub mod tokenizer;


fn main()
{
    let text = "#define hello #define asdf";
    let mut tk = tokenizer::Tokenizer::new(text.chars());
    
    for token in tk {
        println!("{}", token);
    }
    
}
