use crate::token::*;

#[derive(Debug)]
pub struct Lexer {
    source: String,
    current_char: char,
    next_char: char,
    position: isize,
}

impl Lexer {
    //as received from my C# implementation
    pub fn new(source: String) -> Self {
        let mut lexer = Lexer {
            source,
            current_char: '\0',
            next_char: '\0',
            position: -2,
        };
        lexer.next_token();
        lexer
    }

    ///eat whitespaces
    /// I eat whitespaces for breakfast
    pub fn consume_white_space(&mut self) {
        while self.current_char != '\0' && self.current_char.is_whitespace() {
            self.next_token();
        }
    }
    ///some weird people put comments in JSON config files like //this is comment
    /// there should be nothing like comment in a JSON
    fn skip_comments(&mut self) {
        if self.current_char == '/' || self.current_char == '#'{
            while self.current_char != '\0' && self.current_char != '\n'{
                self.next_token();
            }
        }
    }
    //advance - forward ever, backward never LOL!
    pub fn next_token(&mut self) {
        self.position += 1;
        self.current_char = self.next_char;
        if self.position <= (self.source.len() as isize - 2) {
            self.next_char = self
                .source
                .chars()
                .nth((self.position + 1) as usize)
                .unwrap_or('\0');
        } else {
            self.next_char = '\0';
        }
    }

    //the powerHouse... Let there be breakage, let there by CaRnAgE! by the order of the
    // Peaky funky Blinders
    pub fn lex(&mut self) -> Token {
        self.next_token();
        self.consume_white_space();
        let current_char = self.current_char;
        let char_string = current_char.to_string();

        match current_char {
            '{' => Token::new(TokenKind::LCURLY, char_string),
            '}' => Token::new(TokenKind::RCURLY, char_string),
            ':' => Token::new(TokenKind::COLON, char_string),
            '[' => Token::new(TokenKind::LSQUARE, char_string),
            ']' => Token::new(TokenKind::RSQUARE, char_string),
            ',' => Token::new(TokenKind::COMMA, char_string),
            '.' => Token::new(TokenKind::PERIOD, char_string),
            '/' | '#' => {
                self.skip_comments();
                Token::new(TokenKind::EOF, String::new())
            },
            '"' | '\'' => self.make_string_literal(),
            't' | 'f' => self.make_boolean_literal(),
            //_ if current_char.is_alphabetic() => self.make_identifier_literal(),
            _ if current_char.is_numeric() => self.make_numeric_literal(),
            _ => Token::new(TokenKind::EOF, "\0".to_string()),
        }
    }

    //make a wish, pick a number - magic
    //lex numbers
    fn make_numeric_literal(&mut self) -> Token {
        let current_pos = self.position;
        let mut dots = 0;
        while self.current_char != '\0' && (self.next_char.is_digit(10) || self.next_char == '.') {
            if self.current_char == '.' {
                dots += 1;
            }
            self.next_token();
        }
        if dots > 1 {
            panic!("Invalid decimal number format");
        }
        let number_str = &self.source[current_pos as usize..self.position as usize + 1];
        if dots == 0 {
            Token::new(TokenKind::NLITERAL, number_str.to_string())
        } else {
            Token::new(TokenKind::NLITERAL, number_str.to_string())
        }
    }

    //make a boolean, flip the switch (1/0)
    //lex boolean literals - TF
    fn make_boolean_literal(&mut self) -> Token {
        let current_pos = self.position;
        while self.current_char != '\0' && self.current_char != 'e' {
            self.next_token();
        }
        let bool_str = &self.source[current_pos as usize..self.position as usize + 1];
        Token::new(TokenKind::BLITERAL, bool_str.to_lowercase())
    }

    //strings! heck yeah!
    fn make_string_literal(&mut self) -> Token {
        let stop_char = self.current_char;
        self.next_token();
        let current_pos = self.position;
        while self.current_char != '\0' && self.current_char != stop_char {
            self.next_token();
        }
        let literal = &self.source[current_pos as usize..self.position as usize];
        Token::new(TokenKind::SLITERAL, literal.to_string())
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = self.lex();
        if tok.kind == TokenKind::EOF {
            return None;
        }
        Some(tok)
    }
}
