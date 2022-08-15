#[derive(Debug)]
enum TokenType {
    Heading1,
    Heading2,
    Heading3,
}

#[derive(Debug)]
struct Token {
    kind: TokenType,
    value: Option<String>,
}

pub fn parse(input: &Vec<u8>) {
    let mut tokens: Vec<Token> = vec![];
    let mut i = 0;
    while i < input.len() {
        match input[i] as char {
            '#' => {
                println!("{}", i);
                let mut count: u8 = 1;
                let mut j = i + 1;
                while j < input.len() && input[j] as char == '#' && count < 3 {
                    count += 1;
                    j += 1;
                }
                match count {
                    1 => tokens.push(Token {
                        kind: TokenType::Heading1,
                        value: None,
                    }),
                    2 => tokens.push(Token {
                        kind: TokenType::Heading2,
                        value: None,
                    }),
                    3 => tokens.push(Token {
                        kind: TokenType::Heading3,
                        value: None,
                    }),
                    _ => {}
                }
                i += (count - 1) as usize;
            }
            _ => panic!("invalid token!"),
        }
        i += 1;
    }

    println!("{:?}", tokens);
}
