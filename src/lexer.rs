use crate::token::{Token, TokenType};
use std::collections::HashMap;
use std::str;

#[derive(Clone)]
pub struct Lexer<'a> {
    pub input: &'a str,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
    pub keywords: HashMap<&'static str, TokenType>,
    pub end: bool,
}

impl Lexer<'_> {
    pub fn init_lexer<'a>(input: &'a str) -> Lexer<'a> {
        let key = generate_keywords();
        Lexer {
            input: input,
            position: 0,
            read_position: 1,
            ch: input.as_bytes()[0] as char,
            keywords: key,
            end: false,
        }
    }
    pub fn read_char(&mut self) -> () {
        if self.read_position >= self.input.len() {
            self.end = true;
            return;
        } else {
            self.ch = self.input.as_bytes()[self.read_position] as char;
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let mut tok: Token = Token {
            type_: TokenType::UNDEFINED,
            literal: "".to_string(),
        };
        if self.end == true {
            return new_token(TokenType::EOF, "\0");
        }

        self.skip_whitespace();
        match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = new_token(TokenType::EQ, "==")
                } else {
                    tok = new_token(TokenType::ASSIGN, "=")
                }
            }
            '-' => tok = new_token(TokenType::MINUS, "-"),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    tok = new_token(TokenType::NEQ, "!=");
                } else {
                    tok = new_token(TokenType::BANG, "!");
                }
            }
            '/' => tok = new_token(TokenType::SLASH, "/"),
            '*' => tok = new_token(TokenType::ASTERICK, "*"),
            '<' => tok = new_token(TokenType::LT, "<"),
            '>' => tok = new_token(TokenType::GT, ">"),
            ';' => tok = new_token(TokenType::SEMICOLON, ";"),
            '(' => tok = new_token(TokenType::LPAREN, "("),
            ')' => tok = new_token(TokenType::RPAREN, ")"),
            ',' => tok = new_token(TokenType::COMMA, ","),
            '+' => tok = new_token(TokenType::PLUS, "+"),
            '{' => tok = new_token(TokenType::LBRACE, "{"),
            '}' => tok = new_token(TokenType::RBRACE, "}"),
            '"' => {
                tok.type_ = TokenType::STRING;
                tok.literal = self.read_string();
            }
            _default => {
                if self.ch.is_alphabetic() {
                    tok.literal = self.read_identifier();
                    tok.type_ = self.lookup_identifier(tok.literal.as_str());
                } else if self.is_num() {
                    tok.literal = self.read_number();
                    tok.type_ = TokenType::INT;
                } else {
                    tok = Token {
                        type_: TokenType::ILLEGAL,
                        literal: self.ch.to_string(),
                    };
                }
            }
        }
        self.read_char();
        tok
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while self.ch.is_ascii_alphanumeric() {
            self.read_char();
        }
        let buf = &(self.input.as_bytes()[pos..self.position]);
        let x = std::str::from_utf8(buf);
        self.position -= 1;
        self.read_position -= 1;
        match x {
            Ok(s) => return s.to_string(),
            Err(err) => panic!("Paniced read_identifier: {}", err),
        }
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;
        while self.is_num() {
            self.read_char();
        }
        let buf = &(self.input.as_bytes()[pos..self.position]);
        let x = std::str::from_utf8(buf);
        self.position -= 1;
        self.read_position -= 1;
        match x {
            Ok(s) => return s.to_string(),
            Err(err) => panic!("Paniced read_number: {}", err),
        }
    }

    fn read_string(&mut self) -> String {
        let position = self.position + 1;
        self.read_char();
        loop {
            if self.ch == '"' || self.end {
                break;
            }
            self.read_char();
        }
        let buf = &(self.input.as_bytes()[position..self.position]);
        let x = std::str::from_utf8(buf);
        //INFO: the below were removed since the in the next_token function
        //after the match statement it would read_char and the end quote would be consumed
        //self.position -= 1;
        //self.read_position -= 1;
        match x {
            Ok(s) => return s.to_string(),
            Err(err) => panic!("Paniced read_string: {}", err),
        }
    }

    fn is_num(&mut self) -> bool {
        return '0' <= self.ch && self.ch <= '9';
    }

    fn lookup_identifier(&mut self, indent: &str) -> TokenType {
        let res: TokenType;
        let ans = self.keywords.get(indent);
        match ans {
            Some(x) => res = *x,
            None => res = TokenType::IDENT,
        }
        res
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            return '\0';
        } else {
            return self.input.as_bytes()[self.read_position] as char;
        }
    }
}

fn new_token<'a>(token_type: TokenType, literal: &'a str) -> Token {
    Token {
        type_: token_type,
        literal: literal.to_string(),
    }
}

fn generate_keywords() -> HashMap<&'static str, TokenType> {
    let keywords = HashMap::from([
        ("fn", TokenType::FUNCTION),
        ("let", TokenType::LET),
        ("true", TokenType::TRUE),
        ("false", TokenType::FALSE),
        ("if", TokenType::IF),
        ("else", TokenType::ELSE),
        ("return", TokenType::RETURN),
    ]);
    keywords
}

#[cfg(test)]
mod tests {
    use crate::lexer::*;
    #[test]
    fn test_string() {
        let s = "
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10;
        10 != 9;
        \"foobar\"; \"foo bar\";";
        let mut lex = Lexer::init_lexer(s);
        let mut ve = Vec::<Token>::new();
        let mut i = 0;
        //let mut tok: Token;
        while !lex.end {
            //print!("{} {} \n", lex.read_position, lex.input.len());
            ve.push(lex.next_token());
            println!("{:?}", ve[i]);
            i += 1;
        }
    }

    #[test]
    fn test_function_next_token_for_single_char() {
        let s = "=+(){},;";
        let mut lex = Lexer::init_lexer(s);
        let mut i = 0;
        let mut ve = Vec::<Token>::new();
        while i < lex.input.len() {
            ve.push(lex.next_token());
            i += 1;
        }
        ve.push(Token {
            type_: TokenType::EOF,
            literal: "EOF".to_string(),
        });
        //println!("{:?}", ve);
        for index in 0..lex.input.len() {
            // println!("{}", lex.input.as_bytes()[index as usize]as char );
            match lex.input.as_bytes()[index as usize] as char {
                '=' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::ASSIGN, true);
                    assert_eq!(ve[index as usize].literal == "=", true);
                }
                '+' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::PLUS, true);
                    assert_eq!(ve[index as usize].literal == "+", true);
                }
                '(' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::LPAREN, true);
                    assert_eq!(ve[index as usize].literal == "(", true)
                }
                ')' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::RPAREN, true);
                    assert_eq!(ve[index as usize].literal == ")", true)
                }
                '{' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::LBRACE, true);
                    assert_eq!(ve[index as usize].literal == "{", true)
                }
                '}' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::RBRACE, true);
                    assert_eq!(ve[index as usize].literal == "}", true)
                }
                ',' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::COMMA, true);
                    assert_eq!(ve[index as usize].literal == ",", true)
                }
                ';' => {
                    assert_eq!(ve[index as usize].type_ == TokenType::SEMICOLON, true);
                    assert_eq!(ve[index as usize].literal == ";", true)
                }
                _default => {
                    assert_eq!(ve[index as usize].type_ == TokenType::EOF, true);
                    assert_eq!(ve[index as usize].literal == "\0", true)
                }
            }
        }
        // let tok = &ve[ve.len() - 1];
        // assert_eq!(tok.type_ == TokenType::EOF, true);
        // assert_eq!(tok.literal == "\0", true);
    }

    #[test]
    fn test_identifiers() {
        let s = "let five = 5; let ten = 10\r 
        ;let add = fn (x , y ) { x + y ;}; \n
        let result = add (five , ten ); ";
        let mut lex = Lexer::init_lexer(s);
        let mut ve = Vec::<Token>::new();
        let mut tok: Token;
        //let mut i = 0;
        while lex.read_position < lex.input.len() {
            //print!("{} {} \n", lex.read_position, lex.input.len());
            //println!("{}", lex.ch);
            ve.push(lex.next_token());
            //println!("{}", lex.ch);
            //println!("{:?}", ve[i]);
            //i += 1;
        }
        ve.push(Token {
            type_: TokenType::EOF,
            literal: "\0".to_string(),
        });
        // for item in &ve {
        //     println!("{:?}", item);
        // }

        for index in 0..ve.len() {
            // println!("{:?}", ve[index]);
            match index {
                0 => {
                    tok = Token {
                        type_: TokenType::LET,
                        literal: "let".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                1 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "five".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                2 => {
                    tok = Token {
                        type_: TokenType::ASSIGN,
                        literal: "=".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                3 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "5".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                4 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                5 => {
                    tok = Token {
                        type_: TokenType::LET,
                        literal: "let".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                6 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "ten".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                7 => {
                    tok = Token {
                        type_: TokenType::ASSIGN,
                        literal: "=".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                8 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "10".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                9 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                10 => {
                    tok = Token {
                        type_: TokenType::LET,
                        literal: "let".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                11 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "add".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                12 => {
                    tok = Token {
                        type_: TokenType::ASSIGN,
                        literal: "=".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                13 => {
                    tok = Token {
                        type_: TokenType::FUNCTION,
                        literal: "fn".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                14 => {
                    tok = Token {
                        type_: TokenType::LPAREN,
                        literal: "(".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                15 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "x".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                16 => {
                    tok = Token {
                        type_: TokenType::COMMA,
                        literal: ",".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                17 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "y".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                18 => {
                    tok = Token {
                        type_: TokenType::RPAREN,
                        literal: ")".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                19 => {
                    tok = Token {
                        type_: TokenType::LBRACE,
                        literal: "{".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                20 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "x".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                21 => {
                    tok = Token {
                        type_: TokenType::PLUS,
                        literal: "+".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                22 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "y".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                23 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                24 => {
                    tok = Token {
                        type_: TokenType::RBRACE,
                        literal: "}".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                25 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                26 => {
                    tok = Token {
                        type_: TokenType::LET,
                        literal: "let".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                27 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "result".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                28 => {
                    tok = Token {
                        type_: TokenType::ASSIGN,
                        literal: "=".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                29 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "add".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                30 => {
                    tok = Token {
                        type_: TokenType::LPAREN,
                        literal: "(".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                31 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "five".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                32 => {
                    tok = Token {
                        type_: TokenType::COMMA,
                        literal: ",".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                33 => {
                    tok = Token {
                        type_: TokenType::IDENT,
                        literal: "ten".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                34 => {
                    tok = Token {
                        type_: TokenType::RPAREN,
                        literal: ")".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                35 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                36 => {
                    tok = Token {
                        type_: TokenType::EOF,
                        literal: "\0".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                _default => {
                    panic!("unknown token")
                } //_ => panic!("out bounds for vector")
            }
            // println!("{:?}", tok);
        }
    }

    #[test]
    fn test_new_identifiers() {
        //let mut i = 0;
        let s = "
        !-/*5;
        5 < 10 > 5;
        if (5 < 10) {
            return true;
        } else {
            return false;
        }
        10 == 10;
        10 != 9;";
        let mut lex = Lexer::init_lexer(s);
        let mut ve = Vec::<Token>::new();
        let mut tok: Token;
        while !lex.end {
            //print!("{} {} \n", lex.read_position, lex.input.len());
            ve.push(lex.next_token());
            // println!("{:?} \n", ve[i]);
            //i += 1;
        }
        ve.push(Token {
            type_: TokenType::EOF,
            literal: "\0".to_string(),
        });

        //print!("{}\n", ve.len());
        // for item in &ve {
        //     println!(
        //         "{}, token literal '{}'",
        //         item.type_.to_string(),
        //         item.literal
        //     );
        // }
        print!("{ }\n", ve[ve.len() - 1].type_.to_string());
        for index in 0..ve.len() - 1 {
            // println!("{:?}", ve[index]);
            match index {
                0 => {
                    tok = Token {
                        type_: TokenType::BANG,
                        literal: "!".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true);
                }
                1 => {
                    tok = Token {
                        type_: TokenType::MINUS,
                        literal: "-".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true);
                }
                2 => {
                    tok = Token {
                        type_: TokenType::SLASH,
                        literal: "/".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true);
                }
                3 => {
                    tok = Token {
                        type_: TokenType::ASTERICK,
                        literal: "*".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true);
                }
                4 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "5".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true);
                }
                5 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true);
                }
                6 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "5".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true);
                }
                7 => {
                    tok = Token {
                        type_: TokenType::LT,
                        literal: "<".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                8 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "10".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                9 => {
                    tok = Token {
                        type_: TokenType::GT,
                        literal: ">".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                10 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "5".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                11 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                12 => {
                    tok = Token {
                        type_: TokenType::IF,
                        literal: "if".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                13 => {
                    tok = Token {
                        type_: TokenType::LPAREN,
                        literal: "(".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                14 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "5".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                15 => {
                    tok = Token {
                        type_: TokenType::LT,
                        literal: "<".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                16 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "10".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                17 => {
                    tok = Token {
                        type_: TokenType::RPAREN,
                        literal: ")".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                18 => {
                    tok = Token {
                        type_: TokenType::LBRACE,
                        literal: "{".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                19 => {
                    tok = Token {
                        type_: TokenType::RETURN,
                        literal: "return".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                20 => {
                    tok = Token {
                        type_: TokenType::TRUE,
                        literal: "true".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                21 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                22 => {
                    tok = Token {
                        type_: TokenType::RBRACE,
                        literal: "}".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                23 => {
                    tok = Token {
                        type_: TokenType::ELSE,
                        literal: "else".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                24 => {
                    tok = Token {
                        type_: TokenType::LBRACE,
                        literal: "{".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                25 => {
                    tok = Token {
                        type_: TokenType::RETURN,
                        literal: "return".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                26 => {
                    tok = Token {
                        type_: TokenType::FALSE,
                        literal: "false".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                27 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                28 => {
                    tok = Token {
                        type_: TokenType::RBRACE,
                        literal: "}".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                29 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "10".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                30 => {
                    tok = Token {
                        type_: TokenType::EQ,
                        literal: "==".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                31 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "10".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                32 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                33 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "10".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                34 => {
                    tok = Token {
                        type_: TokenType::NEQ,
                        literal: "!=".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                35 => {
                    tok = Token {
                        type_: TokenType::INT,
                        literal: "9".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                36 => {
                    tok = Token {
                        type_: TokenType::SEMICOLON,
                        literal: ";".to_string(),
                    };
                    assert_eq!(ve[index] == tok, true)
                }
                37 => {
                    tok = Token {
                        type_: TokenType::EOF,
                        literal: "\0".to_string(),
                    };
                    //print!("\t{:?}", ve[index]);
                    assert_eq!(ve[index] == tok, true);
                }
                _ => {
                    panic!("unknown token")
                } //_ => panic!("out bounds for vector")
            }
            // println!("{:?}", tok);
        }
    }
}
