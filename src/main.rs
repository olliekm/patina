use crate::lexer::Lexer;

mod lexer;
fn main() {
    let test = String::from("SELECT * FROM users WHERE age > 18");
    let mut lexer = Lexer::new(&test);
    
    match lexer.tokenize() {
        Ok(tokens) => println!("{:?}", tokens),
        Err(e) => println!("Error: {}", e),
    }

}