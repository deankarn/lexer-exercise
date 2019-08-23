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
    IOError(String), // should be io::Error but....
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

    fn next_byte(&mut self) -> Result<u8, Option<io::Error>> {
        let res = self.reader.read(&mut self.buff);
        match res {
            Err(e) => Err(Some(e)),
            Ok(bytes) => {
                if bytes == 0 {
                    Err(None)
                } else {
                    Ok(self.buff[0])
                }
            }
        }
    }
}

macro_rules! next_byte {
    ($slf:ident, $ret:expr) => {
        match $slf.next_byte() {
            Ok(c) => c,
            Err(e) => match e {
                Some(e) => return Some(Token::IOError(e.to_string())),
                None => return $ret,
            },
        };
    };
}

impl<R> Iterator for Document<R>
where
    R: io::Read,
{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_some() {
            return Some(self.next.take().unwrap());
        }

        //        let mut c = match self.next_byte() {
        //            Ok(c) => c,
        //            Err(e) => match e {
        //                Some(e) => Ok(Token::IOError(e.to_string())),
        //                None => return None,
        //            },
        //        };
        let mut c = next_byte!(self, None);

        // eat whitespace
        if c == b' ' {
            loop {
                c = next_byte!(self, None);
                match c {
                    b' ' => continue,
                    _ => break,
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
                let mut prev_backslash = false;

                loop {
                    c = next_byte!(self, Some(Token::InvalidJSON(result))); // if we hit the end here there was no ending quote
                    match c {
                        b'"' => {
                            if prev_backslash {
                                prev_backslash = false;
                                result.push(c);
                                continue;
                            }
                            return Some(Token::String(result));
                        }
                        _ => {
                            if c == b'\\' {
                                prev_backslash = true;
                            }
                            result.push(c)
                        }
                    };
                }
                //                Token::InvalidJSON(result)
            }
            b't' => {
                let mut result = Vec::with_capacity(5);
                result.push(c);

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'r' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'u' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'e' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                //                c = match self.next_byte() {
                //                    NextByte::None => return Some(Token::Bool(true)),
                //                    NextByte::IOError(e) => return Some(e),
                //                    NextByte::Result(c) => c,
                //                };
                //                c = match self.next_byte() {
                //                    Ok(c) => c,
                //                    Err(e) => match e {
                //                        Some(e) => Ok(Token::IOError(e.to_string())),
                //                        None => return Some(Token::Bool(true)),
                //                    },
                //                };
                c = next_byte!(self, Some(Token::Bool(true)));
                match c {
                    b' ' | b'\n' => return Some(Token::Bool(true)),
                    b'}' => {
                        self.next = Some(Token::ObjectEnd);
                        return Some(Token::Bool(true));
                    }
                    b']' => {
                        self.next = Some(Token::ArrayEnd);
                        return Some(Token::Bool(true));
                    }
                    b',' => {
                        self.next = Some(Token::Comma);
                        return Some(Token::Bool(true));
                    }
                    _ => return Some(Token::InvalidJSON(result)),
                };
            }
            b'f' => {
                let mut result = Vec::with_capacity(5);
                result.push(c);

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'a' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'l' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b's' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'e' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                //                c = match self.next_wrap() {
                //                    NextByte::None => return Some(Token::Bool(false)),
                //                    NextByte::IOError(e) => return Some(e),
                //                    NextByte::Result(c) => c,
                //                };
                c = next_byte!(self, Some(Token::Bool(false)));
                match c {
                    b' ' | b'\n' => return Some(Token::Bool(false)),
                    b'}' => {
                        self.next = Some(Token::ObjectEnd);
                        return Some(Token::Bool(false));
                    }
                    b']' => {
                        self.next = Some(Token::ArrayEnd);
                        return Some(Token::Bool(false));
                    }
                    b',' => {
                        self.next = Some(Token::Comma);
                        return Some(Token::Bool(false));
                    }
                    _ => return Some(Token::InvalidJSON(result)),
                };
            }
            b'n' => {
                let mut result = Vec::with_capacity(5);

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'u' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'l' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                c = next_byte!(self, None);
                result.push(c);
                match c {
                    b'l' => {}
                    _ => return Some(Token::InvalidJSON(result)),
                };

                //                c = match self.next_wrap() {
                //                    NextByte::None => return Some(Token::Null),
                //                    NextByte::IOError(e) => return Some(e),
                //                    NextByte::Result(c) => c,
                //                };
                c = next_byte!(self, Some(Token::Null));
                match c {
                    b' ' | b'\n' => return Some(Token::Null),
                    b'}' => {
                        self.next = Some(Token::ObjectEnd);
                        return Some(Token::Null);
                    }
                    b']' => {
                        self.next = Some(Token::ArrayEnd);
                        return Some(Token::Null);
                    }
                    b',' => {
                        self.next = Some(Token::Comma);
                        return Some(Token::Null);
                    }
                    _ => return Some(Token::InvalidJSON(result)),
                };
            }
            b'0'..=b'9' | b'-' | b'+' | b'.' | b'E' | b'e' => {
                let mut result = Vec::with_capacity(5);
                result.push(c);

                loop {
                    c = next_byte!(self, Some(Token::Number(result)));
                    //                    c = match self.next_wrap() {
                    //                        NextByte::None => return Some(Token::Number(result)),
                    //                        NextByte::IOError(e) => return Some(e),
                    //                        NextByte::Result(c) => c,
                    //                    };
                    match c {
                        b' ' | b'\n' => {
                            return Some(Token::Number(result));
                        }
                        b'}' => {
                            self.next = Some(Token::ObjectEnd);
                            return Some(Token::Number(result));
                        }
                        b']' => {
                            self.next = Some(Token::ArrayEnd);
                            return Some(Token::Number(result));
                        }
                        b',' => {
                            self.next = Some(Token::Comma);
                            return Some(Token::Number(result));
                        }
                        _ => result.push(c),
                    };
                }
            }
            _ => Token::InvalidJSON(Vec::new()),
        };
        Some(result)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_works() {
        let mut lexer = Document::new("{}".as_bytes());
        assert_eq!(lexer.next(), Some(Token::ObjectStart));
        assert_eq!(lexer.next(), Some(Token::ObjectEnd));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn it_works_string() {
        let mut lexer = Document::new("\"test\"".as_bytes());
        assert_eq!(
            lexer.next(),
            Some(Token::String("test".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("\"test".as_bytes());
        assert_eq!(
            lexer.next(),
            Some(Token::InvalidJSON(String::from("test").into_bytes()))
        );
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("\"test\\\"blah\\\"\"".as_bytes());
        assert_eq!(
            lexer.next(),
            Some(Token::String("test\\\"blah\\\"".as_bytes().to_vec()))
        );
    }

    #[test]
    fn it_works_bool() {
        let mut lexer = Document::new("true".as_bytes());
        assert_eq!(lexer.next(), Some(Token::Bool(true)));
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("false".as_bytes());
        assert_eq!(lexer.next(), Some(Token::Bool(false)));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn it_works_null() {
        let mut lexer = Document::new("null".as_bytes());
        assert_eq!(lexer.next(), Some(Token::Null));
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn it_works_number() {
        let mut lexer = Document::new("1".as_bytes());
        assert_eq!(lexer.next(), Some(Token::Number("1".as_bytes().to_vec())));
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("1.23".as_bytes());
        assert_eq!(
            lexer.next(),
            Some(Token::Number("1.23".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("-1.23".as_bytes());
        assert_eq!(
            lexer.next(),
            Some(Token::Number("-1.23".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("1.0E+2".as_bytes());
        assert_eq!(
            lexer.next(),
            Some(Token::Number("1.0E+2".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn it_works_whitespace() {
        let mut lexer = Document::new("   1".as_bytes());
        assert_eq!(lexer.next(), Some(Token::Number("1".as_bytes().to_vec())));
        assert_eq!(lexer.next(), None);

        let mut lexer = Document::new("\"   test\"".as_bytes());
        assert_eq!(
            lexer.next(),
            Some(Token::String("   test".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), None);
    }

    #[test]
    fn it_works_comma_colon_whitespace() {
        let mut lexer = Document::new("{ \"key\":\"value\", \"key2\":\"value2\" }".as_bytes());
        assert_eq!(lexer.next(), Some(Token::ObjectStart));
        assert_eq!(lexer.next(), Some(Token::String("key".as_bytes().to_vec())));
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(
            lexer.next(),
            Some(Token::String("value".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), Some(Token::Comma));
        assert_eq!(
            lexer.next(),
            Some(Token::String("key2".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), Some(Token::Colon));
        assert_eq!(
            lexer.next(),
            Some(Token::String("value2".as_bytes().to_vec()))
        );
        assert_eq!(lexer.next(), Some(Token::ObjectEnd));
        assert_eq!(lexer.next(), None);
    }
}
