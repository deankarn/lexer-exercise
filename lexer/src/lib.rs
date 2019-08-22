use std::io;

#[derive(Debug, PartialEq)]
pub enum Token {
    String(Vec<u8>),
    Number(Vec<u8>),
    Bool(bool),
    Null,
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    InvalidJSON(Vec<u8>),
}

#[derive(Debug)]
pub struct Document<R> {
    reader: R,
    buff: [u8; 1],
    next: Option<Token>,
}

impl<R> Document<R>
where
    R: io::Read,
{
    #[inline]
    pub fn new(reader: R) -> Self {
        Self {
            reader,
            buff: [0],
            next: None,
        }
    }

    fn next_byte(&mut self) -> Option<Result<u8, io::Error>> {
        let res = self.reader.read(&mut self.buff);
        match res {
            Err(_) => Some(Err(res.err().unwrap())),
            Ok(bytes) => {
                if bytes == 0 {
                    None
                } else {
                    Some(Ok(self.buff[0]))
                }
            }
        }
    }
}

impl<R> Iterator for Document<R>
where
    R: io::Read,
{
    type Item = Result<Token, io::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_some() {
            return Some(Ok(self.next.take().unwrap()));
        }

        let result = self.next_byte()?;
        let mut c = match result {
            Ok(c) => c,
            Err(e) => return Some(Err(e)),
        };
        // eat whitespace
        if c == b' ' {
            'outer: loop {
                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    match c {
                        b' ' => continue,
                        _ => break 'outer,
                    }
                } else {
                    return None;
                }
            }
        }
        let result = match c {
            b'{' => Token::ObjectStart,
            b'}' => Token::ObjectEnd,
            b'[' => Token::ArrayStart,
            b']' => Token::ArrayEnd,
            b',' => Token::Comma,
            b':' => Token::Colon,
            b'"' => {
                let mut result = Vec::with_capacity(5);
                let mut prev_underscore = false;

                while let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    match c {
                        b'"' => {
                            if prev_underscore {
                                prev_underscore = false;
                                result.push(c);
                                continue;
                            }
                            return Some(Ok(Token::String(result)));
                        }
                        _ => {
                            if c == b'\\' {
                                prev_underscore = true;
                            }
                            result.push(c)
                        }
                    };
                }
                // if we get here there was no ending quote
                Token::InvalidJSON(result)
            }
            b't' => {
                let mut result = Vec::with_capacity(5);

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'r' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'u' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'e' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    match c {
                        b' ' | b'\n' => return Some(Ok(Token::Bool(true))),
                        b'}' => {
                            self.next = Some(Token::ObjectEnd);
                            return Some(Ok(Token::Bool(true)));
                        }
                        b']' => {
                            self.next = Some(Token::ArrayEnd);
                            return Some(Ok(Token::Bool(true)));
                        }
                        b',' => {
                            self.next = Some(Token::Comma);
                            return Some(Ok(Token::Bool(true)));
                        }
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                } else {
                    // if at end we've verified it's a good bool value already
                    Token::Bool(true)
                }
            }
            b'f' => {
                let mut result = Vec::with_capacity(5);

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'a' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'l' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b's' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'e' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    match c {
                        b' ' | b'\n' => return Some(Ok(Token::Bool(false))),
                        b'}' => {
                            self.next = Some(Token::ObjectEnd);
                            return Some(Ok(Token::Bool(false)));
                        }
                        b']' => {
                            self.next = Some(Token::ArrayEnd);
                            return Some(Ok(Token::Bool(false)));
                        }
                        b',' => {
                            self.next = Some(Token::Comma);
                            return Some(Ok(Token::Bool(false)));
                        }
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                } else {
                    // if at end we've verified it's a good bool value already
                    Token::Bool(false)
                }
            }
            b'n' => {
                let mut result = Vec::with_capacity(5);

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'u' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'l' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    result.push(c);
                    match c {
                        b'l' => {}
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                }

                if let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    match c {
                        b' ' | b'\n' => return Some(Ok(Token::Null)),
                        b'}' => {
                            self.next = Some(Token::ObjectEnd);
                            return Some(Ok(Token::Null));
                        }
                        b']' => {
                            self.next = Some(Token::ArrayEnd);
                            return Some(Ok(Token::Null));
                        }
                        b',' => {
                            self.next = Some(Token::Comma);
                            return Some(Ok(Token::Null));
                        }
                        _ => return Some(Ok(Token::InvalidJSON(result))),
                    };
                } else {
                    // if at end we've verified it's a good bool value already
                    Token::Null
                }
            }
            b'0'..=b'9' | b'-' | b'+' | b'.' | b'E' | b'e' => {
                let mut result = Vec::with_capacity(5);
                result.push(c);

                while let Some(current) = self.next_byte() {
                    c = match current {
                        Ok(c) => c,
                        Err(e) => return Some(Err(e)),
                    };
                    match c {
                        b' ' | b'\n' => {
                            return Some(Ok(Token::Number(result)));
                        }
                        b'}' => {
                            self.next = Some(Token::ObjectEnd);
                            return Some(Ok(Token::Number(result)));
                        }
                        b']' => {
                            self.next = Some(Token::ArrayEnd);
                            return Some(Ok(Token::Number(result)));
                        }
                        b',' => {
                            self.next = Some(Token::Comma);
                            return Some(Ok(Token::Number(result)));
                        }
                        _ => result.push(c),
                    };
                }
                return Some(Ok(Token::Number(result)));
            }
            _ => Token::InvalidJSON(Vec::new()),
        };
        Some(Ok(result))
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut lexer = Document::new("{}".as_bytes());

        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::ObjectStart);
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::ObjectEnd);
        assert_eq!(lexer.next().is_none(), true);
    }

    #[test]
    fn it_works_string() {
        let mut lexer = Document::new("\"test\"".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::String("test".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().is_none(), true);

        let mut lexer = Document::new("\"test".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::InvalidJSON(String::from("test").into_bytes())
        );
        assert_eq!(lexer.next().is_none(), true);

        let mut lexer = Document::new("\"test\\\"blah\\\"\"".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::String("test\\\"blah\\\"".as_bytes().to_vec())
        );
    }

    #[test]
    fn it_works_bool() {
        let mut lexer = Document::new("true".as_bytes());
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::Bool(true));
        assert_eq!(lexer.next().is_none(), true);

        let mut lexer = Document::new("false".as_bytes());
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::Bool(false));
        assert_eq!(lexer.next().is_none(), true);
    }

    #[test]
    fn it_works_null() {
        let mut lexer = Document::new("null".as_bytes());
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::Null);
        assert_eq!(lexer.next().is_none(), true);
    }

    #[test]
    fn it_works_number() {
        let mut lexer = Document::new("1".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::Number("1".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().is_none(), true);

        let mut lexer = Document::new("1.23".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::Number("1.23".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().is_none(), true);

        let mut lexer = Document::new("-1.23".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::Number("-1.23".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().is_none(), true);

        let mut lexer = Document::new("1.0E+2".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::Number("1.0E+2".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().is_none(), true);
    }

    #[test]
    fn it_works_whitespace() {
        let mut lexer = Document::new("   1".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::Number("1".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().is_none(), true);

        let mut lexer = Document::new("\"   test\"".as_bytes());
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::String("   test".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().is_none(), true);
    }

    #[test]
    fn it_works_comma_colon_whitespace() {
        let mut lexer = Document::new("{ \"key\":\"value\", \"key2\":\"value2\" }".as_bytes());
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::ObjectStart);
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::String("key".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::Colon);
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::String("value".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::Comma);
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::String("key2".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::Colon);
        assert_eq!(
            lexer.next().unwrap().ok().unwrap(),
            Token::String("value2".as_bytes().to_vec())
        );
        assert_eq!(lexer.next().unwrap().ok().unwrap(), Token::ObjectEnd);
        assert_eq!(lexer.next().is_none(), true);
    }
}
