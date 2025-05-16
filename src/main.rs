mod lexer;

fn main() {

    let hay = "123 ";
    let tokens = lexer::tokenize(hay);
    for token in tokens.iter() {
        println!("{:?}", token);
    }
    println!("Hello, world!");
}
