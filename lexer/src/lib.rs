use std::iter::Peekable;
use std::str::CharIndices;

#[derive(Debug, PartialEq)]
pub enum Token {
    String {
        start: usize,
        end: usize,
    },
    Number {
        start: usize,
        end: usize,
    },
    Bool {
        start: usize,
        end: usize,
        value: bool,
    },
    Null {
        index: usize,
    },
    ObjectStart {
        index: usize,
    },
    ObjectEnd {
        index: usize,
    },
    ArrayStart {
        index: usize,
    },
    ArrayEnd {
        index: usize,
    },
    Colon {
        index: usize,
    },
    Comma {
        index: usize,
    },
    Whitespace {
        start: usize,
        end: usize,
    },
    InvalidJSON {
        index: usize,
    },
}

#[derive(Debug)]
pub struct Document<'a> {
    chars: Peekable<CharIndices<'a>>,
}

impl<'a> Document<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.char_indices().peekable(),
        }
    }
}

impl<'a> Iterator for Document<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, c) = self.chars.next()?;
        let result = match c {
            '{' => Token::ObjectStart { index },
            '}' => Token::ObjectEnd { index },
            '[' => Token::ArrayStart { index },
            ']' => Token::ArrayEnd { index },
            '"' => loop {
                if let Some((end, c)) = self.chars.next() {
                    if c == '"' {
                        return Some(Token::String { start: index, end });
                    }
                } else {
                    return Some(Token::InvalidJSON { index });
                }
            },
            't' => {
                if let Some((end, c)) = self.chars.next() {
                    if c != 'r' {
                        return Some(Token::InvalidJSON { index });
                    }
                } else {
                    return Some(Token::InvalidJSON { index });
                }

                if let Some((end, c)) = self.chars.next() {
                    if c != 'u' {
                        return Some(Token::InvalidJSON { index });
                    }
                } else {
                    return Some(Token::InvalidJSON { index });
                }

                if let Some((end, c)) = self.chars.next() {
                    if c != 'e' {
                        return Some(Token::InvalidJSON { index });
                    }
                } else {
                    return Some(Token::InvalidJSON { index });
                }

                let (i, c) = self.chars.peek()?;
                return Some(match c {
                    ' ' | '\n' | '}' | ']' | ',' => Token::Bool {
                        start: index,
                        end: *i,
                        value: true,
                    },
                    _ => Token::InvalidJSON { index },
                });
            }
            //            't' | 'f' => Token::MaybeBool(index, hint: c == 't'),
            //            '0'..='9' => loop {
            //                if let Some((end, c)) = self.chars.next() {
            //                    if c == '"' {
            //                        return Some(Token::String { start: index, end });
            //                    }
            //                } else {
            //                    return Some(Token::InvalidJSON { index });
            //                }
            //            },
            _ => Token::InvalidJSON { index },
        };
        Some(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut lexer = Document::new("{}");

        assert_eq!(lexer.next(), Some(Token::ObjectStart { index: 0 }));
        assert_eq!(lexer.next(), Some(Token::ObjectEnd { index: 1 }));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn it_works_string() {
        let mut lexer = Document::new("\"test\"");
        assert_eq!(lexer.next(), Some(Token::String { start: 0, end: 5 }));
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("\"test");
        assert_eq!(lexer.next(), Some(Token::InvalidJSON { index: 0 }));
        assert_eq!(lexer.next(), None);
    }
    #[test]
    fn it_works_bool() {
        let mut lexer = Document::new("true ");
        assert_eq!(
            lexer.next(),
            Some(Token::Bool {
                start: 0,
                end: 4,
                value: true
            })
        );
        assert_eq!(lexer.next(), Some(Token::InvalidJSON { index: 4 }));
        assert_eq!(lexer.next(), None);
    }
}
