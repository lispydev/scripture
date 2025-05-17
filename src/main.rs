mod lexer;

fn main() {

    let hay = "123(  \t\n \"test\" ){ x = 1 }";
    let tokens = lexer::tokenize(hay);
    for token in tokens.iter() {
        println!("{:?}", token);
    }
    println!("Hello, world!");
}
