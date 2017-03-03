pub mod token;
use self::token::TokenLocation;

#[derive(PartialEq, Clone)]
pub enum Error {
    String(TokenLocation),
    NumberDigit(TokenLocation),
    NumberPoint(TokenLocation),
    Character(TokenLocation),
    DefaultNode(TokenLocation),
    Undefined
}

impl ToString for Error {
    fn to_string(&self) -> String {
        match *self {
            Error::String(ref a) => format!("String({})", a.to_string()),
            Error::NumberDigit(ref a) => format!("NumberDigit({})", a.to_string()),
            Error::NumberPoint(ref a) => format!("NumberPoint({})", a.to_string()),
            Error::Character(ref a) => format!("Character({})", a.to_string()),
            Error::DefaultNode(ref a) => format!("DefaultNode({})", a.to_string()),
            Error::Undefined => String::from("Undefined")
        }
    }
}
