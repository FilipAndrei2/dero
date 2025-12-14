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
    /// Ok(true) if eof was reached
    /// Ok(false) if eof wan not reached
    /// Err(()) if an error occured.
    fn skip_whitespace(&mut self) -> Result<bool, ()> {
        loop {
            if self.points_end() {
                return Ok(true);
            }
            if !self.points_whitespace()? {
                break;
            }
            self.advance_cursor();
        }
        return Ok(false);
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
    /// Err(()) if trying to go past the last character in buffer (past 'EOF')
    #[inline(always)]
    fn advance_cursor(&mut self) -> Result<(), ()> {
        if self.cursor == self.size {
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
    fn points_whitespace(&self) -> Result<bool, ()> {
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

    /// # Returns
    /// Ok(true) if the cursor points to a digit char ('0'..'9')
    /// Of(false) if the cursor does not point to a digit char
    /// Err(()) if the cursor points to end of buffer
    #[inline(always)]
    fn points_digit(&self) -> Result<bool, ()> {
        if self.points_end() {
            return Err(());
        }
        match self.chars[self.cursor] {
            '0'..'9' => {
                return Ok(true);
            }
            _ => {
                return Ok(false);
            }
        }
    }

    /// # Returns
    /// Ok(true) if the cursor points to the specified char
    /// Of(false) if the cursor does not point to the specified char
    /// Err(()) if the cursor points to end of buffer
    #[inline(always)]
    fn points_char(&self, ch: char) -> Result<bool, ()> {
        if self.points_end() {
            return Err(());
        }
        return Ok(self.chars[self.cursor] == ch);
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
            self.advance_cursor()?;
        }
        Ok(())
    }

    fn parse_lit_int(&self) -> Result<LexiToken, ()> {
        let lexema_str: String = self.chars[self.word_start..self.cursor].iter().collect();
        match lexema_str.as_str().parse::<i32>() {
            Ok(tkn_val) => return Ok(LexiToken::LitInt(tkn_val)),
            Err(_) => {
                return Err(());
            }
        }
    }

    fn parse_lit_float(&self) -> Result<LexiToken, ()> {
        let lexema_str: String = self.chars[self.word_start..self.cursor].iter().collect();
        match lexema_str.as_str().parse::<f64>() {
            Ok(tkn_val) => return Ok(LexiToken::LitFloat(tkn_val)),
            Err(_) => {
                return Err(());
            }
        }
    }

    /// Parses the numerical literal token found at the cursor
    /// The cursor is left at the start of the next unprocessed char
    /// # Returns
    /// Ok(Eof) if the end of file was reached
    /// Ok(LitInt) if the literal could be interpreted as an integer literal (no floating point)
    /// Ok(LitFloat) if the literal could be interpreted as a float integer (one . found inside
    /// the literal)
    /// Err(()) if an error happened during the parsing, or if the present lexema could not be
    /// interpreted as a numerical value
    fn parse_num(&mut self) -> Result<LexiToken, ()> {
        if self.points_end() {
            return Err(());
        }

        self.word_start = self.cursor;
        let mut fp_pos: isize = if self.points_char('.')? {
            0 // 0th char in lexem is '.'
        } else {
            -1
        }; // -1 => no floating point found

        loop {
            if self.points_char('.')? {
                if fp_pos != -1 {
                    // Second floating point found, raise error
                    return Err(());
                } else {
                    fp_pos = (self.cursor - self.word_start) as isize;
                }
            } else if !self.points_digit()? {
                break;
            }
            self.advance_cursor()?;
        }
        if fp_pos == -1 {
            // We must parse an int
            return self.parse_lit_int();
        } else {
            // We must parse a float
            return self.parse_lit_float();
        }
    }

    ///
    /// # Returns
    /// Err(()) if the token could not be parsed,
    fn parse_alpha(&mut self) -> Result<LexiToken, ()> {
        if self.points_whitespace()? {
            if self.skip_whitespace()? {
                return Err(());
            }
        }
        self.prepare_lexema()?;
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
        if self.skip_whitespace()? {
            return Ok(LexiToken::Eof);
        }
        match self.chars[self.cursor] {
            '+' => {
                self.advance_cursor()?;
                return Ok(LexiToken::Plus);
            }
            '-' => Ok(LexiToken::Minus),
            '*' => Ok(LexiToken::Star),
            '\\' => Ok(LexiToken::Slash),
            '%' => Ok(LexiToken::Percent),
            '=' => {
                self.advance_cursor()?;
                return Ok(LexiToken::Eq);
            }

            'a'..'z' | 'A'..'Z' | '_' => {
                return self.parse_alpha();
            }

            '0'..'9' => {
                return self.parse_num();
            }

            _ => Err(()),
        }
    }
}
