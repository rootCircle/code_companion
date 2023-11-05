use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, PartialEq)]
enum TokenType {
    // Metacharacters
    LeftParens,
    RightParens,
    ColonQuestion,
    Backslash,
    LeftSquareBracket,
    RightSquareBracket,
    LeftCurlyBrackets,
    RightCurlyBrackets,
    Comma,

    // Character sets
    Integer, // (Value, Min, Max)
    Float, // (Value, Min, Max)
    String,
    Character,
    // Space,

    // Literals
    LiteralNumber(u64),

    // End of file
    Eof
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

pub(crate) struct Tokens {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    source_language: String,
}

impl Tokens {
    fn new(source_language: String) -> Self {
        Self {
            tokens: Vec::new(),
            start: 0,
            current: 0,
            source_language
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string()
        });
    }

    fn is_at_end(&self) -> bool {
        self.source_language.len() <= self.current
    }

    fn scan_token(&mut self) -> Result<String, String>{
        let c = self.advance();
        match c {
            "(" => self.add_token(TokenType::LeftParens),
            ")" => self.add_token(TokenType::RightParens),
            "[" => self.add_token(TokenType::LeftSquareBracket),
            "]" => self.add_token(TokenType::RightSquareBracket),
            "{" => self.add_token(TokenType::LeftCurlyBrackets),
            "}" => self.add_token(TokenType::RightCurlyBrackets),
            "," => self.add_token(TokenType::Comma),
            "\\" => self.add_token(TokenType::Backslash),
            "N" => self.add_token(TokenType::Integer),
            "F" => self.add_token(TokenType::Float),
            "S" => self.add_token(TokenType::String),
            "C" => self.add_token(TokenType::Character),
            " " | "\r" | "\t" | "\n" => {
                // Do nothing, just those spaces out :evil:
            },
            ":" => {
                if self.match_str("?") {
                    self.add_token(TokenType::ColonQuestion);
                }
                else {
                    return Err("Expected ? after :".to_string())
                }
            },
            _ => {
                if Tokens::is_digit(c) {
                    while Tokens::is_digit(self.peek()) {
                        self.current += 1;
                    }

                    let number = match self.source_language[self.start..self.current].parse::<u64>(){
                        Ok(num) => num,
                        Err(err) => {
                            return  Err("Error parsing the number".to_string());
                        }
                    };

                    self.add_token(TokenType::LiteralNumber(number));
                }
                else {
                    return Err(format!("Unexpected character {c}"))
                }
            }
        }
        Ok("".to_string())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source_language[self.start..self.current].to_string(),
        })
    }

    fn advance(&mut self) -> &str {
        self.current += 1;
        self.char_at(self.current - 1)
    }

    fn char_at(&self, index: usize) -> &str {
        self.source_language.graphemes(true).collect::<Vec<&str>>()[index]
    }

    fn match_str(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.char_at(self.current) != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn is_digit(ch: &str) -> bool {
        ("0"..="9").contains(&ch)
    }

    fn peek(&self) -> &str {
        if self.is_at_end() {
            return "\0";
        }

        self.char_at(self.current)
    }

}


#[cfg(test)]
mod tests {
    use crate::test_language::lexer::{Tokens, Token, TokenType};

    #[test]
    fn tokenization_works() {
        let mut src = "12N3";
        let mut tokens = Tokens::new(src.to_string());

        tokens.scan_tokens();

        assert_eq!(tokens.tokens, vec![
            Token { token_type: TokenType::LiteralNumber(12), lexeme: "12".to_string() },
            Token { token_type: TokenType::Integer, lexeme: "N".to_string() },
            Token { token_type: TokenType::LiteralNumber(3), lexeme: "3".to_string() },
            Token { token_type: TokenType::Eof, lexeme: "".to_string()}
        ]);
    }
}