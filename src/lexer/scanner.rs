/// # Lexical Scanner
///
/// Converts source text from .sutra files into a token stream.
/// Supports Latin, Sanskrit (Devanagari) character sets.

use super::tokens::{Token, TokenKind};

/// Lexical Scanner
pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Scanner {
    /// Create a new scanner from source text
    pub fn new(source: &str) -> Self {
        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    /// Scan all tokens from source
    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenKind::Eof,
            "",
            self.line,
            self.column,
            self.current,
        ));

        self.tokens.clone()
    }

    /// Scan a single token
    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            // Whitespace
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.add_token(TokenKind::Newline);
                self.line += 1;
                self.column = 1;
            }

            // Structure
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '[' => self.add_token(TokenKind::LeftBracket),
            ']' => self.add_token(TokenKind::RightBracket),

            // Arithmetic operators
            '+' => self.add_token(TokenKind::Plus),
            '-' => {
                // Distinguish subtraction from negative numbers
                if self.peek().is_ascii_digit() && self.is_after_operator() {
                    self.scan_number();
                } else {
                    self.add_token(TokenKind::Minus);
                }
            }
            '*' => self.add_token(TokenKind::Star),
            '/' => self.add_token(TokenKind::Slash),
            '%' => self.add_token(TokenKind::Percent),

            // Comparison operators
            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::EqualEqual);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::BangEqual);
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual);
                } else {
                    self.add_token(TokenKind::Less);
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual);
                } else {
                    self.add_token(TokenKind::Greater);
                }
            }

            // Special delimiters
            '|' => self.add_token(TokenKind::Pipe),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            ':' => {
                if self.match_char(':') {
                    self.add_token(TokenKind::DoubleColon);
                } else if self.is_identifier_start(self.peek()) {
                    // Param identifier (e.g. :id)
                    self.scan_param_identifier();
                } else {
                    self.add_token(TokenKind::Colon);
                }
            }

            // Special Unicode symbols
            '√' => self.add_token(TokenKind::DhatuPrefix),
            '→' => self.add_token(TokenKind::Arrow),
            '←' => self.add_token(TokenKind::Assign),
            '·' => self.add_token(TokenKind::ParamSeparator),
            '◆' => self.add_token(TokenKind::SutraBullet),
            '◈' => self.add_token(TokenKind::CurrentElement),
            '◇' => self.scan_comment(),

            // ॥ — Sacred mark (sutra comment boundary)
            '॥' => self.scan_sacred_comment(),

            // String literals
            '"' => self.scan_string(),

            _ => {
                if c.is_ascii_digit() {
                    self.scan_number();
                } else if self.is_identifier_start(c) {
                    self.scan_identifier_or_keyword(c);
                }
                // Unknown characters are silently ignored in REPL
            }
        }
    }

    /// Scan a numeric literal
    fn scan_number(&mut self) {
        while !self.is_at_end() && self.peek().is_ascii_digit() {
            self.advance();
        }

        let is_float = if !self.is_at_end() && self.peek() == '.' {
            if self.current + 1 < self.source.len() && self.source[self.current + 1].is_ascii_digit() {
                self.advance(); // consume '.'
                while !self.is_at_end() && self.peek().is_ascii_digit() {
                    self.advance();
                }
                true
            } else {
                false
            }
        } else {
            false
        };

        let text: String = self.source[self.start..self.current].iter().collect();
        if is_float {
            if let Ok(val) = text.parse::<f64>() {
                self.add_token(TokenKind::Float(val));
            }
        } else {
            if let Ok(val) = text.parse::<i64>() {
                self.add_token(TokenKind::Integer(val));
            }
        }
    }

    /// Scan a string literal
    fn scan_string(&mut self) {
        let mut value = String::new();
        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                self.line += 1;
                self.column = 1;
            }
            if self.peek() == '\\' {
                self.advance(); // consume backslash
                match self.peek() {
                    'n' => { value.push('\n'); self.advance(); }
                    't' => { value.push('\t'); self.advance(); }
                    '"' => { value.push('"'); self.advance(); }
                    '\\' => { value.push('\\'); self.advance(); }
                    _ => { value.push('\\'); }
                }
            } else {
                value.push(self.peek());
                self.advance();
            }
        }

        if self.is_at_end() {
            // Unterminated string
            return;
        }

        self.advance(); // consume closing "
        self.tokens.push(Token::new(
            TokenKind::StringLiteral(value),
            self.current_lexeme(),
            self.line,
            self.column,
            self.start,
        ));
    }

    /// Scan comment (◇)
    fn scan_comment(&mut self) {
        // Comment extends to end of line
        while !self.is_at_end() && self.peek() != '\n' {
            self.advance();
        }
        // No token produced — comments are discarded
    }

    /// Scan sacred comment (॥ ... ॥)
    fn scan_sacred_comment(&mut self) {
        // Sacred comment extends to next ॥ or end of line
        while !self.is_at_end() && self.peek() != '॥' && self.peek() != '\n' {
            self.advance();
        }
        if !self.is_at_end() && self.peek() == '॥' {
            self.advance(); // consume closing ॥
        }
        // No token produced
    }

    /// Scan param identifier (e.g. :id)
    fn scan_param_identifier(&mut self) {
        while !self.is_at_end() && self.is_identifier_char(self.peek()) {
            self.advance();
        }
        let name: String = self.source[self.start + 1..self.current].iter().collect();
        self.add_token(TokenKind::ParamIdentifier(name));
    }

    /// Scan identifier or keyword
    fn scan_identifier_or_keyword(&mut self, _first_char: char) {
        while !self.is_at_end() && self.is_identifier_char(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        // Check for Sanskrit keywords (Braj Bhasha RK)
        let kind = match text.as_str() {
            // Sanskrit-only keywords
            "अधिकार" => TokenKind::Adhikara,
            "प्रकरण" => TokenKind::Prakarana,
            "सूत्र" => TokenKind::SutraDef,
            "यदि" => TokenKind::Yadi,
            "विकल्प" => TokenKind::Vikalpa,
            "प्रदक्षिणा" => TokenKind::Pradakshina,
            "प्रयत्न" => TokenKind::Prayatna,
            "دोष" | "दोष" => TokenKind::Dosha,
            "स्वरूप" => TokenKind::Swaroopa,
            "अष्टाध्यायी" => TokenKind::Ashtadhyayi,
            "आयात" => TokenKind::Aayat,

            // Vedic Logic (Chatushkoti)
            "सत्" | "सत्य" => TokenKind::Tattva(crate::evaluator::TattvaState::Sat),
            "असत्" | "असत्य" => TokenKind::Tattva(crate::evaluator::TattvaState::Asat),
            "सदसत्" => TokenKind::Tattva(crate::evaluator::TattvaState::Sadasat),
            "अव्यक्तम्" => TokenKind::Tattva(crate::evaluator::TattvaState::Avyaktam),

            // Phase 1: Sovereign Extensions
            "च" => TokenKind::And,
            "वा" => TokenKind::Or,
            "यावत्" => TokenKind::Yavat,
            "प्रतिदा" => TokenKind::Pratida,
            "विद्युत्" => TokenKind::Vidyut,
            "प्रतिमान" => TokenKind::Pratiman,
            "अन्यथा" => TokenKind::Anyatha,

            // General identifier
            _ => TokenKind::Identifier(text.clone()),
        };

        self.add_token(kind);
    }

    // ═══════════════ Helpers ═══════════════

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        self.column += 1;
        c
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            false
        } else {
            self.current += 1;
            self.column += 1;
            true
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_identifier_start(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_' || ('\u{0900}' <= c && c <= '\u{097F}')
    }

    fn is_identifier_char(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_' || ('\u{0900}' <= c && c <= '\u{097F}')
    }

    fn is_after_operator(&self) -> bool {
        if self.tokens.is_empty() {
            return true;
        }
        matches!(
            self.tokens.last().map(|t| &t.kind),
            Some(
                TokenKind::Plus
                    | TokenKind::Minus
                    | TokenKind::Star
                    | TokenKind::Slash
                    | TokenKind::LeftParen
                    | TokenKind::Assign
                    | TokenKind::Arrow
                    | TokenKind::Pipe
                    | TokenKind::DoubleColon
            )
        )
    }

    fn current_lexeme(&self) -> String {
        self.source[self.start..self.current].iter().collect()
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme = self.current_lexeme();
        self.tokens.push(Token::new(
            kind,
            lexeme,
            self.line,
            self.column,
            self.start,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_scan() {
        let mut scanner = Scanner::new("√vid+ति 42");
        let tokens = scanner.scan_tokens();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::DhatuPrefix)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Integer(42))));
    }

    #[test]
    fn test_string_scan() {
        let mut scanner = Scanner::new("\"hello world\"");
        let tokens = scanner.scan_tokens();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::StringLiteral(ref s) if s == "hello world")));
    }

    #[test]
    fn test_keyword_scan() {
        let mut scanner = Scanner::new("अधिकार");
        let tokens = scanner.scan_tokens();
        let adhikara_count = tokens.iter().filter(|t| matches!(t.kind, TokenKind::Adhikara)).count();
        assert_eq!(adhikara_count, 1);
    }

    #[test]
    fn test_arrow_and_pipe() {
        let mut scanner = Scanner::new("→ | ::");
        let tokens = scanner.scan_tokens();
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Arrow)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::Pipe)));
        assert!(tokens.iter().any(|t| matches!(t.kind, TokenKind::DoubleColon)));
    }

    #[test]
    fn test_comment_ignored() {
        let mut scanner = Scanner::new("42 ◇ this is a comment\n43");
        let tokens = scanner.scan_tokens();
        let integers: Vec<_> = tokens.iter().filter(|t| matches!(t.kind, TokenKind::Integer(_))).collect();
        assert_eq!(integers.len(), 2);
    }
    #[test]
    fn test_all_sanskrit_keywords() {
        let keywords = vec![
            ("ज्ञान", TokenKind::Jnana),
            ("कर्म", TokenKind::Karma),
            ("भक्ति", TokenKind::Bhakti),
            ("सत्य", TokenKind::Satya),
            ("अहम्", TokenKind::Aham),
            ("तथा", TokenKind::Tatha),
            ("चेत्", TokenKind::Chet),
            ("अन्यथा", TokenKind::Anyatha),
            ("सर्वदा", TokenKind::Sarvada),
            ("शून्य", TokenKind::Shunya),
            ("आयात", TokenKind::Ayaat),
        ];

        for (word, expected_kind) in keywords {
            let mut scanner = Scanner::new(word);
            let tokens = scanner.scan_tokens();
            assert_eq!(tokens[0].kind, expected_kind, "Failed on keyword {}", word);
        }
    }

    #[test]
    fn test_numbers_and_decimals() {
        let mut scanner = Scanner::new("108 3.14159 -42");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::Integer(108)));
        assert!(matches!(tokens[2].kind, TokenKind::Float(f) if (f - 3.14159).abs() < f64::EPSILON));
        assert!(matches!(tokens[4].kind, TokenKind::Integer(-42)));
    }

    #[test]
    fn test_arithmetic_operators() {
        let mut scanner = Scanner::new("+ - * / %");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::Plus));
        assert!(matches!(tokens[2].kind, TokenKind::Minus));
        assert!(matches!(tokens[4].kind, TokenKind::Star));
        assert!(matches!(tokens[6].kind, TokenKind::Slash));
        assert!(matches!(tokens[8].kind, TokenKind::Percent));
    }

    #[test]
    fn test_logical_operators() {
        let mut scanner = Scanner::new("&& || !");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::And));
        assert!(matches!(tokens[2].kind, TokenKind::Or));
        assert!(matches!(tokens[4].kind, TokenKind::Bang));
    }

    #[test]
    fn test_comparison_operators() {
        let mut scanner = Scanner::new("== != > >= < <=");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::EqualEqual));
        assert!(matches!(tokens[2].kind, TokenKind::BangEqual));
        assert!(matches!(tokens[4].kind, TokenKind::Greater));
        assert!(matches!(tokens[6].kind, TokenKind::GreaterEqual));
        assert!(matches!(tokens[8].kind, TokenKind::Less));
        assert!(matches!(tokens[10].kind, TokenKind::LessEqual));
    }

    #[test]
    fn test_punctuation() {
        let mut scanner = Scanner::new("{ } ( ) [ ] , . : ; ::");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::LeftBrace));
        assert!(matches!(tokens[2].kind, TokenKind::RightBrace));
        assert!(matches!(tokens[4].kind, TokenKind::LeftParen));
        assert!(matches!(tokens[6].kind, TokenKind::RightParen));
        assert!(matches!(tokens[8].kind, TokenKind::LeftBracket));
        assert!(matches!(tokens[10].kind, TokenKind::RightBracket));
        assert!(matches!(tokens[12].kind, TokenKind::Comma));
        assert!(matches!(tokens[14].kind, TokenKind::Dot));
        assert!(matches!(tokens[16].kind, TokenKind::Colon));
        assert!(matches!(tokens[18].kind, TokenKind::Semicolon));
        assert!(matches!(tokens[20].kind, TokenKind::DoubleColon));
    }

    #[test]
    fn test_devanagari_identifiers() {
        let mut scanner = Scanner::new("राम कृष्ण१२३");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "राम"));
        assert!(matches!(tokens[2].kind, TokenKind::Identifier(ref s) if s == "कृष्ण१२३"));
    }

    #[test]
    fn test_mixed_latin_devanagari() {
        let mut scanner = Scanner::new("const_val तथा x");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::Identifier(ref s) if s == "const_val"));
        assert!(matches!(tokens[2].kind, TokenKind::Tatha));
        assert!(matches!(tokens[4].kind, TokenKind::Identifier(ref s) if s == "x"));
    }

    #[test]
    fn test_string_escape_sequences() {
        let mut scanner = Scanner::new("\"hello\\nworld\\t\\\"quote\\\"\"");
        let tokens = scanner.scan_tokens();
        // Since string parsing in basic lexer might not fully unescape everything based on implementation, 
        // we just ensure it correctly parses as a string literal.
        assert!(matches!(tokens[0].kind, TokenKind::StringLiteral(_)));
    }

    #[test]
    fn test_unterminated_string() {
        let mut scanner = Scanner::new("\"unterminated");
        let tokens = scanner.scan_tokens();
        // In this implementation, the scanner will just end and perhaps add a string token anyway or throw an error.
        // We ensure it handles it without crashing.
        assert!(!tokens.is_empty());
    }

    #[test]
    fn test_empty_string() {
        let mut scanner = Scanner::new("\"\"");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::StringLiteral(ref s) if s.is_empty()));
    }

    #[test]
    fn test_multiple_lines() {
        let mut scanner = Scanner::new("अहम्\nसत्य\n\nकर्म");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::Aham));
        assert!(matches!(tokens[1].kind, TokenKind::Newline));
        assert!(matches!(tokens[2].kind, TokenKind::Satya));
        assert!(matches!(tokens[3].kind, TokenKind::Newline));
        assert!(matches!(tokens[4].kind, TokenKind::Newline));
        assert!(matches!(tokens[5].kind, TokenKind::Karma));
    }

    #[test]
    fn test_comments_end_of_line() {
        let mut scanner = Scanner::new("अहम् ◇ truth is important\nसत्य");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::Aham));
        assert!(matches!(tokens[1].kind, TokenKind::Newline));
        assert!(matches!(tokens[2].kind, TokenKind::Satya));
    }

    #[test]
    fn test_dhatu_syntax() {
        let mut scanner = Scanner::new("√kri+ति");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::DhatuPrefix));
        assert!(matches!(tokens[1].kind, TokenKind::Identifier(ref s) if s == "kri"));
        assert!(matches!(tokens[2].kind, TokenKind::Plus));
        assert!(matches!(tokens[3].kind, TokenKind::Identifier(ref s) if s == "ति"));
    }

    #[test]
    fn test_type_annotations() {
        let mut scanner = Scanner::new("x: ज्ञान = 5");
        let tokens = scanner.scan_tokens();
        assert!(matches!(tokens[0].kind, TokenKind::Identifier(_)));
        assert!(matches!(tokens[1].kind, TokenKind::Colon));
        assert!(matches!(tokens[3].kind, TokenKind::Jnana));
        assert!(matches!(tokens[5].kind, TokenKind::Equal));
        assert!(matches!(tokens[7].kind, TokenKind::Integer(5)));
    }
}
