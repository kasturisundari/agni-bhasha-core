/// # Abstract Syntax Tree — AST
///
/// AST Nodes representing program structure.
/// Each node maps to a syntactic rule in Braj Bhasha RK (ब्रजभाषा RK).

/// Complete Program node holding statements
#[derive(Debug, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}

/// Single statement
#[derive(Debug, Clone)]
pub enum Statement {
    /// Basic Sutra Rule
    /// `√dhatu+pratyaya [→ transform] [| condition] [:: result]`
    SutraRule(SutraRule),

    /// Module Import
    /// `आयात "module_name.sutra"`
    Import(String),

    /// Variable Assignment
    /// `√sṛj+ति·name ← value`
    Assignment {
        name: String,
        value: Box<Expr>,
    },

    /// Adhikara scope block with inheritance (anuvritti)
    /// `अधिकार context { ... }`
    Adhikara {
        context: Box<Expr>,
        body: Vec<Statement>,
    },

    /// Prakarana sub-scope block
    /// `प्रकरण context { ... }`
    Prakarana {
        context: Box<Expr>,
        body: Vec<Statement>,
    },

    /// Named Sutra (function definition)
    /// `सूत्र name(params) { body }`
    SutraDefinition {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },

    /// Ashtadhyayi term rewriting block
    /// `अष्टाध्यायी { ... }`
    Ashtadhyayi {
        body: Vec<Statement>,
    },

    /// Conditional branch block
    /// `यदि condition { ... } विकल्प { ... }`
    IfElse {
        condition: Box<Expr>,
        then_branch: Vec<Statement>,
        else_branch: Option<Vec<Statement>>,
    },


    /// For-Each loop block
    /// `प्रदक्षिणा item ← collection { ... }`
    ForEach {
        item: String,
        collection: Box<Expr>,
        body: Vec<Statement>,
    },

    /// Try/Catch exception block
    /// `प्रयत्न { ... } दोष err { ... }`
    TryCatch {
        try_block: Vec<Statement>,
        error_var: String,
        catch_block: Vec<Statement>,
    },

    /// Struct definition block
    /// `स्वरूप User { name, level }`
    StructDef {
        name: String,
        fields: Vec<String>,
    },

    /// While loop block
    /// `यावत् condition { ... }`
    WhileLoop {
        condition: Box<Expr>,
        body: Vec<Statement>,
    },

    /// Return statement
    /// `प्रतिदा value`
    Return(Box<Expr>),

    /// Pattern match block
    /// `प्रतिमान target { pattern { ... } अन्यथा { ... } }`
    Match {
        target: Box<Expr>,
        arms: Vec<(Expr, Vec<Statement>)>,
        default: Option<Vec<Statement>>,
    },

    /// Simple Expression statement
    Expression(Expr),
}

/// Declarative Sutra rule
#[derive(Debug, Clone)]
pub struct SutraRule {
    /// Source root and suffix
    pub source: DhatuExpr,
    /// Transform clause (optional)
    pub transform: Option<DhatuExpr>,
    /// Condition clause (optional)
    pub condition: Option<Box<Expr>>,
    /// Result clause (optional)
    pub result: Option<Box<Expr>>,
}

/// Root + Suffix expression representation
#[derive(Debug, Clone)]
pub struct DhatuExpr {
    /// Dhatu name (e.g. sṛj, dhā, vid)
    pub root: String,
    /// Suffix identifier (e.g. ti, syati)
    pub suffix: Option<String>,
    /// Param lists (e.g. ·8080)
    pub params: Vec<Expr>,
}

/// Generic expression representation
#[derive(Debug, Clone)]
pub enum Expr {
    /// Literal value
    Literal(Literal),

    /// Identifier variable name
    Identifier(String),

    /// Dhatu call expression (√dhatu+pratyaya)
    Dhatu(DhatuExpr),

    /// Binary operation (a + b, a == b, etc.)
    Binary {
        left: Box<Expr>,
        operator: BinaryOp,
        right: Box<Expr>,
    },

    /// Unary operation (-x, !x)
    Unary {
        operator: UnaryOp,
        operand: Box<Expr>,
    },

    /// Function or Sutra call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },

    /// Current loop index value (◈)
    CurrentElement,

    /// Lambda / Anonymous function
    /// `विद्युत्(params) { body }`
    Lambda {
        params: Vec<String>,
        body: Vec<Statement>,
    },

    /// Number range (1→10)
    Range {
        start: Box<Expr>,
        end: Box<Expr>,
    },

    /// Route param reference (:id)
    ParamRef(String),

    /// Embedded inline Sutra rule
    SutraExpr(SutraRule),

    /// Dictionary data representation
    /// `["key": value, "key2": value2]`
    Dict(Vec<(Expr, Expr)>),

    /// Map index lookup
    /// `user["name"]`
    IndexAccess {
        object: Box<Expr>,
        index: Box<Expr>,
    },

    /// Struct property access
    /// `user.name`
    PropertyAccess {
        object: Box<Expr>,
        property: String,
    },
}

/// Primitive Literals
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Str(String),
    Tattva(crate::evaluator::TattvaState),
    /// शून्य — Empty / Null state
    Shunya,
}

/// Binary operators
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    /// √yuj — Join / Concatenate
    Join,
    /// च — Logical AND
    And,
    /// वा — Logical OR
    Or,
}

/// Unary operators
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Integer(n) => write!(f, "{}", n),
            Literal::Float(n) => write!(f, "{}", n),
            Literal::Str(s) => write!(f, "\"{}\"", s),
            Literal::Tattva(t) => write!(f, "{}", match t {
                crate::evaluator::TattvaState::Sat => "सत्",
                crate::evaluator::TattvaState::Asat => "असत्",
                crate::evaluator::TattvaState::Sadasat => "सदसत्",
                crate::evaluator::TattvaState::Avyaktam => "अव्यक्तम्",
            }),
            Literal::Shunya => write!(f, "शून्य"),
        }
    }
}
