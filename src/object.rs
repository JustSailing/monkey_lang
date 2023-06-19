use std::collections::HashMap;

use crate::ast::{BlockStatement, Expression};

#[derive(PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    Null,
    Return(Box<Object>),
    Error(String),
    Function(Function),
    String(String),
}

impl Object {
    pub fn inspect(&self) -> String {
        match self {
            Object::Integer(x) => format!("{}", x),
            Object::Boolean(x) => format!("{}", x),
            Object::Null => format!("null"),
            Object::Return(x) => format!("{}", x.inspect()),
            Object::Error(x) => format!("Error: {}", x),
            Object::Function(x) => {
                let mut params = Vec::<String>::new();
                for par in &x.parameters {
                    params.push(par.print());
                }
                format!("fn( {:#?} ) {{\n {} \n}}", params, x.body.print())
            }
            Object::String(x) => format!("{}", x),
        }
    }

    pub fn type_(&self) -> String {
        match self {
            Object::Integer(_) => "INTEGER_OBJ".to_string(),
            Object::Boolean(_) => "BOOLEAN_OBJ".to_string(),
            Object::Null => "NULL".to_string(),
            Object::Return(_) => "RETURN_VALUE_OBJ".to_string(),
            Object::Error(_) => "ERROR".to_string(),
            Object::Function(_) => "FUNCTION".to_string(),
            Object::String(_) => "STRING".to_string(),
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Function {
    pub parameters: Vec<Expression>,
    pub body: BlockStatement,
    pub env: Environment,
}

#[derive(PartialEq, Clone)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::<String, Object>::new(),
            outer: None,
        }
    }

    pub fn new_enclosed(&mut self, envir: Environment) -> Environment {
        let mut env = Environment::new();
        env.outer = Some(Box::new(envir));
        env
    }

    pub fn get(&self, name: String) -> Option<Object> {
        let p = self.store.get(&name);
        match p {
            Some(c) => return Some(c.clone()),
            None => {
                if self.outer != None {
                    let p = self.outer.as_ref()?;

                    let pp = p.get(name.clone());
                    match pp {
                        Some(c) => return Some(c),
                        None => return None,
                    }
                } else {
                    None
                }
            }
        }
    }

    pub fn set(&mut self, name: String, obj: Object) -> Object {
        self.store.insert(name, obj.clone());
        return obj;
    }
}
