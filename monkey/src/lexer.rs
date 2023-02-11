use crate::token::Token;

#[derive(Default, Debug)]
pub struct Lexer {
    chars: Vec<char>,
    position: usize,      // 当前字符位置
    read_position: usize, // 当前字符下一个位置
    char: Option<char>,   // 表示到达文件尾或者还没开始
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let mut l = Lexer {
            chars: input.chars().collect(),
            ..Default::default()
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.skip_whitespace();
        let ch = self.char?;

        let mut flag = true;

        let token = match ch {
            '=' => {
                if let Some('=') = self.peek_next() {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '+' => Token::Plus,
            '-' => Token::Minus,
            '!' => {
                if let Some('=') = self.peek_next() {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '*' => Token::Asterisk,
            '/' => Token::Slash,
            '<' => Token::LT,
            '>' => Token::GT,
            ',' => Token::Comma,
            ';' => Token::Semi,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            //
            _ => {
                if ch.is_ascii_digit() {
                    let literal = self.read_int();
                    flag = false;
                    Token::Int(literal)
                } else if ch.is_alphabetic() {
                    let literal = self.read_ident();
                    flag = false;
                    Lexer::lookup_ident(&literal)
                } else {
                    Token::Illegal
                }
            }
        };
        if flag {
            self.read_char();
        }
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

    fn lookup_ident(ident: &str) -> Token {
        match ident {
            "let" => Token::Let,
            "fn" => Token::Function,
            "if" => Token::If,
            "else" => Token::Else,
            "return" => Token::Return,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident.into()),
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
    use crate::token::Token;

    #[test]
    fn test_lexer1() {
        let input = std::fs::read_to_string("monkey/00.monkey").unwrap();
        let tests = [
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semi,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semi,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semi,
            Token::RBrace,
            Token::Semi,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::LParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::RParen,
            Token::Semi,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in tests {
            let t = lexer.next_token().unwrap();
            assert_eq!(expected_token, t);
        }
    }
    #[test]
    fn test_lexer2() {
        let input = std::fs::read_to_string("monkey/01.monkey").unwrap();
        let tests = [
            Token::Let,
            Token::Ident("five".into()),
            Token::Assign,
            Token::Int("5".into()),
            Token::Semi,
            Token::Let,
            Token::Ident("ten".into()),
            Token::Assign,
            Token::Int("10".into()),
            Token::Semi,
            Token::Let,
            Token::Ident("add".into()),
            Token::Assign,
            Token::Function,
            Token::LParen,
            Token::Ident("x".into()),
            Token::Comma,
            Token::Ident("y".into()),
            Token::RParen,
            Token::LBrace,
            Token::Ident("x".into()),
            Token::Plus,
            Token::Ident("y".into()),
            Token::Semi,
            Token::RBrace,
            Token::Semi,
            Token::Let,
            Token::Ident("result".into()),
            Token::Assign,
            Token::Ident("add".into()),
            Token::LParen,
            Token::Ident("five".into()),
            Token::Comma,
            Token::Ident("ten".into()),
            Token::RParen,
            Token::Semi,
            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Int("5".into()),
            Token::Semi,
            Token::Int("5".into()),
            Token::LT,
            Token::Int("10".into()),
            Token::GT,
            Token::Int("5".into()),
            Token::Semi,
            Token::If,
            Token::LParen,
            Token::Int("5".into()),
            Token::LT,
            Token::Int("10".into()),
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::True,
            Token::Semi,
            Token::RBrace,
            Token::Else,
            Token::LBrace,
            Token::Return,
            Token::False,
            Token::Semi,
            Token::RBrace,
            Token::Int("10".into()),
            Token::Eq,
            Token::Int("10".into()),
            Token::Semi,
            Token::Int("10".into()),
            Token::NotEq,
            Token::Int("9".into()),
            Token::Semi,
        ];

        let mut lexer = Lexer::new(input);

        for expected_token in tests {
            let t = lexer.next_token().unwrap();
            assert_eq!(expected_token, t);
        }
    }
}
