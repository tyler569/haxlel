mod lexer;
mod value;

use self::lexer::token::TokenType;
use self::lexer::token::TokenLocation;
use self::lexer::token::Token;

use self::lexer::LexerState;

pub fn interpret(source: &String) {
    let tokens = lexer::get_tokens(source);

    if tokens.is_ok() {
        for token in tokens.ok().unwrap() {
            println!("{} | {}", token.token_type.to_string(), token.value.unwrap_or(String::new()));
        }
    } else {
        println!("Error: {}", tokens.err().unwrap());
    }
}
