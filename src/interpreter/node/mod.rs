pub mod lexer;
use self::lexer::error::token::TokenLocation;
use self::lexer::error::Error;

use std::hash::Hash;
use std::hash::Hasher;
use std::collections::hash_map::HashMap;

pub struct Array<'a> {
    pub elements: HashMap<Value<'a>, Value<'a>>,
    pub references: usize
}

pub struct Function<'a> {
    pub parameters: Vec<String>,
    pub node: Node<'a>
}

pub struct Node<'a> {
    pub value: Value<'a>,
    pub location: TokenLocation,
    pub children: Option<Vec<Node<'a>>>,
    pub resolve: fn(&Node<'a>) -> Result<Value<'a>, Error>
}

pub struct Variables<'a> {
    pub variables: HashMap<String, Value<'a>>,
    pub outer: &'a HashMap<String, Value<'a>>
}

pub fn default_resolve<'a>(node: &Node<'a>) -> Result<Value<'a>, Error> {
    Err(Error::DefaultNode(node.location.clone()))
}

pub enum Value<'a> {
    Function(&'a Node<'a>),
    Array(&'a mut Array<'a>),
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Node
}

impl<'a> Hash for Value<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match *self {
            Value::String(ref a) => a.hash(state),
            Value::Number(a) => (a as i64).hash(state),
            Value::Null => (0).hash(state),
            _ => panic!("Invalid state")
        }
    }
}
