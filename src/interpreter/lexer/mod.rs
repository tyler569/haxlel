pub mod token;
use self::token::TokenType;
use self::token::TokenLocation;
use self::token::Token;

use std::string::String;

pub struct LexerState<'a> {
    source: &'a String,
    character: char,
    position: usize,
    location: TokenLocation
}

fn advance(state: &mut LexerState) {
    state.position += 1;

    if state.position >= state.source.len() {
        state.character = 0 as char;
    } else {
        state.character = state.source.chars().nth(state.position).unwrap();
    }
    
    if state.character == '\n' {
        state.location.character = 0;
        state.location.line += 1;
    } else {
        state.location.character += 1;
    }
}

fn get_identifier(lowercase: bool, state: &mut LexerState) -> Result<Token, &'static str> {
    let mut result = String::new();

    let location = state.location.clone();
    
    while {
        result.push(state.character);

        advance(state);

        if lowercase {
            state.character.is_lowercase()
        } else {
            state.character.is_uppercase()
        }
    } {}
    
    Ok(match result.as_ref() {
        "true" | "TRUE" => Token {
            token_type: TokenType::BooleanTrue,
            value: None,
            location: location
        },
        "false" | "FALSE" => Token {
            token_type: TokenType::BooleanFalse,
            value: None,
            location: location
        },
        _ => Token {
            token_type: TokenType::Identifier,
            value: Some(result),
            location: location
        }
    })
}

fn get_number(state: &mut LexerState) -> Result<Token, &'static str> {
    let mut result = String::new();
    
    let location = state.location.clone();

    while {
        result.push(state.character);

        advance(state);

        state.character.is_digit(10)
    } {}
    
    if state.character == '.' {
        advance(state);
        
        if state.character.is_digit(10) {
            while state.character.is_digit(10) {
                result.push(state.character);

                advance(state);
            }

            Ok(Token {
                token_type: TokenType::Number,
                value: Some(result),
                location: location
            })
        } else {
            Err("Character after a decimal point isn't a decimal")
        }
    } else {
        Ok(Token {
            token_type: TokenType::Number,
            value: Some(result),
            location: location
        })
    }
}

fn get_string(state: &mut LexerState) -> Result<Token, &'static str> {
    let mut result = String::new();
    
    let location = state.location.clone();

    advance(state);

    while (state.character != '!' && state.character != '?' && state.character != 0 as char) || state.character == '\\' {
        if state.character == '\\' {
            advance(state);
        }
        
        result.push(state.character);

        advance(state);
    }

    if state.character == 0 as char {
        Err("String isn't terminated: reached end of file")
    } else {
        advance(state);

        Ok(Token {
            token_type: TokenType::String,
            value: Some(result),
            location: location
        })
    }
}

fn get_next_token(state: &mut LexerState) -> Result<Token, &'static str> {
    let location = state.location.clone();
    
    match state.character {
        ch if ch.is_alphabetic() => get_identifier(ch.is_lowercase(), state),
        ch if ch.is_digit(10) => get_number(state),
        '"' => get_string(state),
        '.' => {
            advance(state);

            Ok(Token {
                token_type: TokenType::Dot,
                value: None,
                location: location
            })
        }
        _ => Err("Invalid character")
    }
}
