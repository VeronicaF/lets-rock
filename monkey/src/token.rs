#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Illegal,
    // 标识符
    Ident,
    // 字面量
    Int,
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

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}
