use std::iter::Peekable;
use std::str::Chars;
use crate::bibtex::error::ParseError;

#[derive(Debug, PartialEq)]
pub enum Token {
    At, OpenBrace, CloseBrace, Comma, Equals, Ident(String), 
    String(String), Eof, 
}

impl Token {
    pub fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }
}

#[derive(Debug)]
pub struct TokenPacket {
    token: Token, 
    row: u32, 
    col: u32,
}

type TokenStream = Vec<TokenPacket>;

pub struct Parser<'a> {
    input: Peekable<Chars<'a>>, 
    row: u32, col: u32, 
    brace_level: u16,  
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser { 
            input: input.chars().peekable(), 
            row: 1, col: 1, brace_level: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<TokenStream, ParseError> {
        let mut tokens = TokenStream::new();

        loop {
            self.skip_whitespace_and_comments();
            let token = self.consume_token();
            
        }
    }

    fn advance(&mut self) -> Option<char> {
        match self.input.next() {
            Some('\n') => {
                self.row += 1;
                self.col = 1;
                Some('\n')
            }
            Some(c) => {
                self.col += 1;
                Some(c)
            }
            None => None, 
        }
    }

    fn peek(&mut self) -> Option<&char> {
        self.input.peek()
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some('%') => {
                    self.advance();
                    while let Some(c) = self.advance() {
                        if c == '\n' {
                            break;
                        }
                    }
                }
                Some(c) if c.is_ascii_whitespace() => {
                    self.advance();
                }
                _ => break,
            }
        }
    }

    fn consume_token(&mut self) -> Token {
        if let Some(c) = self.peek() {
            let token_kind = match c {
                '@' => Token::At, 
                '{' => { self.increment_brace_level(); Token::OpenBrace }, 
                '}' => { self.decrement_brace_level(); Token::CloseBrace }, 
                ',' => { Token::Comma }, 
                _ => todo!()
            }
        } else {
            Token::Eof
        }
    }

    fn increment_brace_level(&mut self) {
        self.brace_level += 1;
    }

    fn decrement_brace_level(&mut self) -> Result<(), ParseError> {
        self.brace_level -= 1;
        if self.brace_level < 0 {

        }
    }
}