use std::vec;

use crate::{
    ast::{
        BlockStatement, Expression, ExpressionStatement, Identifier, IfExpression, Program,
        ReturnStatement, Statement,
    },
    object::{Environment, Function, Object},
};

pub fn eval_prog(prog: Program, env: &mut Environment) -> Object {
    let result: Object = Object::Null;
    for statement in prog {
        let result = eval(statement, env);

        match result {
            Some(Object::Return(x)) => return *x,
            Some(Object::Error(x)) => return Object::Error(x),
            _ => (),
        }
    }
    return result;
}

fn eval(state: Statement, env: &mut Environment) -> Option<Object> {
    match state {
        Statement::LetStatement(x) => {
            let mut res = Object::Null;
            match x.value {
                Some(x) => res = eval_expr(*x, env),
                None => {
                    res = Object::Error("Could not evaluate let statement expression".to_string())
                }
            }
            if is_error(res.clone()) {
                return Some(res);
            }
            match x.name {
                Some(x) => {
                    env.set(x.value, res.clone());
                }
                None => res = Object::Error("Could not add identifier to environment".to_string()),
            }
            if is_error(res.clone()) {
                return Some(res);
            }
            Some(res)
        }
        Statement::ReturnStatement(x) => {
            let mut res = Object::Null;

            res = eval_ret(x, env);
            if is_error(res.clone()) {
                return Some(res);
            }
            Some(res)
        }
        Statement::ExpressionStatement(x) => {
            let mut res = Object::Null;

            res = eval_expression_stmt(x, env);
            Some(res)
        }
        Statement::BlockStatement(x) => {
            let mut res = Object::Null;

            res = eval_block(x, env);
            Some(res)
        }
    }
}

fn eval_ret(ret: ReturnStatement, env: &mut Environment) -> Object {
    let mut val = Object::Null;
    match ret.value {
        Some(x) => val = eval_expr(*x, env),
        None => return val,
    }
    Object::Return(Box::new(val))
}

fn eval_block(block: BlockStatement, env: &mut Environment) -> Object {
    let val = Object::Null;
    for statement in block.statements {
        let val = eval(statement, env);
        let v: Object;
        match val {
            Some(s) => v = s,
            None => v = Object::Null,
        };
        if v != Object::Null
            && (v.type_() == "RETURN_VALUE_OBJ".to_string() || v.type_() == "ERROR".to_string())
        {
            return v;
        }
    }
    return val;
}

fn eval_expression_stmt(expr: ExpressionStatement, env: &mut Environment) -> Object {
    let val = Object::Null;
    match expr.expr {
        Some(x) => return eval_expr(*x, env),
        None => return val,
    }
}
fn eval_expr(expr: Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::Identifier(x) => return eval_identifier(x, env),
        Expression::IntegerLiteral(x) => return Object::Integer(x.value),
        Expression::PrefixExpression(x) => {
            let mut right = Object::Null;
            match x.right {
                Some(x) => right = eval_expr(*x, env),
                None => (),
            }
            if is_error(right.clone()) {
                return right;
            }
            return eval_prefix(x.operator, right);
        }
        Expression::InfixExpression(x) => {
            let mut left = Object::Null;
            match x.left {
                Some(l) => left = eval_expr(*l, env),
                None => (),
            }
            if is_error(left.clone()) {
                return left;
            }
            let mut right = Object::Null;
            match x.right {
                Some(r) => right = eval_expr(*r, env),
                None => (),
            }
            if is_error(right.clone()) {
                return right;
            }
            return eval_infix_expr(x.operator, left, right);
        }
        Expression::Boolean(x) => return Object::Boolean(x.value),
        Expression::IfExpression(x) => return eval_if_expr(x, env),
        Expression::FunctionLiteral(x) => {
            let param: Vec<Expression>;
            match x.parameters {
                Some(x) => param = x,
                None => param = vec![],
            };
            let bod: BlockStatement;
            match x.body {
                Some(b) => match *b {
                    Statement::BlockStatement(x) => bod = x,
                    _ => return Object::Error("not a block statement".to_string()),
                },
                None => {
                    return Object::Error("expected statement but got something else".to_string())
                }
            }
            return Object::Function(Function {
                parameters: param,
                body: bod,
                env: env.clone(),
            });
        }
        Expression::CallExpression(x) => {
            let function: Object;
            match x.func {
                Some(x) => function = eval_expr(*x, env),
                None => {
                    function = Object::Error("expected function got something else".to_string())
                }
            }
            if is_error(function.clone()) {
                return function;
            }
            let mut arg: Vec<Expression> = vec![];
            match x.args {
                Some(x) => arg = x,
                None => (),
            }
            let args = eval_exprs(arg, env.clone());
            if args.len() == 1 && is_error(args[0].clone()) {
                return args[0].clone();
            }
            return apply_function(function, args, env);
        }
        Expression::StringLiteral(x) => Object::String(x.value),
    }
}

fn eval_exprs(exps: Vec<Expression>, env: Environment) -> Vec<Object> {
    let mut result = Vec::<Object>::new();
    for exp in exps {
        let evaluated = eval_expr(exp, &mut env.clone());
        if is_error(evaluated.clone()) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }
    return result;
}

fn eval_identifier(ident: Identifier, env: &mut Environment) -> Object {
    let check = env.get(ident.value.clone());
    match check {
        Some(x) => return x.clone(),
        None => return Object::Error(format!("identifier not found: {}", ident.value.clone())),
    }
}

fn eval_prefix(oper: String, right: Object) -> Object {
    match oper.as_str() {
        "!" => return eval_bang_oper(right),
        "-" => return eval_minus_oper(right),
        _ => Object::Error(format!("unknown operator: {} {}", oper, right.type_())),
    }
}

fn eval_bang_oper(obj: Object) -> Object {
    match obj {
        Object::Boolean(true) => return Object::Boolean(false),
        Object::Boolean(false) => return Object::Boolean(true),
        Object::Null => return Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_oper(obj: Object) -> Object {
    match obj {
        Object::Integer(x) => return Object::Integer(-x),
        _ => Object::Error(format!("Don't know yet")),
    }
}

fn eval_infix_expr(oper: String, left: Object, right: Object) -> Object {
    if left.type_().as_str() == "INTEGER_OBJ" && right.type_().as_str() == "INTEGER_OBJ" {
        return eval_integer_infix(oper, left, right);
    } else if left.type_().as_str() == "STRING" && right.type_().as_str() == "STRING" {
        return eval_string_infix(oper, left, right);
    }
    Object::Error(format!("type mismatch: {} {}", left.type_(), right.type_()))
}

fn eval_integer_infix(oper: String, left: Object, right: Object) -> Object {
    match oper.as_str() {
        "+" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Integer(leftval + rightval);
        }
        "-" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Integer(leftval - rightval);
        }
        "*" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Integer(leftval * rightval);
        }
        "/" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Integer(leftval / rightval);
        }
        "<" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Boolean(leftval < rightval);
        }
        ">" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Boolean(leftval > rightval);
        }
        "==" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Boolean(leftval == rightval);
        }
        "!=" => {
            let mut leftval: i64 = 0;
            let mut rightval: i64 = 0;
            match left {
                Object::Integer(x) => leftval = x,
                _ => (),
            }
            match right {
                Object::Integer(x) => rightval = x,
                _ => (),
            }
            return Object::Boolean(leftval != rightval);
        }
        _ => Object::Error(format!(
            "unknown operator: {} {} {}",
            left.type_(),
            oper,
            right.type_()
        )),
    }
}

fn eval_string_infix(oper: String, left: Object, right: Object) -> Object {
    if oper != "+".to_string() {
        return new_error(format!(
            "unknown operator: {} {} {}",
            left.type_(),
            oper,
            right.type_()
        ));
    }
    let mut left_val: String;
    match left {
        Object::String(x) => left_val = x,
        _ => todo!(),
    }
    let right_val: String;
    match right {
        Object::String(x) => right_val = x,
        _ => todo!(),
    }
    left_val.push_str(right_val.as_str());
    return Object::String(left_val);
}

fn eval_if_expr(expr: IfExpression, env: &mut Environment) -> Object {
    let mut cond = Object::Null;
    match expr.cond {
        Some(c) => cond = eval_expr(*c, env),
        None => (),
    }
    if is_error(cond.clone()) {
        return cond;
    }
    if is_truthy(cond) {
        let mut cons = Object::Null;
        match expr.consequence {
            Some(c) => {
                let con = eval(*c, env);
                match con {
                    Some(c) => cons = c,
                    None => {
                        cons = Object::Error(
                            "Could not evaluate consequence part of if expression".to_string(),
                        )
                    }
                }
            }
            None => (),
        }
        return cons;
    } else {
        match expr.alternative {
            Some(a) => {
                let e = eval(*a, env);
                match e {
                    Some(e) => return e,
                    None => {
                        return Object::Error(
                            "could not evaluated alternative part of the if expression".to_string(),
                        )
                    }
                }
            }
            None => return Object::Null,
        }
    }
}

#[inline(always)]
fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Boolean(true) => true,
        Object::Boolean(false) => false,
        _ => false,
    }
}

#[inline(always)]
fn new_error(format: String) -> Object {
    Object::Error(format)
}

#[inline(always)]
fn is_error(obj: Object) -> bool {
    obj.type_() == "ERROR".to_string()
}

fn apply_function(fun: Object, args: Vec<Object>, env: &mut Environment) -> Object {
    match fun.clone() {
        Object::Function(f) => {
            let mut extended_env = extended_func_env(fun, args, env);
            let evaluated = eval_block(f.body, &mut extended_env);
            return unwrap_return_value(evaluated);
        }
        _ => Object::Error(format!("not a function: {}", fun.type_())),
    }
}

fn extended_func_env(fun: Object, args: Vec<Object>, env: &mut Environment) -> Environment {
    match fun {
        Object::Function(x) => {
            let func = x;
            let mut envex = env.new_enclosed(func.env);
            for (index, param) in func.parameters.iter().enumerate() {
                match param.clone() {
                    Expression::Identifier(x) => {
                        envex.set(x.value, args[index].clone());
                    }
                    Expression::IntegerLiteral(_) => todo!(),
                    Expression::PrefixExpression(_) => todo!(),
                    Expression::InfixExpression(_) => todo!(),
                    Expression::Boolean(_) => todo!(),
                    Expression::IfExpression(_) => todo!(),
                    Expression::FunctionLiteral(_) => todo!(),
                    Expression::CallExpression(_) => todo!(),
                    Expression::StringLiteral(_) => todo!(),
                }
            }
            return envex;
        }
        _ => (),
    }
    Environment::new()
}

fn unwrap_return_value(obj: Object) -> Object {
    match obj {
        Object::Integer(_) => obj,
        Object::Boolean(_) => obj,
        Object::Null => obj,
        Object::Return(x) => *x,
        Object::Error(_) => obj,
        Object::Function(_) => obj,
        Object::String(_) => obj,
    }
}

#[cfg(test)]
mod tests {
    use crate::eval::*;
    use crate::lexer::Lexer;
    use crate::object::Environment;
    use crate::parser::Parser;

    #[test]
    fn string_test() {
        let mut env = Environment::new();
        //FIXME: functions don't seem to work when applied in this fashion below
        let input = "let makeGreeter = fn(x) {fn(y) {return x + y; };};
         let hello = makeGreeter(\"Hello\");
         let x = hello( \"William\");
         return x;";
        let mut lex = Lexer::init_lexer(input);
        let mut par = Parser::new(&mut lex);
        let program: Program = par.parse_program();
        let evaluated = eval_prog(program, &mut env);
        println!("{}", evaluated.inspect());
    }

    #[test]
    fn let_test() {
        let mut env = Environment::new();
        let input = "let f = fn(x, y) {
            let result = x + y + 10;
            return result;
         };
         let y = f(11, 12);
         return y;";
        let mut lex = Lexer::init_lexer(input);
        let mut par = Parser::new(&mut lex);
        let program: Program = par.parse_program();
        let evaluated = eval_prog(program, &mut env);
        println!("{}", evaluated.inspect());
    }

    #[test]
    fn err_test() {
        let mut env = Environment::new();
        let input = "5 + true;";
        let mut lex = Lexer::init_lexer(input);
        let mut par = Parser::new(&mut lex);
        let program: Program = par.parse_program();
        let evaluated = eval_prog(program, &mut env);
        println!("{}", evaluated.inspect());
    }
}
