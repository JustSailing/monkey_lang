use crate::token::Token;

pub type Program = Vec<Statement>;

#[derive(Clone, PartialEq)]
pub enum Statement {
    LetStatement(LetStatement),
    ReturnStatement(ReturnStatement),
    ExpressionStatement(ExpressionStatement),
    BlockStatement(BlockStatement),
}

impl Statement {
    pub fn print(&self) -> String {
        match self {
            Statement::LetStatement(x) => return x.print(),
            Statement::ReturnStatement(x) => return x.print(),
            Statement::ExpressionStatement(x) => return x.print(),
            Statement::BlockStatement(x) => return x.print(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct LetStatement {
    pub token: Token,
    pub name: Option<Identifier>,
    pub value: Option<Box<Expression>>,
}

impl LetStatement {
    fn print(&self) -> String {
        let val: String;
        let nm: String;
        match &self.name {
            Some(n) => nm = n.print(),
            None => nm = "None".to_string(),
        }
        match &self.value {
            Some(v) => val = v.print(),
            None => val = "None".to_string(),
        }
        format!("LetStmt: {:?} name: {} value {}", self.token, nm, val)
    }
}

#[derive(Clone, PartialEq)]
pub struct ReturnStatement {
    pub token: Token,
    pub value: Option<Box<Expression>>,
}

impl ReturnStatement {
    fn print(&self) -> String {
        let val: String;
        match &self.value {
            Some(v) => val = v.print(),
            None => val = "None".to_string(),
        }
        format!("ReturnStmt: {:?} value {}", self.token, val)
    }
}

#[derive(Clone, PartialEq)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expr: Option<Box<Expression>>,
}

impl ExpressionStatement {
    fn print(&self) -> String {
        let val: String;
        match &self.expr {
            Some(v) => val = v.print(),
            None => val = "None".to_string(),
        }
        format!("ExprStmt: {:?} expr: {}", self.token, val)
    }
}

#[derive(Clone, PartialEq)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn print(&self) -> String {
        let mut val: String = String::from("");
        for i in &self.statements {
            val.push_str(i.print().as_str());
        }

        format!("BlockStmt: {:?} statements: {}", self.token, val)
    }
}

#[derive(Clone, PartialEq)]
pub enum Expression {
    Identifier(Identifier),
    IntegerLiteral(IntegerLiteral),
    PrefixExpression(PrefixExpression),
    InfixExpression(InfixExpression),
    Boolean(Boolean),
    IfExpression(IfExpression),
    FunctionLiteral(FunctionLiteral),
    CallExpression(CallExpression),
    StringLiteral(StringLiteral),
}

impl Expression {
    pub fn print(&self) -> String {
        match self {
            Expression::Identifier(x) => return x.print(),
            Expression::IntegerLiteral(x) => return x.print(),
            Expression::PrefixExpression(x) => return x.print(),
            Expression::InfixExpression(x) => return x.print(),
            Expression::Boolean(x) => return x.print(),
            Expression::IfExpression(x) => return x.print(),
            Expression::FunctionLiteral(x) => return x.print(),
            Expression::CallExpression(x) => return x.print(),
            Expression::StringLiteral(x) => return x.print(),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Identifier {
    pub fn print(&self) -> String {
        let s = format!("(Identifier: {:?} value: {})", self.token, self.value);
        //println!("{}",s);
        s
    }
}

#[derive(Clone, PartialEq)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl IntegerLiteral {
    fn print(&self) -> String {
        let s = format!("(IntegerLiteral: {:?} value: {})", self.token, self.value);
        //println!("{}", s);
        s
    }
}

#[derive(Clone, PartialEq)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl PrefixExpression {
    fn print(&self) -> String {
        let s: String;
        match &self.right {
            Some(r) => s = r.print(),
            None => s = "None".to_string(),
        }
        let s2 = format!(
            "(PrefixExpr: {:?} operator: {} right: {} )",
            self.token, self.operator, s
        );
        //println!("{}", s2);
        s2
    }
}

#[derive(Clone, PartialEq)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Option<Box<Expression>>,
    pub operator: String,
    pub right: Option<Box<Expression>>,
}

impl InfixExpression {
    fn print(&self) -> String {
        let sl: String;
        let sr: String;
        match &self.left {
            Some(l) => sl = l.print(),
            None => sl = "None".to_string(),
        }

        match &self.right {
            Some(r) => sr = r.print(),
            None => sr = "None".to_string(),
        }
        let s = format!(
            "InfixExpr: {:?} left: {} operator: {} right: {}",
            self.token, sl, self.operator, sr
        );
        //println!("{}", s);
        s
    }
}

#[derive(Clone, PartialEq)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Boolean {
    fn print(&self) -> String {
        let s = format!("{:?} value: {} ", self.token, self.value);
        //println!("{}", s);
        s
    }
}

#[derive(Clone, PartialEq)]
pub struct IfExpression {
    pub token: Token,
    pub cond: Option<Box<Expression>>,
    pub consequence: Option<Box<Statement>>,
    pub alternative: Option<Box<Statement>>,
}

impl IfExpression {
    fn print(&self) -> String {
        let con: String;
        let cons: String;
        let alt: String;
        match &self.cond {
            Some(c) => con = c.print(),
            None => con = "None".to_string(),
        }
        match &self.consequence {
            Some(c) => cons = c.print(),
            None => cons = "None".to_string(),
        }
        match &self.alternative {
            Some(c) => alt = c.print(),
            None => alt = "None".to_string(),
        }

        format!(
            "IfExpr: {:?} cond: {} cons: {} alt: {}",
            self.token, con, cons, alt
        )
    }
}

#[derive(Clone, PartialEq)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Option<Vec<Expression>>,
    pub body: Option<Box<Statement>>,
}

impl FunctionLiteral {
    fn print(&self) -> String {
        let mut par: String = String::from("");
        let bod: String;
        let s: String;
        match &self.parameters {
            Some(p) => {
                // let mut s: String = String::from("");
                for i in p {
                    par.push_str(i.print().as_str());
                }
            }
            None => par = "None".to_string(),
        }
        match &self.body {
            Some(b) => bod = b.print(),
            None => bod = "None".to_string(),
        }

        s = format!("{:?}: params: {} body: {}", self.token, par, bod);
        //println!("{}", s);
        s
    }
}

#[derive(Clone, PartialEq)]
pub struct CallExpression {
    pub token: Token,
    pub func: Option<Box<Expression>>,
    pub args: Option<Vec<Expression>>,
}

impl CallExpression {
    fn print(&self) -> String {
        let f: String;
        let mut arg: String = String::from("");
        let s: String;
        match &self.func {
            Some(fun) => f = fun.print(),
            None => f = "None".to_string(),
        }

        match &self.args {
            Some(a) => {
                for i in a {
                    arg.push_str(i.print().as_str());
                }
            }
            None => arg = "None".to_string(),
        }
        s = format!("CallExpr: {:?} func: {} args: {}", self.token, f, arg);
        //println!("{}", s);
        s
    }
}

#[derive(Clone, PartialEq)]
pub struct StringLiteral {
    pub token: Token,
    pub value: String,
}

impl StringLiteral {
    fn print(&self) -> String {
        format!("String: {}", self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ast::{Expression, Identifier, IntegerLiteral},
        token::{Token, TokenType},
    };

    #[test]
    fn ident_print() {
        let i = Expression::Identifier(Identifier {
            token: Token {
                type_: TokenType::IDENT,
                literal: "x".to_string(),
            },
            value: "5".to_string(),
        });
        let s = i.print();
        println!("{}", s);
        //Should print (Identifier: Token { type_: Ident, literal: "x" } value: 5)
    }

    #[test]
    fn integer_print() {
        let i = Expression::IntegerLiteral(IntegerLiteral {
            token: Token {
                type_: TokenType::INT,
                literal: "100".to_string(),
            },
            value: 100,
        });
        let s = i.print();
        println!("{}", s);
    }
}
