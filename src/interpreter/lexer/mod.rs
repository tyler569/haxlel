pub mod token;

use std::string::String;
use token::TokenType;
use token::Token;

pub struct LexerState {
    source: &String,
    character: char,
    position: usize,
    location: (usize, usize)
}

fn advance(state: &mut LexerState) {
    state.position += 1;

    if state.position >= state.source.len() {
        state.character = 0 as char;
    } else {
        state.character = state.source.chars().nth(state.position);
    }
}

fn get_next_token(state: &mut LexerState) -> Result<Token, &'static str> {

}
