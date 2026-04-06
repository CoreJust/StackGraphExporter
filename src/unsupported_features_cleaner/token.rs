#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    Id {
        is_keyword: bool,
        is_type_keyword: bool,
    },
    Text {
        is_singleline: bool,
        is_multiline: bool,
        is_char: bool,
    },
    Comment {
        is_line: bool,
    },
    Other,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub ty: TokenType,
    pub text: &'a str,
    pub at: usize,
}

impl<'a> Token<'a> {
    pub fn is_keyword(&self) -> bool {
        matches!(
            self.ty,
            TokenType::Id {
                is_keyword: true,
                ..
            }
        )
    }
}
