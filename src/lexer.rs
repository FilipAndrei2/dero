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

    // Primitive
    Int32,

    // End of file
    Eof
}


pub struct Lexer{
    chars: Vec<char>,
    cursor: usize,
    word_start: usize,
    size: usize,
}

impl Lexer  {
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
    fn skip_whitespace(&mut self) {
        loop {
            match self.chars[self.cursor] {
                ' ' | '\t' | '\n' => { self.cursor += 1;} ,
                _ => {
                    break;
                }
            }
        }
    }

    /// Returns the Integer literal parsed from [word_start..cursor)
    /// If word_start == cursor, returns a token formed from the character found at the cursor
    fn parse_lit_int(&mut self) -> Option<LexiToken> {
        todo!("Not implemented yet");
    }

    /// Returns the Float literal parsed from [word_start..cursor], with floating dot point in
    /// point_pos position, relative to the string to be parsed
    fn parse_lit_float(&mut self, point_pos: usize) -> Option<LexiToken> {
        todo!("Not implemented yet");
    }

    /// Parses the numerical literal token found at the cursor 
    /// The cursor is left at the start of the next unprocessed char
    /// # Returns
    /// Some(Eof) if the end of file was reached
    /// None if no char literal is found at the cursor
    /// Some(LitInt) if the literal could be interpreted as an integer literal (no floating point)
    /// Some(LitFloat) if the literal could be interpreted as a float integer (one . found inside
    /// the literal)
    fn parse_num_lit(&mut self) -> Option<LexiToken>{
        self.word_start = self.cursor;
        let mut point_pos: i64 = -1; // -1 means no floating point
        loop {
            if self.cursor + 1 >= self.size {
                if self.cursor == self.word_start { // No token was read
                    return Some(LexiToken::Eof); // End of file reached,
                }
                if point_pos == -1 { return self.parse_lit_int(); } else { return self.parse_lit_float(point_pos as usize); }
            }

            if self.chars[self.cursor] == '.' {
                if point_pos != -1 {
                    // Second . found - Return None
                    return None;
                }
               point_pos = (self.cursor - self.word_start) as i64;
            } else if !self.chars[self.cursor].is_digit(10) {
                if self.cursor == self.word_start { return None; }
                if point_pos == -1 { return self.parse_lit_int(); } else { return self.parse_lit_float(point_pos as usize); }
            }
            self.cursor += 1;
        }
    }

    fn parse_alphastr_lit(&mut self) -> Option<LexiToken> {
        todo!();
    }

    // TODO: MAKE PRIVATE, IS PUB FOR DEBUG
    pub fn next_token(&mut self) -> Option<LexiToken> {
        self.skip_whitespace();
        match self.chars[self.word_start] {
            '+' => Some(LexiToken::Plus),
            '-' => Some(LexiToken::Minus),
            '*' => Some(LexiToken::Star),
            '\\' => Some(LexiToken::Slash),
            '%' => Some(LexiToken::Percent),

            'a'..'z' | 'A'..'Z' | '_' => self.parse_alphastr_lit(),
            '0'..'9' => self.parse_num_lit(),

            _ => None
        }
    }
}
