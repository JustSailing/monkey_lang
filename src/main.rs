use std::io::*;
mod ast;
mod eval;
mod lexer;
mod object;
mod parser;
mod token;
use crate::eval::*;
use crate::lexer::*;
use crate::object::*;
use crate::parser::*;
fn main() {
    let mut env = Environment::new();
    loop {
        let mut input = String::new();
        let var1 = std::io::stdout().write(">> ".as_bytes()).unwrap();
        std::io::stdout().flush();
        let x = stdin().read_line(&mut input).unwrap();
        match x {
            _ => {
                let mut lexer = Lexer::init_lexer(&input);
                let mut parser = Parser::new(&mut lexer);
                let mut program = parser.parse_program();
                if parser.errors().len() != 0 {
                    println!("error");
                    continue;
                }
                let mut evaluated = eval_prog(program, &mut env);
                if evaluated != Object::Null {
                    std::io::stdout().write(format!("{}", evaluated.inspect()).as_bytes());
                    std::io::stdout().flush();
                }
            }
            0 => println!("Problem"),
        };
    }
}
