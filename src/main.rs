mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;


fn main() {
    let sql = "SELECT * FROM users WHERE age > 18";
    let mut lexer = Lexer::new(sql);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            println!("Lexer error: {}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => println!("AST: {:#?}", ast),
        Err(e) => println!("Parser Error: {}", e),
    }

}