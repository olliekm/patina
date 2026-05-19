use crate::lexer::Token;

// 1. AST types (what the parser produces)
#[derive(Debug, Clone)]
pub struct SelectStatement {
    /// Specifies which columns are selected in the statement.
    pub columns: ColumnSelection,
    /// The table from which to select.
    pub table: String,
    /// Optional WHERE clause for filtering results.
    pub where_clause: Option<WhereClause>,
}
#[derive(Debug, Clone)]
pub struct WhereClause {
    pub column: String,
    pub operator: Operator,
    /// The value to compare the column against.
    pub value: Value,
}
#[derive(Debug, Clone)]
pub enum ColumnSelection {
    All,
    Specific(Vec<String>),
}
#[derive(Debug, Clone)]
pub enum Operator {
    Equals,
    GreaterThan,
    LessThan,
}
#[derive(Debug, Clone)]
pub enum Value {
    Number(i64),
    String(String),
} 

// 2. Parser (the tools to create AST)
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    pub fn parse(&mut self) -> Result<SelectStatement, String> {
        self.expect(&Token::Select)?;

        let columns = self.parse_columns()?;

        self.expect(&Token::From)?;

        let table = self.parse_table_name()?;

        let where_clause = if self.current_token() == &Token::Where {
            Some(self.parse_where_clause()?)
        } else {
            None
        };

        Ok(SelectStatement {
            columns,
            table,
            where_clause,
        })
    }

    fn current_token(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn expect(&mut self, expected: &Token) -> Result<(), String> {
        if self.current_token() == expected {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "At position {}: Expected {:?}, got {:?}",
                self.position,
                expected,
                self.current_token()))
        }
    }

    fn parse_columns(&mut self) -> Result<ColumnSelection, String> {
        match self.current_token() {
            Token::Asterisk => {
                self.advance();
                Ok(ColumnSelection::All)
            }
            Token::Identifier(_) => {
                self.parse_columns_list()
            }
            _ => Err(format!("Expected * or column name, got {:?}", self.current_token()))
        }
    }

    fn parse_columns_list(&mut self) -> Result<ColumnSelection, String> {
        let mut columns = Vec::new();

        loop {
            match self.current_token() {
                Token::Identifier(name) => {
                    columns.push(name.clone());
                    self.advance();
                }
                _ => return Err("Expected columns name".to_string())
            }

            if self.current_token() == &Token::Comma {
                self.advance();
                continue;
            } else {
                break;
            }
        }
        
        Ok(ColumnSelection::Specific(columns))
    }

    fn parse_table_name(&mut self) -> Result<String, String> {
        match self.current_token() {
            Token::Identifier(name) => {
                let table = name.clone();
                self.advance();
                Ok(table)
            }
            _ => Err(format!("Expected table name, got {:?}", self.current_token()))
        }
    }

    fn parse_where_clause(&mut self) -> Result<WhereClause, String> {
        self.advance();

        let column = match self.current_token() {
            Token::Identifier(name) => name.clone(),
            _ => return Err(format!("Expected column name, got {:?}", self.current_token()))
        };
        self.advance();

        let operator = match self.current_token() {
            Token::Equals => Operator::Equals,
            Token::GreaterThan => Operator::GreaterThan,
            Token::LessThan => Operator::LessThan,
            _ => return Err(format!("Expected operator, got {:?}", self.current_token()))
        };
        self.advance();

        let value = match self.current_token() {
            Token::Number(n) => Value::Number(*n), 
            Token::StringLiteral(s) => Value::String(s.clone()),
            _ => return Err(format!("Expected a value, got {:?}", self.current_token()))
        };
        self.advance();

        Ok(WhereClause { column, operator, value })
    }
}