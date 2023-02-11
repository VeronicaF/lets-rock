use strum_macros::IntoStaticStr;
use crate::ast::Node;

macro_rules! enum_str {
    (enum $name:ident {
        $($variant:ident = $val:expr),*,
    }) => {
        enum $name {
            $($variant = $val),*
        }

        impl $name {
            fn name(&self) -> &'static str {
                match self {
                    $($name::$variant => stringify!($variant)),*
                }
            }
        }
    };
}

#[derive(Clone, Debug, PartialEq, Eq, IntoStaticStr)]
pub enum Token {
    Illegal,
    // 标识符
    Ident(String),
    // 字面量
    Int(String),
    // 运算符
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    LT,
    GT,
    // 分隔符
    Comma,
    Semi,
    LParen,
    RParen,
    LBrace,
    RBrace,
    // 关键字
    Function,
    Let,
    If,
    Else,
    Return,
    True,
    False,
    Eq,
    NotEq,
}

impl Node for Token {
    fn token_literal(&self) -> String {
        match self {
            Token::Illegal => "",
            Token::Ident(str) => str,
            Token::Int(str) => str,
            Token::Assign => "=",
            Token::Plus => "+",
            Token::Minus => "-",
            Token::Bang => "!",
            Token::Asterisk => "*",
            Token::Slash => "/",
            Token::LT => "<",
            Token::GT => ">",
            Token::Comma => ",",
            Token::Semi => ";",
            Token::LParen => "(",
            Token::RParen => ")",
            Token::LBrace => "{",
            Token::RBrace => "}",
            Token::Function => "fn",
            Token::Let => "let",
            Token::If => "if",
            Token::Else => "else",
            Token::Return => "return",
            Token::True => "true",
            Token::False => "false",
            Token::Eq => "==",
            Token::NotEq => "!=",
        }
        .into()
    }
}
