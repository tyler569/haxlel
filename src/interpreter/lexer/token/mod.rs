use std::fmt;
use std::option::Option;

#[derive(PartialEq, Clone, Copy)]
pub enum TokenType {
    BooleanTrue, BooleanFalse, Number, String,
    Identifier,
    Dot,
    Colon,
    Plus, Minus, Asterisk, Slash, Percentage, Approx,
    ReferenceEquals, Equals, NotEquals, Less, Greater, LessEquals, GreaterEquals,
    And, Or,
    Bracket, CurlyBracket,
    Assign,
    If, ElseIf, Else,
    Loop, ConditionalLoop,
    Return, Break, Continue,
    Function,
    LocalTerminator, GlobalTerminator
}

#[derive(PartialEq, Clone)]
pub struct TokenLocation {
    pub character: usize,
    pub line: usize
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub location: TokenLocation
}
