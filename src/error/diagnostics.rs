/// #   — Diagnostics
///
///     .
/// राधा कृष्ण भाषा

///  
#[derive(Debug, Clone, PartialEq)]
pub enum DiagnosticLevel {
    /// त्रुटि — 
    Error,
    /// चेतावनी — 
    Warning,
    /// सूचना — 
    Info,
}

///   
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn error(msg: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Error,
            message: msg.into(),
            line: None,
            column: None,
            hint: None,
        }
    }

    pub fn warning(msg: impl Into<String>) -> Self {
        Self {
            level: DiagnosticLevel::Warning,
            message: msg.into(),
            line: None,
            column: None,
            hint: None,
        }
    }

    pub fn at(mut self, line: usize, col: usize) -> Self {
        self.line = Some(line);
        self.column = Some(col);
        self
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prefix = match self.level {
            DiagnosticLevel::Error => "✗ त्रुटि",
            DiagnosticLevel::Warning => "⚠ चेतावनी",
            DiagnosticLevel::Info => "ℹ सूचना",
        };

        write!(f, "{}: {}", prefix, self.message)?;

        if let (Some(line), Some(col)) = (self.line, self.column) {
            write!(f, " [{}:{}]", line, col)?;
        }

        if let Some(hint) = &self.hint {
            write!(f, "\n  → संकेत: {}", hint)?;
        }

        Ok(())
    }
}
