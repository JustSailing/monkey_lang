use crate::ast::*;
use crate::lexer::Lexer;
use crate::token::*;
use std::collections::HashMap;

const PARSE_IDENTIFIER: &str = "parse_identifier";
const PARSE_INTEGER_LITERAL: &str = "parse_integer_literal";
const PARSE_PREFIX_EXPR: &str = "parse_prefix_expr";
const PARSE_INFIX_EXPR: &str = "parse_infix_expr";
const PARSE_BOOLEAN: &str = "parse_boolean_expr";
const PARSE_GROUP: &str = "parse_group_expr";
const PARSE_IF: &str = "parse_if_expr";
const PARSE_FUNCTION: &str = "parse_function_literal";
const PARSE_CALL: &str = "parse_call";
const PARSE_STRING: &str = "parse_string";

const LOWEST: i32 = 0;
const EQUALS: i32 = 1;
const LESSGREATER: i32 = 2;
const SUM: i32 = 3;
const PRODUCT: i32 = 4;
const PREFIX: i32 = 5;
const CALL: i32 = 6;

pub struct Parser<'a> {
    lex: &'a mut Lexer<'a>,
    cur_token: Token,
    peek_token: Token,
    prefix_fns: HashMap<TokenType, &'a str>,
    infix_fns: HashMap<TokenType, &'a str>,
    precedence: HashMap<TokenType, i32>,
    errors: Vec<String>,
}

impl<'a> Parser<'a> {
    pub fn new(l: &'a mut Lexer<'a>) -> Parser<'a> {
        let mut p = Parser {
            lex: l,
            cur_token: Token {
                type_: TokenType::UNDEFINED,
                literal: "".to_string(),
            },
            peek_token: Token {
                type_: TokenType::UNDEFINED,
                literal: "".to_string(),
            },
            prefix_fns: HashMap::<TokenType, &str>::new(),
            infix_fns: HashMap::<TokenType, &str>::new(),
            precedence: HashMap::<TokenType, i32>::new(),
            errors: Vec::<String>::new(),
        };
        p.next_token();
        p.next_token();

        p.precedence.insert(TokenType::EQ, EQUALS);
        p.precedence.insert(TokenType::NEQ, EQUALS);
        p.precedence.insert(TokenType::LT, LESSGREATER);
        p.precedence.insert(TokenType::GT, LESSGREATER);
        p.precedence.insert(TokenType::PLUS, SUM);
        p.precedence.insert(TokenType::MINUS, SUM);
        p.precedence.insert(TokenType::SLASH, PRODUCT);
        p.precedence.insert(TokenType::ASTERICK, PRODUCT);
        p.precedence.insert(TokenType::LPAREN, CALL);

        p.register_prefix(TokenType::IDENT, PARSE_IDENTIFIER);
        p.register_prefix(TokenType::INT, PARSE_INTEGER_LITERAL);
        p.register_prefix(TokenType::BANG, PARSE_PREFIX_EXPR);
        p.register_prefix(TokenType::MINUS, PARSE_PREFIX_EXPR);
        p.register_prefix(TokenType::TRUE, PARSE_BOOLEAN);
        p.register_prefix(TokenType::FALSE, PARSE_BOOLEAN);
        p.register_prefix(TokenType::LPAREN, PARSE_GROUP);
        p.register_prefix(TokenType::IF, PARSE_IF);
        p.register_prefix(TokenType::FUNCTION, PARSE_FUNCTION);
        p.register_prefix(TokenType::STRING, PARSE_STRING);

        p.register_infix(TokenType::PLUS, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::MINUS, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::SLASH, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::ASTERICK, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::EQ, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::NEQ, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::LT, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::GT, PARSE_INFIX_EXPR);
        p.register_infix(TokenType::LPAREN, PARSE_CALL);

        p
    }
    pub fn next_token(&mut self) {
        self.cur_token = self.peek_token.clone();
        self.peek_token = self.lex.next_token();
    }

    pub fn register_prefix(&mut self, tok_type: TokenType, fun: &'a str) {
        self.prefix_fns.insert(tok_type, fun);
    }

    pub fn register_infix(&mut self, tok_type: TokenType, fun: &'a str) {
        self.infix_fns.insert(tok_type, fun);
    }

    fn cur_token_is(&mut self, t: TokenType) -> bool {
        self.cur_token.type_ == t
    }

    fn peek_token_is(&mut self, t: TokenType) -> bool {
        self.peek_token.type_ == t
    }

    fn expect_peek(&mut self, t: TokenType) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            return true;
        } else {
            self.peek_error(t);
            return false;
        }
    }

    pub fn errors(&mut self) -> Vec<String> {
        self.errors.clone()
    }

    fn peek_error(&mut self, tok: TokenType) {
        let message = format!(
            "expected next token to be {tok}, got {}",
            self.peek_token.type_
        );
        self.errors.push(message);
    }

    fn peek_precedence(&mut self) -> i32 {
        let prec = self.precedence.get(&self.peek_token.type_);
        match prec {
            Some(prec) => return prec.clone(),
            None => return LOWEST,
        }
    }

    fn cur_precedence(&mut self) -> i32 {
        let prec = self.precedence.get(&self.cur_token.type_);
        match prec {
            Some(prec) => return prec.clone(),
            None => return LOWEST,
        }
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program: Program = Program::new();
        let mut ct = self.cur_token.type_;
        while ct != TokenType::EOF && self.lex.position < self.lex.input.len() {
            let stmt: Option<Statement> = self.parse_statement();
            match stmt {
                Some(s) => program.push(s),
                _ => break,
            }
            self.next_token();
            ct = self.cur_token.type_;
        }
        return program;
    }

    fn parse_statement(&mut self) -> Option<Statement> {
        match self.cur_token.type_ {
            TokenType::LET => {
                let stmt: Option<Statement> = self.parse_let_statement();
                match stmt {
                    Some(s) => return Some(s),
                    None => return None,
                }
            }
            TokenType::RETURN => {
                let stmt = self.parse_return_statement();
                match stmt {
                    Some(s) => return Some(s),
                    None => return None,
                }
            }
            _default => {
                let stmt = self.parse_expression_statement();
                match stmt {
                    Some(s) => return Some(s),
                    None => return None,
                }
            }
        }
        // None
    }

    fn parse_let_statement(&mut self) -> Option<Statement> {
        let mut stmt = Statement::LetStatement(LetStatement {
            token: self.cur_token.clone(),
            name: None,
            value: None,
        });

        if !self.expect_peek(TokenType::IDENT) {
            return None;
        }
        match stmt {
            Statement::LetStatement(ref mut x) => {
                x.name = Some(Identifier {
                    token: self.cur_token.clone(),
                    value: self.cur_token.literal.clone(),
                })
            }
            _ => (),
        }

        if !self.expect_peek(TokenType::ASSIGN) {
            return None;
        }
        self.next_token();
        let expr = self.parse_expression(LOWEST);

        match stmt {
            Statement::LetStatement(ref mut x) => match expr {
                Some(e) => x.value = Some(Box::new(e)),
                _ => (),
            },
            _ => println!("Value not found for let statement"),
        }

        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        Some(stmt)
    }

    fn parse_return_statement(&mut self) -> Option<Statement> {
        let mut stmt: Statement = Statement::ReturnStatement(ReturnStatement {
            token: self.cur_token.clone(),
            value: None,
        });
        self.next_token();
        if self.cur_token_is(TokenType::SEMICOLON) {
            //break;
        } else {
            let right_expr = self.parse_expression(LOWEST);
            match right_expr {
                Some(r) => match stmt {
                    Statement::ReturnStatement(ref mut x) => x.value = Some(Box::new(r)),
                    _ => println!("Should have gotten a return statement"),
                },
                None => (),
            }
            if self.peek_token_is(TokenType::SEMICOLON) {
                self.next_token();
            }
        }
        Some(stmt)
    }

    fn parse_expression_statement(&mut self) -> Option<Statement> {
        let mut stmt: Statement = Statement::ExpressionStatement(ExpressionStatement {
            token: self.cur_token.clone(),
            expr: None,
        });
        let prec = self.precedence.get(&self.cur_token.type_).clone();
        let precedence: i32;
        match prec {
            Some(p) => {
                precedence = *p;
            }
            _ => precedence = LOWEST,
        }
        let st = self.parse_expression(precedence);
        match st {
            Some(s) => {
                match stmt {
                    Statement::ExpressionStatement(ref mut x) => x.expr = Some(Box::new(s)),
                    _ => (), //error handling here i guess
                }
            }
            None => (),
        }
        //stmt.expr = self.parse_expression( LOWEST);
        if self.peek_token_is(TokenType::SEMICOLON) {
            self.next_token();
        }
        return Some(stmt);
    }

    fn parse_expression(&mut self, prec: i32) -> Option<Expression> {
        let prefix = self.prefix_fns.get(&self.cur_token.type_);
        let mut left_expr: Option<Box<Expression>> = None;
        match prefix {
            Some(fun) => {
                let x = &*fun.clone();
                match x {
                    PARSE_IDENTIFIER => {
                        let s = self.parse_identifier();
                        match s {
                            Some(stmt) => {
                                match stmt {
                                    Expression::Identifier(_) => left_expr = Some(Box::new(stmt)),
                                    _ => left_expr = None, // Maybe add error handling here
                                }
                            }
                            _ => left_expr = None,
                        }
                    }
                    PARSE_INTEGER_LITERAL => {
                        let s = self.parse_integer_literal();
                        match s {
                            Some(stmt) => {
                                match stmt {
                                    Expression::IntegerLiteral(_) => {
                                        left_expr = Some(Box::new(stmt))
                                    }
                                    _ => left_expr = None, //NOTE: Error handling here
                                }
                            }
                            _ => left_expr = None,
                        }
                    }

                    PARSE_PREFIX_EXPR => {
                        let s = self.parse_prefix_expression();
                        match s {
                            Some(stmt) => left_expr = Some(Box::new(stmt)),
                            _ => left_expr = None,
                        }
                    }

                    PARSE_BOOLEAN => {
                        let s = self.parse_boolean();
                        match s {
                            Some(stmt) => left_expr = Some(Box::new(stmt)),
                            _ => left_expr = None,
                        }
                    }
                    PARSE_GROUP => {
                        let s = self.parse_group_expression();
                        match s {
                            Some(stmt) => left_expr = Some(Box::new(stmt)),
                            _ => left_expr = None,
                        }
                    }
                    PARSE_IF => {
                        let s = self.parse_if_expression();
                        match s {
                            Some(stmt) => left_expr = Some(Box::new(stmt)),
                            _ => left_expr = None,
                        }
                    }
                    PARSE_FUNCTION => {
                        let s = self.parse_function_expression();
                        match s {
                            Some(stmt) => left_expr = Some(Box::new(stmt)),
                            _ => left_expr = None,
                        }
                    }
                    PARSE_STRING => {
                        let s = self.parse_string();
                        match s {
                            Some(stmt) => left_expr = Some(Box::new(stmt)),
                            _ => left_expr = None,
                        }
                    }
                    _ => return None,
                }
            }
            None => return None,
        }

        while !self.peek_token_is(TokenType::SEMICOLON) && prec < self.peek_precedence() {
            let infix = self.infix_fns.get(&self.peek_token.type_);
            match infix {
                Some(inx) => {
                    let x = &*inx.clone();
                    match x {
                        PARSE_INFIX_EXPR => {
                            self.next_token();
                            let ex = self.parse_infix_expression(left_expr);
                            match ex {
                                Some(s) => left_expr = Some(Box::new(s)),
                                _ => return None,
                            }
                        }
                        PARSE_CALL => {
                            self.next_token();
                            let ex = self.parse_call_expression(left_expr);
                            match ex {
                                Some(s) => left_expr = Some(Box::new(s)),
                                _ => return None,
                            }
                        }
                        _ => return None,
                    }
                }
                None => (), //return left_expr,
            }
        }
        match left_expr {
            Some(e) => return Some(*e),
            _ => return None,
        }
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        Some(Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_integer_literal(&mut self) -> Option<Expression> {
        let mut lit: Expression = Expression::IntegerLiteral(IntegerLiteral {
            token: Token {
                type_: self.cur_token.type_,
                literal: self.cur_token.literal.clone(),
            },
            value: 0,
        });
        let val = self.cur_token.literal.parse::<i64>();
        match val {
            Ok(i) => {
                match lit {
                    Expression::IntegerLiteral(ref mut x) => x.value = i,
                    _ => println!("Wrong expression. Should of been integer literal"),
                }
                return Some(lit);
            }
            Err(err) => {
                let msg = format!(
                    "could not parse {} as integer. {}",
                    self.cur_token.literal, err
                );
                self.errors.push(msg);
                return None;
            }
        }
    }

    fn parse_prefix_expression(&mut self) -> Option<Expression> {
        let mut expr: Expression = Expression::PrefixExpression(PrefixExpression {
            token: self.cur_token.clone(),
            operator: self.cur_token.literal.clone(),
            right: None,
        });
        self.next_token();
        let s = self.parse_expression(PREFIX);
        match s {
            Some(st) => match expr {
                Expression::PrefixExpression(ref mut x) => x.right = Some(Box::new(st)),
                _ => println!("Should have been prefix expression"),
            },
            _ => (),
        }
        Some(expr)
    }

    fn parse_string(&mut self) -> Option<Expression> {
        Some(Expression::StringLiteral(StringLiteral {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        }))
    }

    fn parse_boolean(&mut self) -> Option<Expression> {
        Some(Expression::Boolean(Boolean {
            token: self.cur_token.clone(),
            value: self.cur_token_is(TokenType::TRUE),
        }))
    }

    fn parse_group_expression(&mut self) -> Option<Expression> {
        self.next_token();
        let exp = self.parse_expression(LOWEST);
        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }
        // self.next_token();
        return exp;
    }

    fn parse_if_expression(&mut self) -> Option<Expression> {
        let mut exp: Expression = Expression::IfExpression(IfExpression {
            token: Token {
                type_: TokenType::IF,
                literal: "if".to_string(),
            },
            cond: None,
            consequence: None,
            alternative: None,
        });
        if !self.expect_peek(TokenType::LPAREN) {
            return None;
        }
        self.next_token();
        let con = self.parse_expression(LOWEST);
        match con {
            Some(e) => match exp {
                Expression::IfExpression(ref mut x) => x.cond = Some(Box::new(e)),
                _ => println!("Should have been an if expression (err in cond section)"),
            },
            None => todo!(),
        }

        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }
        if !self.expect_peek(TokenType::LBRACE) {
            return None;
        }

        let conseq = self.parse_block_statement();
        match conseq {
            Some(c) => match exp {
                Expression::IfExpression(ref mut x) => x.consequence = Some(Box::new(c)),
                _ => println!("Should have been an if expression (err in consequence section)"),
            },
            None => (),
        }
        if self.peek_token_is(TokenType::ELSE) {
            self.next_token();
            if !self.expect_peek(TokenType::LBRACE) {
                return Some(exp);
            }
            let alt = self.parse_block_statement();
            match alt {
                Some(b) => match exp {
                    Expression::IfExpression(ref mut x) => x.alternative = Some(Box::new(b)),
                    _ => println!("Should have been an if expression (err in consequence section)"),
                },
                None => (),
            }
        }

        return Some(exp);
    }

    fn parse_block_statement(&mut self) -> Option<Statement> {
        let mut block = Statement::BlockStatement(BlockStatement {
            token: Token {
                type_: self.cur_token.type_,
                literal: self.cur_token.literal.clone(),
            },
            statements: Vec::<Statement>::new(),
        });
        self.next_token();
        while !self.cur_token_is(TokenType::RBRACE) && !self.cur_token_is(TokenType::EOF) {
            let stmt = self.parse_statement();
            match stmt {
                Some(s) => match block {
                    Statement::BlockStatement(ref mut x) => x.statements.push(s),
                    _ => (),
                },
                None => (),
            }
            self.next_token();
        }
        return Some(block);
    }

    fn parse_infix_expression(&mut self, left: Option<Box<Expression>>) -> Option<Expression> {
        let mut expr: Expression = Expression::InfixExpression(InfixExpression {
            token: self.cur_token.clone(),
            operator: self.cur_token.literal.clone(),
            left,
            right: None,
        });
        let mut prc = self.cur_precedence();
        match expr {
            Expression::InfixExpression(ref mut x) => {
                if x.operator == "+".to_string() {
                    prc = prc - 1;
                }
            }
            _ => (),
        }
        //println!("{}", expr.print());
        self.next_token();
        let stmt = self.parse_expression(prc);
        match stmt {
            Some(s) => {
                match expr {
                    Expression::InfixExpression(ref mut x) => {
                        x.right = Some(Box::new(s));
                        return Some(expr);
                    }
                    _ => return None, //println!("Problem parsing the infix expression")
                }
                //println!("{}", expr.print());
                // return Some(expr);
            }
            _ => return None,
        }
    }

    fn lower_precedence(&mut self, prc: i32) -> i32 {
        let p = prc - 1;
        if p < 0 {
            return 0;
        } else {
            return p;
        }
    }

    fn parse_function_expression(&mut self) -> Option<Expression> {
        let mut lit: Expression = Expression::FunctionLiteral(FunctionLiteral {
            token: self.cur_token.clone(),
            parameters: None,
            body: None,
        });
        if !self.expect_peek(TokenType::LPAREN) {
            return None;
        }
        let par = self.parse_function_parameters();
        match par {
            Some(v) => match lit {
                Expression::FunctionLiteral(ref mut x) => x.parameters = Some(v),
                _ => (),
            }, //lit.parameters = Some(v),
            None => (), //TODO: add error handling here when parsing func parameters goes wrong
                        //I dont think its necessary a function can have no parameters;
        }

        if !self.expect_peek(TokenType::LBRACE) {
            return None;
        }

        let bod = self.parse_block_statement();
        match bod {
            Some(b) => match lit {
                Expression::FunctionLiteral(ref mut x) => x.body = Some(Box::new(b)),
                _ => (),
            }, //lit.body = Some(b),
            None => return Some(lit), //TODO: add error handling here as well
        }

        return Some(lit);
    }

    fn parse_function_parameters(&mut self) -> Option<Vec<Expression>> {
        let mut ident = Vec::<Expression>::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Some(ident);
        }
        self.next_token();
        let identi = Expression::Identifier(Identifier {
            token: self.cur_token.clone(),
            value: self.cur_token.literal.clone(),
        });
        ident.push(identi);
        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            ident.push(Expression::Identifier(Identifier {
                token: self.cur_token.clone(),
                value: self.cur_token.literal.clone(),
            }));
        }
        if !self.expect_peek(TokenType::RPAREN) {
            return None;
        }
        return Some(ident);
    }

    fn parse_call_expression(&mut self, func: Option<Box<Expression>>) -> Option<Expression> {
        let mut expr: Expression = Expression::CallExpression(CallExpression {
            token: self.cur_token.clone(),
            func,
            args: None,
        });
        let arguments = self.parse_call_arguments();
        match arguments {
            Some(args) => {
                match expr {
                    Expression::CallExpression(ref mut x) => x.args = Some(args),
                    _ => (),
                }
                return Some(expr);
            }
            None => return Some(expr),
        }
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expression>> {
        let mut arg = Vec::<Expression>::new();
        if self.peek_token_is(TokenType::RPAREN) {
            self.next_token();
            return Some(arg);
        }

        self.next_token();
        let ag = self.parse_expression(LOWEST);
        match ag {
            Some(a) => {
                arg.push(a);
            }
            None => return None,
        }

        while self.peek_token_is(TokenType::COMMA) {
            self.next_token();
            self.next_token();
            let ag = self.parse_expression(LOWEST);
            match ag {
                Some(a) => {
                    arg.push(a);
                }
                None => return None,
            }
        }
        if !self.expect_peek(TokenType::RPAREN) {
            return None; //TODO: Error handling
        }

        return Some(arg);
    }

    fn no_prefix_parse_fn_error(&mut self, token: TokenType) {
        let msg = format!("no prefix parse function for {} found", token);
        self.errors.push(msg);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn let_test() {
        let input = "let ten = 10; let five = 5; let foobar = 833833;";
        let mut lex = Lexer::init_lexer(input);
        let mut par = Parser::new(&mut lex);
        let program: Program = par.parse_program();
        println!("{}", program.len());
        for i in 0..program.len() {
            println!("{}", program[i].print());
        }
        assert_eq!(program.len(), 3);
    }

    #[test]
    fn return_test() {
        let input: &'static str = "return 5;
        return 10;
        return 993322;";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }
        if prog.len() != 3 {
            assert!(false);
        }
        let pe = p.errors();
        if pe.len() > 0 {
            assert!(false);
        }
        assert_eq!(prog.len(), 3);
    }

    #[test]
    fn parse_prefix_ex() {
        let input: &'static str = "-5; !5;";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }

        println!("{:?}", p.errors);
    }

    #[test]
    fn parse_infix_ex() {
        let input: &'static str = "10 + 5; 5 > 6;";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }

        println!("{:?}", p.errors);
    }

    #[test]
    fn parse_boolean() {
        let input = "!true; false; let foobar = true; let barfoo = false;";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }
        println!("{:?}", p.errors);
    }

    #[test]
    fn parse_group() {
        let input = "1 + (2 + 3) + 4;";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }
        println!("{:?}", p.errors);
    }

    #[test]
    fn parse_if() {
        let input = "if(x < y) {x} else {y};";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }
        println!("{:?}", p.errors);
    }

    #[test]
    fn parse_fun() {
        let input = "fn(x) {let x = 3;}; fn(x, y) {let y = 1;};";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }
        println!("{:?}", p.errors);
    }

    #[test]
    fn parse_call() {
        let input = " a + add(b * c) + d;";
        let mut lex: Lexer = Lexer::init_lexer(input);
        let mut p: Parser = Parser::new(&mut lex);
        let prog = p.parse_program();
        for i in &prog {
            println!("{}", i.print());
        }
        println!("{:?}", p.errors);
    }
}
