use super::token::{Token, TokenKind};

pub fn parse<T: Parse>(iter: &mut dyn Iterator<Item = Token>) -> Result<T, String> {
    T::parse(iter.next().ok_or("unexpected EOF")?)
}

pub trait Parse
where
    Self: Sized,
{
    fn parse(token: Token) -> Result<Self, String>;
}

impl Parse for String {
    fn parse(token: Token) -> Result<Self, String> {
        match token.kind() {
            TokenKind::Str => Ok(token.value().into()),
            others => Err(format!(
                "invalid token type instead of `String`: {}",
                others
            )),
        }
    }
}

impl Parse for u32 {
    fn parse(token: Token) -> Result<Self, String> {
        match token.kind() {
            TokenKind::Literal => token.value().parse().map_err(|e| format!("{}", e)),
            others => Err(format!("invalid token type instead of `u32`: {}", others)),
        }
    }
}

pub struct Colon;
impl Parse for Colon {
    fn parse(token: Token) -> Result<Self, String> {
        match token.kind() {
            TokenKind::Colon => Ok(Colon),
            others => Err(format!("invalid token type instead of `Colon`: {}", others)),
        }
    }
}

pub struct SemiColon;
impl Parse for SemiColon {
    fn parse(token: Token) -> Result<Self, String> {
        match token.kind() {
            TokenKind::SemiColon => Ok(SemiColon),
            others => Err(format!("invalid token type instead of `Colon`: {}", others)),
        }
    }
}
pub struct Comma;
impl Parse for Comma {
    fn parse(token: Token) -> Result<Self, String> {
        match token.kind() {
            TokenKind::Comma => Ok(Comma),
            others => Err(format!("invalid token type instead of `Colon`: {}", others)),
        }
    }
}
