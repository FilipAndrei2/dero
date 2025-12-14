#[derive(Debug)]
pub enum LexiToken {
    // Literals
    LitString(String),
    LitInt(i32),
    LitFloat(f64),

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eq,
    Bang,

    // Punctuation
    Semi,
    Dot,
    OpenBrace,
    CloseBrace,
    OpenCurly,
    CloseCurly,
    OpenSquare,
    CloseSquare,
    Comma,

    // Keywords
    If,
    Let,

    // Primitive
    Int32,

    // End of file
    Eof,
}

pub struct Lexer {
    chars: Vec<char>,
    cursor: usize,
    word_start: usize,
    size: usize,
}

impl Lexer {
    pub fn new(input_buff: &str) -> Self {
        Self {
            chars: input_buff.chars().collect(),
            cursor: 0 as usize,
            word_start: 0 as usize,
            size: input_buff.chars().count(),
        }
    }

    /// Skips all whitespace chars and positions the cursor at the begining of a word. Does not
    /// change word_start
    /// # Returns
    /// Err(()) if the cursor points to the end
    /// Ok(()) if the operation was succesfull
    fn skip_whitespace(&mut self) -> Result<(), ()> {
        if self.points_end() {
            return Err(());
        }
        loop {
            match self.chars[self.cursor] {
                ' ' | '\t' | '\n' => {
                    self.advance_cursor()?;
                }
                _ => {
                    break;
                }
            }
        }
        Ok(())
    }

    /// # Returns
    /// true if the cursor is at the end of the buffer
    /// false otherwise
    fn points_end(&self) -> bool {
        return self.cursor >= self.size;
    }

    /// Advances the cursor, with a bound check
    /// # Returns
    /// Ok(()) if the cursor was advanced
    /// Err(()) if the cursor is on the last char of the buffer
    #[inline(always)]
    fn advance_cursor(&mut self) -> Result<(), ()> {
        if self.cursor + 1 >= self.size {
            // End of buffer reached
            return Err(());
        }
        self.cursor += 1;
        return Ok(());
    }

    /// # Returns
    /// Ok(true) if the cursor points to a whitespace character (' ', '\t', '\n')
    /// Ok(false) if the cursor points to a non-whitespace char
    /// Err(()) if the cursor points to end of buffer
    #[inline(always)]
    fn points_whitespace(&mut self) -> Result<bool, ()> {
        if self.points_end() {
            return Err(());
        }
        match self.chars[self.cursor] {
            ' ' | '\t' | '\n' => {
                return Ok(true);
            }
            _ => {
                return Ok(false);
            }
        }
    }

    /// Positions the self.word_start and self.cursor at the begining and end of a lexema separated
    /// by whitespace both sides
    /// After the call, the lexema is found in the interval [word_start..cursor)
    /// # Returns
    /// Ok(()) if the operation succeded
    /// Err(()) if the end of buff was reached
    fn prepare_lexema(&mut self) -> Result<(), ()> {
        while self.points_whitespace()? {
            self.skip_whitespace();
        }
        self.word_start = self.cursor;
        while !self.points_whitespace()? {
            self.advance_cursor();
        }
        Ok(())
    }

    /// Returns the Integer literal parsed from [word_start..cursor)
    /// If word_start == cursor, returns a token formed from the character found at the cursor
    fn parse_lit_int(&mut self) -> Option<LexiToken> {
        if self.word_start == self.cursor {
            let res: i32 = self.chars[self.cursor] as i32;
            if res < 10 {
                return Some(LexiToken::LitInt(res));
            } else {
                return None;
            }
        }
        let slice: String = self.chars[self.word_start..self.cursor].iter().collect();

        match slice.parse::<i32>() {
            Ok(num) => {
                return Some(LexiToken::LitInt(num));
            }
            Err(_) => {
                return None;
            }
        }
    }

    /// Returns the Float literal parse from [word_start..cursor], with floating dot point in
    /// point_pos position, relative to the string to be parsed
    fn parse_lit_float(&mut self, point_pos: usize) -> Option<LexiToken> {
        if self.word_start == self.cursor {
            panic!("Floating point literal can't have only one letter");
        }
        let slice: String = self.chars[self.word_start..self.cursor].iter().collect();

        match slice.parse::<f64>() {
            Ok(num) => {
                return Some(LexiToken::LitFloat(num));
            }
            Err(_) => {
                return None;
            }
        }
    }

    /// Parses the numerical literal token found at the cursor
    /// The cursor is left at the start of the next unprocessed char
    /// # Returns
    /// Some(Eof) if the end of file was reached
    /// None if no char literal is found at the cursor
    /// Some(LitInt) if the literal could be interpreted as an integer literal (no floating point)
    /// Some(LitFloat) if the literal could be interpreted as a float integer (one . found inside
    /// the literal)
    fn parse_num(&mut self) -> Option<LexiToken> {
        self.word_start = self.cursor;
        let mut point_pos: i64 = -1; // -1 means no floating point
        loop {
            if self.cursor >= self.size {
                if self.cursor == self.word_start {
                    // No token was read
                    return Some(LexiToken::Eof); // End of file reached,
                }
                if point_pos == -1 {
                    return self.parse_lit_int();
                } else {
                    return self.parse_lit_float(point_pos as usize);
                }
            }

            if self.chars[self.cursor] == '.' {
                if point_pos != -1 {
                    // Second . found - Return None
                    return None;
                }
                point_pos = (self.cursor - self.word_start) as i64;
            } else if !self.chars[self.cursor].is_digit(10) {
                if self.cursor == self.word_start {
                    return None;
                }
                if point_pos == -1 {
                    return self.parse_lit_int();
                } else {
                    return self.parse_lit_float(point_pos as usize);
                }
            }
            self.cursor += 1;
        }
    }

    ///
    /// # Returns
    /// Err(()) if the token could not be parsed,
    fn parse_alpha(&mut self) -> Result<LexiToken, ()> {
        if self.points_whitespace()? {
            self.skip_whitespace();
        }
        self.prepare_lexema();
        let lexema: String = self.chars[self.word_start..self.cursor].iter().collect();
        match lexema.as_str() {
            // Keywords check
            "if" => {
                return Ok(LexiToken::If);
            }
            "let" => {
                return Ok(LexiToken::Let);
            }
            _ => {
                return Ok(LexiToken::LitString(lexema));
            }
        }
    }

    // TODO: MAKE PRIVATE, IS PUB FOR DEBUG
    pub fn next_token(&mut self) -> Result<LexiToken, ()> {
        if self.points_end() {
            return Ok(LexiToken::Eof);
        }
        self.skip_whitespace();
        match self.chars[self.word_start] {
            '+' => Ok(LexiToken::Plus),
            '-' => Ok(LexiToken::Minus),
            '*' => Ok(LexiToken::Star),
            '\\' => Ok(LexiToken::Slash),
            '%' => Ok(LexiToken::Percent),

            'a'..'z' | 'A'..'Z' | '_' => {
                return self.parse_alpha();
            }
            '0'..'9' => {
                if let Some(res) = self.parse_num() {
                    return Ok(res);
                } else {
                    return Err(());
                }
            }

            _ => Err(()),
        }
    }
}
