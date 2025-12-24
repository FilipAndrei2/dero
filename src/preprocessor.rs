#![allow(dead_code)]

pub struct Preprocessor {
    cursor: usize,
    ws_cursor: usize,
    text: Vec<char>,
    size: usize,
}

pub enum PreprocErr {
    Cursor,
}

pub enum PreprocInfo {
    CursorEof,
    CursorOk,
    PreprocOk,
}

impl Preprocessor {
    pub fn new(code: &str) -> Preprocessor {
        let text: Vec<char> = code.chars().collect();
        return Preprocessor {
            cursor: 0 as usize,
            ws_cursor: 0 as usize,
            size: text.len(),
            text,
        };
    }

    /// # Returns
    /// false if cursor is not pointing to eof
    /// true if cursor points to eof
    fn points_eof(&self) -> bool {
        return self.cursor == self.size;
    }

    /// # Returns
    /// Ok(char) the character pointed by the cursor
    /// Err(PreprocErr::Cursor) if the cursor points to eof
    fn peek(&self) -> Result<char, PreprocErr> {
        if self.points_eof() {
            return Err(PreprocErr::Cursor);
        }
        return Ok(self.text[self.cursor]);
    }

    /// # Returns
    /// Err(PreprocErr::Cursor) if the cursor could not be advanced
    /// Ok(PreprocInfo::CursorOk) if the cursor was moved
    /// Ok(PreprocInfo::CursorEof) if the cursor was moved, and now is pointing to eof
    fn advance_cursor(&mut self) -> Result<PreprocInfo, PreprocErr> {
        if self.cursor >= self.size {
            return Err(PreprocErr::Cursor);
        }
        self.cursor += 1;
        if self.points_eof() {
            return Ok(PreprocInfo::CursorEof);
        }
        return Ok(PreprocInfo::CursorOk);
    }

    /// Reads a char, and increments the cursor
    /// # Returns
    /// Ok(ch: char, stream_end: bool)
    /// ch -> read char
    /// stream_end true if stream end was reached
    /// Err(PreprocErr) if an error was found
    fn next_char(&mut self) -> Result<(char, bool), PreprocErr> {
        let ch = self.peek()?;
        let mut stream_end = false;
        if let PreprocInfo::CursorEof = self.advance_cursor()? {
            stream_end = true;
        }
        return Ok((ch, stream_end));
    }

    fn main_loop(&mut self, preproc_code_buff: &mut Vec<char>) -> Result<PreprocInfo, PreprocErr> {
        while let Ok((ch, stream_end)) = self.next_char()
            && !stream_end
        {
            preproc_code_buff.push(ch);
        }
        return Ok(PreprocInfo::PreprocOk);
    }

    pub fn preprocess(&mut self) -> Result<Vec<char>, PreprocErr> {
        let mut preprocessed_code: Vec<char> = Vec::with_capacity(self.size);
        self.main_loop(&mut preprocessed_code)?;

        return Ok(preprocessed_code);
    }
}
