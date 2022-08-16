use std::cell::RefCell;

#[derive(Debug)]
enum TokenType {
    Hash,
    String,
    Asterisk,
    Underscore,
    Whitespace,
    GraveAccent,
    Pipe,
    Newline,
    Dash,
    GreaterThan,
    Colon,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Exclamation,
    SingleQuote,
    DoubleQuote,
}

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    value: RefCell<String>,
}

pub fn tokenize(input: &Vec<u8>) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut i = 0;
    while i < input.len() {
        match input[i] as char {
            '#' => tokens.push(Token {
                r#type: TokenType::Hash,
                value: RefCell::new('#'.to_string()),
            }),
            '*' => tokens.push(Token {
                r#type: TokenType::Asterisk,
                value: RefCell::new('*'.to_string()),
            }),
            '_' => tokens.push(Token {
                r#type: TokenType::Underscore,
                value: RefCell::new('_'.to_string()),
            }),
            '`' => tokens.push(Token {
                r#type: TokenType::GraveAccent,
                value: RefCell::new('`'.to_string()),
            }),
            '|' => tokens.push(Token {
                r#type: TokenType::Pipe,
                value: RefCell::new('|'.to_string()),
            }),
            '-' => tokens.push(Token {
                r#type: TokenType::Dash,
                value: RefCell::new('-'.to_string()),
            }),
            '>' => tokens.push(Token {
                r#type: TokenType::GreaterThan,
                value: RefCell::new('>'.to_string()),
            }),
            ':' => tokens.push(Token {
                r#type: TokenType::Colon,
                value: RefCell::new(':'.to_string()),
            }),
            '!' => tokens.push(Token {
                r#type: TokenType::Exclamation,
                value: RefCell::new('!'.to_string()),
            }),
            '[' => tokens.push(Token {
                r#type: TokenType::LBracket,
                value: RefCell::new('['.to_string()),
            }),
            ']' => tokens.push(Token {
                r#type: TokenType::RBracket,
                value: RefCell::new(']'.to_string()),
            }),
            '(' => tokens.push(Token {
                r#type: TokenType::LParen,
                value: RefCell::new("\\(".to_string()),
            }),
            ')' => tokens.push(Token {
                r#type: TokenType::RParen,
                value: RefCell::new("\\)".to_string()),
            }),
            '\'' => tokens.push(Token {
                r#type: TokenType::SingleQuote,
                value: RefCell::new('\''.to_string()),
            }),
            '"' => tokens.push(Token {
                r#type: TokenType::DoubleQuote,
                value: RefCell::new('"'.to_string()),
            }),
            ' ' => tokens.push(Token {
                r#type: TokenType::Whitespace,
                value: RefCell::new(' '.to_string()),
            }),
            '\n' => tokens.push(Token {
                r#type: TokenType::Newline,
                value: RefCell::new('\n'.to_string()),
            }),
            // append to last token's value if string
            v => {
                let last = tokens.last();
                if last.is_some() && matches!(last.unwrap().r#type, TokenType::String) {
                    last.unwrap().value.borrow_mut().push(v);
                } else {
                    tokens.push(Token {
                        r#type: TokenType::String,
                        value: RefCell::new(v.to_string()),
                    });
                }
            }
        }
        i += 1;
    }
    return tokens;
}
