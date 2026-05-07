
// We can print tokens, compare tokens, and clone them
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    // Keywords
    Select,
    From,
    Where,

    // Identifiers and literals
    Identifier(String),
    Number(i64),
    StringLiteral(String),

    // Operators
    Asterisk,
    Equals,
    GreaterThan,
    LessThan,

    // Punctuation
    Comma,
    Semicolon,

    Eof,
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize, // usize is used for indexing
}

impl Lexer {

    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();

        loop {
            let token = self.next_token()?;
            if token == Token::Eof {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }

        Ok(tokens)
    }


    fn next_token(&mut self) -> Result<Token, String> {
        while self.position < self.input.len() && self.input[self.position].is_whitespace() {
            self.position += 1;
        }

        if self.position >= self.input.len() {
            return Ok(Token::Eof);
        }

        let ch = self.input[self.position];

        match ch {
            '*' => {
                self.position += 1;
                Ok(Token::Asterisk)
            }
            ',' => {
                self.position += 1;
                Ok(Token::Comma)
            }
            ';' => {
                self.position += 1;
                Ok(Token::Semicolon)
            }
            '>' => {
                self.position += 1;
                Ok(Token::GreaterThan)
            }
            '<' => {
                self.position += 1;
                Ok(Token::LessThan)
            }
            '=' => {
                self.position += 1;
                Ok(Token::Equals)
            }
            _ if ch.is_alphabetic() => self.read_identifer_or_keyword(),
            _ if ch.is_ascii_digit() => self.read_number(),
            _ => Err(format!("Unexpected character: {}", ch))
        }
    }

    fn read_identifer_or_keyword(&mut self) -> Result<Token, String> {
        let mut str_rep = String::new();
        while self.position < self.input.len() {
            let ch = self.input[self.position];
            if ch.is_alphanumeric() || ch == '_' {
                str_rep.push(self.input[self.position]);
                self.position += 1;
            } else {
                break;
            }
        }
        
        let token = match str_rep.to_uppercase().as_str() {
            "SELECT" => Token::Select,
            "FROM" => Token::From,
            "WHERE" => Token::Where,
            _ => Token::Identifier(str_rep),
        };

        Ok(token)
    }

    fn read_number(&mut self) -> Result<Token, String> {
        let mut str_rep = String::new();
        while self.position < self.input.len() && self.input[self.position].is_ascii_digit() {
            str_rep.push(self.input[self.position]);
            self.position += 1;
        }
        str_rep.parse::<i64>()
            .map(Token::Number)
            .map_err(|_| format!("Invalid number: {}", str_rep))
    }


}