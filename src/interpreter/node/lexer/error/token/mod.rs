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
    Break,
    Function,
    LocalTerminator, GlobalTerminator,
    Eof
}

impl ToString for TokenType {
    fn to_string(&self) -> String {
        String::from(match *self {
            TokenType::BooleanTrue => "BooleanTrue",
            TokenType::BooleanFalse => "BooleanFalse",
            TokenType::Number => "Number",
            TokenType::String => "String",
            TokenType::Identifier => "Identifier",
            TokenType::Dot => "Dot",
            TokenType::Colon => "Colon",
            TokenType::Plus => "Plus",
            TokenType::Minus => "Minus",
            TokenType::Asterisk => "Asterisk",
            TokenType::Slash => "Slash",
            TokenType::Percentage => "Percentage",
            TokenType::Approx => "Approx",
            TokenType::ReferenceEquals => "ReferenceEquals",
            TokenType::Equals => "Equals",
            TokenType::NotEquals => "NotEquals",
            TokenType::Less => "Less",
            TokenType::Greater => "Greater",
            TokenType::LessEquals => "LessEquals",
            TokenType::GreaterEquals => "GreaterEquals",
            TokenType::And => "And",
            TokenType::Or => "Or",
            TokenType::Bracket => "Bracket",
            TokenType::CurlyBracket => "CurlyBracket",
            TokenType::Assign => "Assign",
            TokenType::If => "If",
            TokenType::ElseIf => "ElseIf",
            TokenType::Else => "Else",
            TokenType::Loop => "Loop",
            TokenType::ConditionalLoop => "ConditionalLoop",
            TokenType::Break => "Break",
            TokenType::Function => "Function",
            TokenType::LocalTerminator => "LocalTerminator",
            TokenType::GlobalTerminator => "GlobalTerminator",
            TokenType::Eof => "Eof"
        })
    }
}

#[derive(PartialEq, Clone)]
pub struct TokenLocation {
    pub character: usize,
    pub line: usize
}

impl ToString for TokenLocation {
    fn to_string(&self) -> String {
        format!("{}, {}", self.character, self.line)
    }
}

#[derive(PartialEq, Clone)]
pub enum TokenError {
    String(TokenLocation),
    NumberDigit(TokenLocation),
    NumberPoint(TokenLocation),
    Character(TokenLocation),
    Undefined
}

#[derive(PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub location: TokenLocation
}
