use std::fmt::Display;

/// All the type of Tokens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    Colon,     // :
    SemiColon, // ;
    Comma,     // ,
    Str,       // Text
    Literal,   // 42
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Colon => write!(f, ":"),
            TokenKind::SemiColon => write!(f, ";"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Str => write!(f, "str"),
            TokenKind::Literal => write!(f, "literal"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    kind: TokenKind,
    slice: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(kind: TokenKind, slice: &'a str) -> Self {
        Self { kind, slice }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    pub fn value(&self) -> &'a str {
        self.slice
    }
}
