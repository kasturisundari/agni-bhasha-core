/// # Token Types
///
/// Defines every token that can appear in a .sutra source file.

/// Source location of a token
#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
    pub length: usize,
}

impl Span {
    pub fn line_col(&self) -> (usize, usize) {
        (self.line, self.column)
    }
}

/// Single lexical token
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

/// Token kinds
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // ═══════════════ Roots & Suffixes ═══════════════
    /// √ — Dhatu root prefix (e.g. √sṛj)
    DhatuPrefix,
    /// Root name (e.g. sṛj, dhā, vid)
    DhatuName(String),
    /// + — Pratyaya join connector
    PratyayaJoin,
    /// Suffix name (e.g. ti, syati, tavya)
    PratyayaName(String),

    // ═══════════════ Delimiters ═══════════════
    /// · — Parameter separator (e.g. √bandh+त्र·8080)
    ParamSeparator,
    /// → — Transform flow arrow
    Arrow,
    /// | — Condition separator
    Pipe,
    /// :: — Result specifier
    DoubleColon,
    /// : — Colon (for dicts)
    Colon,
    /// . — Dot (property access)
    Dot,
    /// , — Comma
    Comma,
    /// ← — Assignment arrow
    Assign,
    /// ◆ — Sutra bullet in block
    SutraBullet,
    /// ◈ — Current iteration element
    CurrentElement,
    /// ◇ — Comment start
    Comment,

    // ═══════════════ Structure ═══════════════
    /// { — Open block
    LeftBrace,
    /// } — Close block
    RightBrace,
    /// ( — Open paren
    LeftParen,
    /// ) — Close paren
    RightParen,
    /// [ — Open bracket
    LeftBracket,
    /// ] — Close bracket
    RightBracket,

    // ═══════════════ Keywords ═══════════════
    /// अधिकार — Scope declaration (adhikara)
    Adhikara,
    /// प्रकरण — Sub-scope (prakarana)
    Prakarana,
    /// सूत्र — Named sutra / function (sutra)
    SutraDef,
    /// यदि — Conditional (If)
    Yadi,
    /// विकल्प — Alternative (Else)
    Vikalpa,
    /// प्रदक्षिणा — Iteration (ForEach)
    Pradakshina,
    /// प्रयत्न — Attempt (Try)
    Prayatna,
    /// दोष — Fault (Catch)
    Dosha,
    /// स्वरूप — Data template (Struct)
    Swaroopa,
    /// अष्टाध्यायी — Term Rewriting Block
    Ashtadhyayi,
    /// ॥ — Sacred sutra comment boundary
    SacredMark,

    // ═══════════════ Phase 1: Sovereign Extensions ═══════════════
    /// आयात — Import module
    Aayat,
    /// च — Logical AND (Cha)
    And,
    /// वा — Logical OR (Va)
    Or,
    /// यावत् — While loop (Yavat)
    Yavat,
    /// प्रतिदा — Return statement (Pratida)
    Pratida,
    /// विद्युत् — Lambda / Anonymous function (Vidyut)
    Vidyut,
    /// प्रतिमान — Pattern match (Pratiman)
    Pratiman,
    /// अन्यथा — Default match arm (Anyatha)
    Anyatha,

    // ═══════════════ Literals ═══════════════
    /// Integer literal
    Integer(i64),
    /// Float literal
    Float(f64),
    /// String literal
    StringLiteral(String),
    /// Vedic multi-valued logic
    Tattva(crate::evaluator::TattvaState),

    // ═══════════════ Operators ═══════════════
    /// + — Addition
    Plus,
    /// - — Subtraction
    Minus,
    /// * — Multiplication
    Star,
    /// / — Division
    Slash,
    /// % — Modulo
    Percent,
    /// == — Equality
    EqualEqual,
    /// != — Inequality
    BangEqual,
    /// < — Less than
    Less,
    /// <= — Less or equal
    LessEqual,
    /// > — Greater than
    Greater,
    /// >= — Greater or equal
    GreaterEqual,

    // ═══════════════ Identifiers ═══════════════
    /// General identifier (variable or function name)
    Identifier(String),
    /// Colon-prefixed identifier (e.g. :id — route param)
    ParamIdentifier(String),

    // ═══════════════ Special ═══════════════
    /// Newline
    Newline,
    /// End of file
    Eof,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, line: usize, column: usize, offset: usize) -> Self {
        let lex = lexeme.into();
        let length = lex.len();
        Self {
            kind,
            lexeme: lex,
            span: Span {
                line,
                column,
                offset,
                length,
            },
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}('{}')", self.kind, self.lexeme)
    }
}
