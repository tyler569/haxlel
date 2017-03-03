mod node;

use self::node::lexer::error::token::TokenType;
use self::node::lexer::error::token::TokenLocation;
use self::node::lexer::error::token::Token;
use self::node::lexer::error::Error;

use self::node::lexer::LexerState;

use self::node::Value;
use self::node::Array;
use self::node::Node;

use self::node::lexer;

pub struct InterpreterState<'a> {
    lexer_state: LexerState<'a>,
    token: Token,
    functions: Vec<Node<'a>>,
    arrays: Vec<Value<'a>>
}

fn advance(state: &mut InterpreterState) -> Option<Error> {
    let next_token = lexer::get_next_token(&mut state.lexer_state);
    
    if next_token.is_ok() {
        state.token = next_token.ok().unwrap();

        None
    } else {
        Some(next_token.err().unwrap())
    }
}

fn expect(token_type: TokenType, state: &mut InterpreterState) -> bool {
    token_type == state.token.token_type
}

fn interpret_factor<'a>(state: &mut InterpreterState) -> Result<Node<'a>, Error> {
    let location = state.token.location.clone();
    
    match state.token.token_type {
        TokenType::BooleanTrue => {
            advance(state);

            Ok(Node {
                value: Value::Boolean(true),
                location: location,
                children: None,
                resolve: node::default_resolve
            })
        },
        TokenType::BooleanFalse => {
            advance(state);

            Ok(Node {
                value: Value::Boolean(true),
                location: location,
                children: None,
                resolve: node::default_resolve
            })
        },
        TokenType::Number => {
            let number = state.token.value.clone().unwrap().parse::<f64>().unwrap();

            advance(state);

            Ok(Node {
                value: Value::Number(number),
                location: location,
                children: None,
                resolve: node::default_resolve
            })
        },
        TokenType::String => {
            let string = state.token.value.clone().unwrap();

            advance(state);

            Ok(Node {
                value: Value::String(string),
                location: location,
                children: None,
                resolve: node::default_resolve
            })
        },
        TokenType::Plus => {
            advance(state);
            
            let factor = interpret_factor(state);

            if factor.is_err() {
                return Err(factor.err().unwrap());
            }

            Ok(Node {
                value: Value::Node,
                location: location,
                children: Some(vec!(factor.ok().unwrap())),
                resolve: node::default_resolve
            })
        },
        TokenType::Minus => {
            advance(state);

            let factor = interpret_factor(state);

            if factor.is_err() {
                return Err(factor.err().unwrap());
            }

            Ok(Node {
                value: Value::Node,
                location: location,
                children: Some(vec!(factor.ok().unwrap())),
                resolve: node::default_resolve
            })
        },
        TokenType::Approx => {
            advance(state);

            let factor = interpret_factor(state);

            if factor.is_err() {
                return Err(factor.err().unwrap());
            }

            Ok(Node {
                value: Value::Node,
                location: location,
                children: Some(vec!(factor.ok().unwrap())),
                resolve: node::default_resolve
            })
        },
        _ => Err(Error::Undefined)
    }
}

pub fn interpret(source: &String) {
    let tokens = lexer::get_tokens(source);

    if tokens.is_ok() {
        for token in tokens.ok().unwrap() {
            println!("{} | {}", token.token_type.to_string(), token.value.unwrap_or(String::new()));
        }
    } else {
        println!("Error: {}", tokens.err().unwrap().to_string());
    }
}
