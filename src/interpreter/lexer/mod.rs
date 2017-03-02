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
    
    result = result.to_lowercase();

    Ok(match result.as_ref() {
        "true" => Token {
            token_type: TokenType::BooleanTrue,
            value: None,
            location: location
        },
        "false" => Token {
            token_type: TokenType::BooleanFalse,
            value: None,
            location: location
        },
        "if" => Token {
            token_type: TokenType::If,
            value: None,
            location: location
        },
        "elif" => Token {
            token_type: TokenType::ElseIf,
            value: None,
            location: location
        },
        "else" => Token {
            token_type: TokenType::Else,
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
            result.push('.');

            while state.character.is_digit(10) {
                result.push(state.character);

                advance(state);
            }
            
            if state.character == '.' {
                Err("Number has 2 decimal points")
            } else {
                Ok(Token {
                    token_type: TokenType::Number,
                    value: Some(result),
                    location: location
                })
            }
        } else {
            Err("Character after a decimal point isn't a character representing a decimal digit")
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
    while state.character.is_whitespace() {
        advance(state);
    }
    
    let location = state.location.clone();
    
    if state.position >= state.source.len() {
        Ok(Token {
            token_type: TokenType::Eof,
            value: None,
            location: location
        })
    } else {match state.character {
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
            },
            ':' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Colon,
                    value: None,
                    location: location
                })
            },
            '+' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Plus,
                    value: None,
                    location: location
                })
            },
            '-' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Minus,
                    value: None,
                    location: location
                })
            },
            '*' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Asterisk,
                    value: None,
                    location: location
                })
            },
            '/' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Slash,
                    value: None,
                    location: location
                })
            },
            '%' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Percentage,
                    value: None,
                    location: location
                })
            },
            '~' => {
                advance(state);

                Ok(if state.character == '=' {
                    advance(state);
                    
                    Token {
                        token_type: TokenType::ReferenceEquals,
                        value: None,
                        location: location
                    }
                } else {
                    Token {
                        token_type: TokenType::Approx,
                        value: None,
                        location: location
                    }
                })
            },
            '=' => {
                advance(state);

                Ok(if state.character == '=' {
                    advance(state);

                    Token {
                        token_type: TokenType::Equals,
                        value: None,
                        location: location
                    }
                } else {
                    Token {
                        token_type: TokenType::Assign,
                        value: None,
                        location: location
                    }
                })
            },
            '!' => {
                advance(state);

                Ok(if state.character == '=' {
                    advance(state);

                    Token {
                        token_type: TokenType::NotEquals,
                        value: None,
                        location: location
                    }
                } else {
                    Token {
                        token_type: TokenType::LocalTerminator,
                        value: None,
                        location: location
                    }
                })
            },
            '?' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::GlobalTerminator,
                    value: None,
                    location: location
                })
            },
            '<' => {
                advance(state);

                Ok(if state.character == '=' {
                    advance(state);

                    Token {
                        token_type: TokenType::LessEquals,
                        value: None,
                        location: location
                    }
                } else {
                    Token {
                        token_type: TokenType::Less,
                        value: None,
                        location: location
                    }
                })
            },
            '>' => {
                advance(state);

                Ok(if state.character == '=' {
                    advance(state);

                    Token {
                        token_type: TokenType::GreaterEquals,
                        value: None,
                        location: location
                    }
                } else {
                    Token {
                        token_type: TokenType::Greater,
                        value: None,
                        location: location
                    }
                })
            },
            '&' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::And,
                    value: None,
                    location: location
                })
            },
            '|' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Or,
                    value: None,
                    location: location
                })
            },
            '(' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Bracket,
                    value: None,
                    location: location
                })
            },
            '{' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::CurlyBracket,
                    value: None,
                    location: location
                })
            },
            '@' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Loop,
                    value: None,
                    location: location
                })
            },
            '#' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::ConditionalLoop,
                    value: None,
                    location: location
                })
            },
            '^' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Break,
                    value: None,
                    location: location
                })
            },
            '$' => {
                advance(state);

                Ok(Token {
                    token_type: TokenType::Function,
                    value: None,
                    location: location
                })
            },
            _ => Err("Invalid character")
        }
    }
}

fn init(source: &String) -> LexerState {
    let character = source.chars().nth(0);

    LexerState {
        source: source,
        character: if character.is_some() {
            character.unwrap()
        } else {
            0 as char
        },
        position: 0,
        location: TokenLocation {
            character: 0,
            line: 1
        }
    }
}

pub fn get_tokens(source: &String) -> Result<Vec<Token>, &'static str> {
    let mut tokens = Vec::new();
    
    let mut state = init(source);

    while {
        let token = get_next_token(&mut state);
        
        if token.is_err() {
            return Err(token.err().unwrap());
        }

        tokens.push(token.clone().ok().unwrap());

        token.ok().unwrap().token_type != TokenType::Eof
    } {}

    Ok(tokens)
}
