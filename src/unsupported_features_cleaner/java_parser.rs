use crate::unsupported_features_cleaner::constants;
use crate::unsupported_features_cleaner::token::{Token, TokenType};

pub struct JavaParser<'a> {
    src: &'a str,
    pos: usize,
}

impl<'a> JavaParser<'a> {
    pub fn new(src: &'a str) -> Self {
        JavaParser { src, pos: 0 }
    }

    fn skip_whitespace(&mut self) {
        let bytes = self.src.as_bytes();
        while self.pos < bytes.len()
            && matches!(bytes[self.pos], b' ' | b'\t' | b'\r' | b'\n' | b'\x0C')
        {
            self.pos += 1;
        }
    }

    fn read_line_comment(&mut self) -> Token<'a> {
        let start = self.pos - 2;
        let bytes = self.src.as_bytes();
        while self.pos < bytes.len() && bytes[self.pos] != b'\n' {
            self.pos += 1;
        }
        Token {
            ty: TokenType::Comment { is_line: true },
            text: &self.src[start..self.pos],
            at: start,
        }
    }

    fn read_block_comment(&mut self) -> Option<Token<'a>> {
        let start = self.pos - 2;
        let bytes = self.src.as_bytes();
        while self.pos + 1 < bytes.len() {
            if bytes[self.pos] == b'*' && bytes[self.pos + 1] == b'/' {
                self.pos += 2;
                return Some(Token {
                    ty: TokenType::Comment { is_line: false },
                    text: &self.src[start..self.pos],
                    at: start,
                });
            }
            self.pos += 1;
        }
        None
    }

    fn read_string(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        let bytes = self.src.as_bytes();
        self.pos += 1; // skip "
        while self.pos < bytes.len() {
            match bytes[self.pos] {
                b'\\' => self.pos += 2,
                b'"' => {
                    self.pos += 1;
                    return Some(Token {
                        ty: TokenType::Text {
                            is_singleline: true,
                            is_multiline: false,
                            is_char: false,
                        },
                        text: &self.src[start..self.pos],
                        at: start,
                    });
                }
                _ => self.pos += 1,
            }
        }
        None
    }

    fn read_char(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        let bytes = self.src.as_bytes();
        self.pos += 1; // skip '
        while self.pos < bytes.len() {
            match bytes[self.pos] {
                b'\\' => self.pos += 2,
                b'\'' => {
                    self.pos += 1;
                    return Some(Token {
                        ty: TokenType::Text {
                            is_singleline: false,
                            is_multiline: false,
                            is_char: true,
                        },
                        text: &self.src[start..self.pos],
                        at: start,
                    });
                }
                _ => self.pos += 1,
            }
        }
        None
    }

    fn read_text_block(&mut self) -> Option<Token<'a>> {
        let start = self.pos;
        let bytes = self.src.as_bytes();
        self.pos += 3; // skip """
        while self.pos + 2 < bytes.len() {
            if bytes[self.pos] == b'"' && bytes[self.pos + 1] == b'"' && bytes[self.pos + 2] == b'"'
            {
                self.pos += 3;
                return Some(Token {
                    ty: TokenType::Text {
                        is_singleline: false,
                        is_multiline: true,
                        is_char: false,
                    },
                    text: &self.src[start..self.pos],
                    at: start,
                });
            }
            self.pos += 1;
        }
        None
    }

    fn is_text_block_start(&self) -> bool {
        let bytes = self.src.as_bytes();
        self.pos + 2 < bytes.len()
            && bytes[self.pos] == b'"'
            && bytes[self.pos + 1] == b'"'
            && bytes[self.pos + 2] == b'"'
    }

    fn is_ident_start(b: u8) -> bool {
        matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'_' | b'$')
    }

    fn is_ident_continue(b: u8) -> bool {
        matches!(b, b'a'..=b'z' | b'A'..=b'Z' | b'0'..=b'9' | b'_' | b'$')
    }

    fn read_identifier(&mut self) -> Token<'a> {
        let start = self.pos;
        let bytes = self.src.as_bytes();
        while self.pos < bytes.len() && Self::is_ident_continue(bytes[self.pos]) {
            self.pos += 1;
        }
        let ident = &self.src[start..self.pos];
        let is_keyword = constants::JAVA_KEYWORDS.contains(&ident);
        let is_type_keyword = constants::JAVA_TYPE_KEYWORDS.contains(&ident);
        Token {
            ty: TokenType::Id {
                is_keyword,
                is_type_keyword,
            },
            text: ident,
            at: start,
        }
    }

    fn read_other(&mut self) -> Token<'a> {
        let start = self.pos;
        self.pos += 1;
        Token {
            ty: TokenType::Other,
            text: &self.src[start..self.pos],
            at: start,
        }
    }

    pub fn next(&mut self) -> Option<Token<'a>> {
        loop {
            self.skip_whitespace();
            if self.pos >= self.src.len() {
                return None;
            }
            let bytes = self.src.as_bytes();
            let ch = bytes[self.pos];
            match ch {
                b'/' if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'/' => {
                    self.pos += 2;
                    return Some(self.read_line_comment());
                }
                b'/' if self.pos + 1 < bytes.len() && bytes[self.pos + 1] == b'*' => {
                    self.pos += 2;
                    if let Some(tok) = self.read_block_comment() {
                        return Some(tok);
                    }
                    return None;
                }
                b'"' => {
                    if self.is_text_block_start() {
                        if let Some(tok) = self.read_text_block() {
                            return Some(tok);
                        }
                    } else if let Some(tok) = self.read_string() {
                        return Some(tok);
                    }
                    return None;
                }
                b'\'' => {
                    if let Some(tok) = self.read_char() {
                        return Some(tok);
                    }
                    return None;
                }
                _ if Self::is_ident_start(ch) => {
                    return Some(self.read_identifier());
                }
                _ => {
                    return Some(self.read_other());
                }
            }
        }
    }

    pub fn find_matching_brace_end(&mut self, open_pos: usize) -> Option<usize> {
        if self.src.as_bytes().get(open_pos) != Some(&b'{') {
            return None;
        }
        let mut depth = 1;
        let mut parser = JavaParser::new(&self.src[open_pos + 1..]);
        while let Some(token) = parser.next() {
            match token.text {
                "{" => depth += 1,
                "}" => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(open_pos + 1 + token.at + token.text.len());
                    }
                }
                _ => {}
            }
        }
        None
    }
}
