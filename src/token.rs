use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub struct Token {
    pub type_: TokenType,
    pub literal: String,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    IDENT,
    INT,
    ASSIGN,
    PLUS,
    MINUS,
    BANG,
    ASTERICK,
    SLASH,
    EQ,
    NEQ,
    LT,
    GT,
    TRUE,
    FALSE,
    IF,
    ELSE,
    RETURN,
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    FUNCTION,
    LET,
    UNDEFINED,
    STRING,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TokenType::ILLEGAL => write!(f, "TokenType: ILLEGAL"),
            TokenType::EOF => write!(f, "TokenType: EOF"),
            TokenType::IDENT => write!(f, "TokenType: IDENT"),
            TokenType::INT => write!(f, "TokenType: INT"),
            TokenType::ASSIGN => write!(f, "TokenType: ASSIGN"),
            TokenType::PLUS => write!(f, "TokenType: PLUS"),
            TokenType::MINUS => write!(f, "TokenType: MINUS"),
            TokenType::BANG => write!(f, "TokenType: BANG"),
            TokenType::ASTERICK => write!(f, "TokenType: ASTERISK"),
            TokenType::SLASH => write!(f, "TokenType: SLASH"),
            TokenType::EQ => write!(f, "TokenType: EQ"),
            TokenType::NEQ => write!(f, "TokenType: NEQ"),
            TokenType::LT => write!(f, "TokenType: LT"),
            TokenType::GT => write!(f, "TokenType: GT"),
            TokenType::TRUE => write!(f, "TokenType: TRUE"),
            TokenType::FALSE => write!(f, "TokenType: FALSE"),
            TokenType::IF => write!(f, "TokenType: IF"),
            TokenType::ELSE => write!(f, "TokenType: ELSE"),
            TokenType::RETURN => write!(f, "TokenType: RETURN"),
            TokenType::COMMA => write!(f, "TokenType: COMMA"),
            TokenType::SEMICOLON => write!(f, "TokenType: SEMICOLON"),
            TokenType::LPAREN => write!(f, "TokenType: LPAREN"),
            TokenType::RPAREN => write!(f, "TokenType: RPAREN"),
            TokenType::LBRACE => write!(f, "TokenType: LBRACE"),
            TokenType::RBRACE => write!(f, "TokenType: RBRACE"),
            TokenType::FUNCTION => write!(f, "TokenType: FUNCTION"),
            TokenType::LET => write!(f, "TokenType: LET"),
            TokenType::UNDEFINED => write!(f, "TokenType: UNDEFINED"),
            TokenType::STRING => write!(f, "TokenType: STRING"),
        }
    }
}
