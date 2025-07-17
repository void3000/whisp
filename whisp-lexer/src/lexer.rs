use std::str::Chars;
use crate::token::Token;

/// Lessons:
/// (1) Avoid use of generics on traits, especially
///     if th return value is a generic. Rather use
///     associate types.
pub trait TokenIterator {
    type Item;

    fn next(&mut self) -> Result<Self::Item, String>;
}

pub trait Collection {
    type Iter;

    fn stream(&self) -> Self::Iter;
}

pub struct Stream<'a> {
    chars: std::iter::Peekable<Chars<'a>>
}

pub struct Lexer<'a> {
    input: &'a str
}

impl<'a> TokenIterator for Stream<'a> {
    type Item = Token;

    fn next(&mut self) -> Result<Self::Item, String> {
        loop {
            // Skip whitespace
            while let Some(&c) = self.chars.peek() {
                if c.is_whitespace() {
                    self.chars.next();
                } else {
                    break;
                }
            }

            // Skip comments starting with '#'
            if let Some('#') = self.chars.peek().copied() {
                while let Some(c) = self.chars.next() {
                    if c == '\n' {
                        break;
                    }
                }
            } else {
                break;
            }
        }

        let result = match self.chars.next() {
            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,
            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some('+') => Token::Add,
            Some('-') => Token::Sub,
            Some('*') => Token::Mul,
            Some('/') => Token::Div,
            Some('%') => Token::Mod,
            Some('=') => {
                if let Some('=') = self.chars.peek() {
                    self.chars.next();
                    Token::Equal
                } else {
                    Token::Assign
                }
            },
            Some('>') => {
                if let Some('=') = self.chars.peek() {
                    self.chars.next();
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            },
            Some('<') => {
                if let Some('=') = self.chars.peek() {
                    self.chars.next();
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            },
            Some('"') => {
                let mut string_lit = String::new();

                while let Some(&next) = self.chars.peek() {
                    self.chars.next();
                    match next {
                        '"' => break,
                        '\\' => {
                            if let Some(&escaped) = self.chars.peek() {
                                self.chars.next();
                                match escaped {
                                    'n' => string_lit.push('\n'),
                                    't' => string_lit.push('\t'),
                                    '"' => string_lit.push('"'),
                                    '\\' => string_lit.push('\\'),
                                    other => {
                                        return Err(format!("Invalid escape character: \\{}", other));
                                    }
                                }
                            } else {
                                return Err("Unterminated escape sequence.".to_string());
                            }
                        }
                        other => string_lit.push(other),
                    }
                }

                Token::String(string_lit)
            },
            Some(':') => {
                match self.chars.next() {
                    Some(':') => Token::DoubleColon,
                    _ => return Err("Unexpected single ':'.".to_string())
                }
            },
            None => Token::Eof,
            Some(c) => {
                // Numeric
                if c.is_ascii_digit() {
                    let mut number = c.to_string();
                    while let Some(&next) = self.chars.peek() {
                        if next.is_ascii_digit() {
                            number.push(self.chars
                                    .next()
                                    .unwrap());
                        } else {
                            break;
                        }
                    }

                    let numeric_literal = number.parse::<i32>();
                    if numeric_literal.is_ok() {
                        return Ok(Token::Int(numeric_literal.unwrap()));
                    }

                    return Err("unrecognozed numeric literal.".to_string());
                }

                // Keywords, identifiers
                if c.is_ascii_alphabetic() || c == '_' {
                    let mut ident = c.to_string();
                    while let Some(&next) = self.chars.peek() {
                        if next.is_ascii_alphanumeric() || next == '_' {
                            ident.push(self.chars.next().unwrap());
                        } else {
                            break;
                        }
                    }

                    let keyword_or_ident = match ident.as_str() {
                        "import"    => Token::Import,
                        "let"       => Token::Let,
                        "if"        => Token::If,
                        "elif"      => Token::Elif,
                        "else"      => Token::Else,
                        "while"     => Token::While,
                        "for"       => Token::For,
                        "in"        => Token::In,
                        "def"       => Token::Def,
                        "return"    => Token::Return,
                        "array"     => Token::Array,
                        "and"       => Token::And,
                        "or"        => Token::Or,
                        "true"      => Token::Bool(true),
                        "false"     => Token::Bool(false),
                        _           => Token::Identifier(ident),
                    };
                    return Ok(keyword_or_ident);
                }
                return Err(format!("unrecognized character '{}' found.", c));
            }
        };
        Ok(result)
    }
}

impl<'a> Collection for Lexer<'a> {
    type Iter = Stream<'a>;

    fn stream(&self) -> Self::Iter {
        Stream {
            chars: self.input.chars().peekable(),
        }
    }
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}


#[cfg(test)]
mod tokenizer_tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_stream() {
        let lexer = Lexer::new("
            let x = 5;

            x + 1;
        ");
        let expt_tokens = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Int(5),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Add,
            Token::Int(1),
            Token::Semicolon,
            Token::Eof
        ];

        let mut stream = lexer.stream();
        let mut token = stream.next();
        let mut index = 0;

        assert!(token.is_ok());

        while token != Ok(Token::Eof) {
            assert_eq!(token.unwrap(), *expt_tokens.get(index).unwrap());
            index = index + 1;
            token = stream.next();
        }
    }

    #[test]
    fn test_unrecognized_token() {
        let lexer = Lexer::new("$");

        let mut stream = lexer.stream();
        let mut token = stream.next();
        
        assert!(token.is_err());
        assert!(token.unwrap_err().contains("unrecognized character '$' found."));
    }

    #[test]
    fn test_import_statement_tokens() {
        let lexer = Lexer::new("import path1::path2::sort;");
        let expt_tokens = vec![
            Token::Import,
            Token::Identifier("path1".to_string()),
            Token::DoubleColon,
            Token::Identifier("path2".to_string()),
            Token::DoubleColon,
            Token::Identifier("sort".to_string()),
            Token::Semicolon
        ];

        let mut stream = lexer.stream();
        let mut token = stream.next();

        assert!(token.is_ok());

        let mut index = 0;

        while token != Ok(Token::Semicolon) {
            assert_eq!(token.unwrap(), *expt_tokens.get(index).unwrap());
            index = index + 1;
            token = stream.next();
        }
    }

    #[test]
    fn test_single_comment() {
        let lexer = Lexer::new("
            # This is a comment
            let x = 10;
        ");
        let expected = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Int(10),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut stream = lexer.stream();
        for token in expected {
            assert_eq!(stream.next().unwrap(), token);
        }
    }

    #[test]
    fn test_multiple_consecutive_comments() {
        let lexer = Lexer::new("
            # comment one
            # comment two
            let x = 1;
        ");
        let expected = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Int(1),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut stream = lexer.stream();
        for token in expected {
            assert_eq!(stream.next().unwrap(), token);
        }
    }

    #[test]
    fn test_comment_between_tokens() {
        let lexer = Lexer::new("
            let x = 1; # a comment here
            x + 2;
        ");
        let expected = vec![
            Token::Let,
            Token::Identifier("x".to_string()),
            Token::Assign,
            Token::Int(1),
            Token::Semicolon,
            Token::Identifier("x".to_string()),
            Token::Add,
            Token::Int(2),
            Token::Semicolon,
            Token::Eof,
        ];

        let mut stream = lexer.stream();
        for token in expected {
            assert_eq!(stream.next().unwrap(), token);
        }
    }

    #[test]
    fn test_comment_inside_function() {
        let lexer = Lexer::new("
            def f() {
                # comment inside body
                # comment inside body
                return 42;
            }
        ");
        let expected = vec![
            Token::Def,
            Token::Identifier("f".to_string()),
            Token::LParen,
            Token::RParen,
            Token::LBrace,
            Token::Return,
            Token::Int(42),
            Token::Semicolon,
            Token::RBrace,
            Token::Eof,
        ];

        let mut stream = lexer.stream();
        for token in expected {
            assert_eq!(stream.next().unwrap(), token);
        }
    }
}
