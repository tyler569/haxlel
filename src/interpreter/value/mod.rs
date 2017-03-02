use std::string::String;
use std::vec::Vec;

pub enum Value {
    Function,
    Array(Vec<Value>),
    String(String),
    Number(f64),
    None
}
