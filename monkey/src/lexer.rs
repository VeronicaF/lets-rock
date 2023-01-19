use crate::token::{Token, TokenType};

#[derive(Default, Debug)]
pub struct Lexer {
    _raw_input: String,
    chars: Vec<char>,
    position: usize,      // 当前字符位置
    read_position: usize, // 当前字符下一个位置
    char: Option<char>,   // 表示到达文件尾或者还没开始
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            chars: input.chars().collect(),
            _raw_input: input,
            ..Default::default()
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let ch = self.char?;
        let mut token = Token {
            token_type: TokenType::Illegal,
            literal: ch.to_string(),
        };

        match ch {
            '=' => {
                if let Some('=') = self.peek_next() {
                    self.read_char();
                    token.token_type = TokenType::Eq;
                    token.literal = "==".to_string();
                } else {
                    token.token_type = TokenType::Assign;
                }
            }
            '+' => {
                token.token_type = TokenType::Plus;
            }
            '-' => {
                token.token_type = TokenType::Minus;
            }
            '!' => {
                if let Some('=') = self.peek_next() {
                    self.read_char();
                    token.token_type = TokenType::NotEq;
                    token.literal = "!=".to_string();
                } else {
                    token.token_type = TokenType::Bang;
                }
            }
            '*' => {
                token.token_type = TokenType::Asterisk;
            }
            '/' => {
                token.token_type = TokenType::Slash;
            }
            '<' => {
                token.token_type = TokenType::LT;
            }
            '>' => {
                token.token_type = TokenType::GT;
            }
            ',' => {
                token.token_type = TokenType::Comma;
            }
            ';' => {
                token.token_type = TokenType::Semi;
            }
            '(' => {
                token.token_type = TokenType::LParen;
            }
            ')' => {
                token.token_type = TokenType::RParen;
            }
            '{' => {
                token.token_type = TokenType::LBrace;
            }
            '}' => {
                token.token_type = TokenType::RBrace;
            }
            //
            _ => {
                if ch.is_ascii_digit() {
                    token.literal = self.read_int();
                    token.token_type = TokenType::Int;
                    return Some(token);
                } else if ch.is_alphabetic() {
                    token.literal = self.read_ident();
                    token.token_type = Lexer::lookup_ident(&token.literal);
                    return Some(token);
                }
            }
        }
        self.read_char();
        Some(token)
    }

    /**
        将下一个 char 读到 self.char，并更新位置
    */
    fn read_char(&mut self) {
        if self.read_position >= self.chars.len() {
            self.char = None;
        } else {
            self.char = Some(self.chars[self.read_position]);
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.char.is_some_and(|c| c.is_ascii_digit()) {
            self.read_char();
        }
        self.chars[position..self.position].iter().collect()
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;
        while self.char.is_some_and(|c| c.is_alphanumeric()) {
            self.read_char();
        }
        self.chars[position..self.position].iter().collect()
    }

    fn skip_whitespace(&mut self) {
        while self.char.is_some() && self.char.unwrap().is_whitespace() {
            self.read_char();
        }
    }

    fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "let" => TokenType::Let,
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "return" => TokenType::Return,
            "true" => TokenType::True,
            "false" => TokenType::False,
            _ => TokenType::Ident,
        }
    }

    fn peek_next(&self) -> Option<char> {
        if self.read_position >= self.chars.len() {
            None
        } else {
            Some(self.chars[self.read_position])
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lexer::Lexer;
    use crate::token::TokenType;

    #[test]
    fn test_lexer1() {
        let input = std::fs::read_to_string("monkey/00.monkey").unwrap();
        let tests = [
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "five".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Int, "5".to_string()),
            (TokenType::Semi, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "ten".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Int, "10".to_string()),
            (TokenType::Semi, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "add".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Function, "fn".to_string()),
            (TokenType::LParen, "(".to_string()),
            (TokenType::Ident, "x".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Ident, "y".to_string()),
            (TokenType::RParen, ")".to_string()),
            (TokenType::LBrace, "{".to_string()),
            (TokenType::Ident, "x".to_string()),
            (TokenType::Plus, "+".to_string()),
            (TokenType::Ident, "y".to_string()),
            (TokenType::Semi, ";".to_string()),
            (TokenType::RBrace, "}".to_string()),
            (TokenType::Semi, ";".to_string()),
            (TokenType::Let, "let".to_string()),
            (TokenType::Ident, "result".to_string()),
            (TokenType::Assign, "=".to_string()),
            (TokenType::Ident, "add".to_string()),
            (TokenType::LParen, "(".to_string()),
            (TokenType::Ident, "five".to_string()),
            (TokenType::Comma, ",".to_string()),
            (TokenType::Ident, "ten".to_string()),
            (TokenType::RParen, ")".to_string()),
            (TokenType::Semi, ";".to_string()),
        ];

        let mut lexer = Lexer::new(input);

        for (expected_type, expected_literal) in tests {
            let t = lexer.next_token().unwrap();
            assert_eq!(expected_type, t.token_type, "token type not match");
            assert_eq!(expected_literal, t.literal, "literal not match");
        }
    }
    #[test]
    fn test_lexer2() {
        let input = std::fs::read_to_string("monkey/01.monkey").unwrap();
        let tests = [
            (TokenType::Let, "let"),
            (TokenType::Ident, "five"),
            (TokenType::Assign, "="),
            (TokenType::Int, "5"),
            (TokenType::Semi, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "ten"),
            (TokenType::Assign, "="),
            (TokenType::Int, "10"),
            (TokenType::Semi, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "add"),
            (TokenType::Assign, "="),
            (TokenType::Function, "fn"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "x"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "y"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Ident, "x"),
            (TokenType::Plus, "+"),
            (TokenType::Ident, "y"),
            (TokenType::Semi, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Semi, ";"),
            (TokenType::Let, "let"),
            (TokenType::Ident, "result"),
            (TokenType::Assign, "="),
            (TokenType::Ident, "add"),
            (TokenType::LParen, "("),
            (TokenType::Ident, "five"),
            (TokenType::Comma, ","),
            (TokenType::Ident, "ten"),
            (TokenType::RParen, ")"),
            (TokenType::Semi, ";"),
            (TokenType::Bang, "!"),
            (TokenType::Minus, "-"),
            (TokenType::Slash, "/"),
            (TokenType::Asterisk, "*"),
            (TokenType::Int, "5"),
            (TokenType::Semi, ";"),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::GT, ">"),
            (TokenType::Int, "5"),
            (TokenType::Semi, ";"),
            (TokenType::If, "if"),
            (TokenType::LParen, "("),
            (TokenType::Int, "5"),
            (TokenType::LT, "<"),
            (TokenType::Int, "10"),
            (TokenType::RParen, ")"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::True, "true"),
            (TokenType::Semi, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Else, "else"),
            (TokenType::LBrace, "{"),
            (TokenType::Return, "return"),
            (TokenType::False, "false"),
            (TokenType::Semi, ";"),
            (TokenType::RBrace, "}"),
            (TokenType::Int, "10"),
            (TokenType::Eq, "=="),
            (TokenType::Int, "10"),
            (TokenType::Semi, ";"),
            (TokenType::Int, "10"),
            (TokenType::NotEq, "!="),
            (TokenType::Int, "9"),
            (TokenType::Semi, ";"),
        ];

        let mut lexer = Lexer::new(input);

        for (expected_type, expected_literal) in tests {
            if let Some(t) = lexer.next_token() {
                assert_eq!(expected_type, t.token_type, "token type not match");
                assert_eq!(expected_literal, t.literal, "literal not match");
            }
        }
    }
}
