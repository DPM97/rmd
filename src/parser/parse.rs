/*
 HEADING

 
 let mut count: u8 = 1;
                let mut j = i + 1;
                while j < input.len() && input[j] as char == '#' && count < 6 {
                    count += 1;
                    j += 1;
                }

                tokens.push(Token {
                    kind: match count {
                        1 => TokenType::Heading1,
                        2 => TokenType::Heading2,
                        3 => TokenType::Heading3,
                        4 => TokenType::Heading4,
                        5 => TokenType::Heading5,
                        6 => TokenType::Heading6,
                        _ => panic!("1 <= count <= 6"),
                    },
                    value: None,
                });
*/
