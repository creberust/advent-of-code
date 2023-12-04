use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::token::{Token, TokenKind};

/// Lexer over a buffer that returns Tokens.
pub struct Lexer<'b> {
    buffer: &'b str,
    inner: Peekable<Enumerate<Chars<'b>>>,
}

impl<'b> Lexer<'b> {
    pub fn new(buffer: &'b str) -> Self {
        Self {
            buffer,
            inner: buffer.chars().enumerate().peekable(),
        }
    }

    fn next_token(&mut self) -> Option<Token<'b>> {
        let (index, next) = self.inner.peek()?;
        let index = *index;

        match *next {
            c if char::is_ascii_digit(&c) => {
                Some(Token::new(TokenKind::Literal, self.parse_number(index)))
            }
            c if char::is_alphabetic(c) => Some(Token::new(TokenKind::Str, self.parse_str(index))),
            c if char::is_whitespace(c) => {
                self.consume_whitespaces();
                self.next_token()
            }
            ':' => {
                self.inner.next();
                Some(Token::new(TokenKind::Colon, &self.buffer[index..index + 1]))
            }
            ';' => {
                self.inner.next();
                Some(Token::new(
                    TokenKind::SemiColon,
                    &self.buffer[index..index + 1],
                ))
            }
            ',' => {
                self.inner.next();
                Some(Token::new(TokenKind::Comma, &self.buffer[index..index + 1]))
            }
            others => panic!("unexpected token: {}", others),
        }
    }

    fn parse_str(&mut self, start: usize) -> &'b str {
        let mut end = start;

        while let Some((_, c)) = self.inner.peek() {
            if char::is_alphabetic(*c) {
                self.inner.next();
                end += 1;
            } else {
                break;
            }
        }

        &self.buffer[start..end]
    }

    fn parse_number(&mut self, start: usize) -> &'b str {
        let mut end = start;

        while let Some((_, c)) = self.inner.peek() {
            if char::is_ascii_digit(c) {
                self.inner.next();
                end += 1;
            } else {
                break;
            }
        }

        &self.buffer[start..end]
    }

    fn consume_whitespaces(&mut self) {
        while let Some((_, c)) = self.inner.peek() {
            if char::is_whitespace(*c) {
                self.inner.next();
            } else {
                break;
            }
        }
    }
}

impl<'b> Iterator for Lexer<'b> {
    type Item = Token<'b>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        // Given
        let buffer = "Game 1: 3 blue, 42 red; 1 red, 2 green, 6 blue; 2 green";
        let mut lexer = Lexer::new(buffer);

        // When
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Str, "Game")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Literal, "1")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Colon, ":")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Literal, "3")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Str, "blue")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Comma, ",")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Literal, "42")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Str, "red")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::SemiColon, ";")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Literal, "1")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Str, "red")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Comma, ",")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Literal, "2")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Str, "green")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Comma, ",")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Literal, "6")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Str, "blue")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::SemiColon, ";")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Literal, "2")));
        assert_eq!(lexer.next(), Some(Token::new(TokenKind::Str, "green")));

        // Then
        assert_eq!(lexer.next(), None);
    }
}
