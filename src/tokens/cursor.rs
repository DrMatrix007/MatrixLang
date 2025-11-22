use std::str::Chars;

pub struct Cursor<'a> {
    string: &'a str,
    chars: Chars<'a>,
    remains: i64,
}

impl<'a> Cursor<'a> {
    pub fn new(data: &'a str) -> Self {
        Self {
            string: data,
            chars: data.chars(),
            remains: data.len() as _,
        }
    }

    pub fn advance(&mut self) -> Option<char> {
        self.chars.next()
    }

    pub fn peek_first(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn reset_token_start(&mut self) {
        self.remains = self.chars.as_str().len() as _;
    }

    pub fn get_current_token_len(&self) -> i64 {
        self.remains - self.chars.as_str().len() as i64
    }

    pub fn advance_while(&mut self, mut f: impl FnMut(char) -> bool) -> usize {
        let mut counter = 0;
        while let Some(ch) = self.peek_first()
            && f(ch)
        {
            self.advance();
            counter += 1;
        }

        counter
    }
    pub fn advance_while_inc_str(&mut self, mut f: impl FnMut(&str) -> bool) {
        let base_str = self.chars.as_str();
        let mut chars = base_str.char_indices();

        while let Some(_) = chars.next() {
            if f(&base_str[0..(chars.offset())]) {
                self.advance();
            } else {
                break;
            }
        }
    }

    pub fn string(&self) -> &'a str {
        self.string
    }

    pub fn remains(&self) -> &'a str {
        self.chars.as_str()
    }
}
