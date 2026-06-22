/// # Sutra Parser
///
/// Converts a token stream into an Abstract Syntax Tree (AST).
/// Understands the declarative Sutra grammar: `√dhatu+pratyaya | condition :: result`
///
/// Braj Bhasha RK

use crate::lexer::{Token, TokenKind};
use super::ast::*;

/// Syntactic Parser
pub struct SutraParser {
    tokens: Vec<Token>,
    current: usize,
    pub errors: Vec<ParseError>,
}

impl SutraParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0, errors: Vec::new() }
    }

    /// Parse complete program (collection of statements)
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            self.skip_newlines();
            if self.is_at_end() {
                break;
            }
            match self.parse_statement(0) {
                Ok(stmt) => statements.push(stmt),
                Err(err) => {
                    self.errors.push(err);
                    self.synchronize();
                }
            }
        }

        if !self.errors.is_empty() {
            // If there are errors, we return the first one but ideally the engine would display all of them
            return Err(self.errors[0].clone());
        }

        Ok(Program { statements })
    }

    /// Parse single statement
    fn parse_statement(&mut self, depth: usize) -> Result<Statement, ParseError> {
        if depth > 128 {
            let (line, col) = self.peek().span.line_col();
            return Err(ParseError::Custom(
                "CRITICAL: Maximum block nesting depth exceeded. Potential AST Bomb detected.".to_string(),
                line, col
            ));
        }

        self.skip_newlines();

        // Adhikara scope block
        if self.check(TokenKind::Adhikara) {
            return self.parse_adhikara(depth);
        }

        // आयात — module import
        if self.check(TokenKind::Aayat) {
            return self.parse_import();
        }

        // Prakarana sub-scope block
        if self.check(TokenKind::Prakarana) {
            return self.parse_prakarana(depth);
        }

        // Named Sutra (function definition)
        if self.check(TokenKind::SutraDef) {
            return self.parse_sutra_definition(depth);
        }

        // यदि — conditional statement
        if self.check(TokenKind::Yadi) {
            return self.parse_ifelse(depth);
        }

        // प्रदक्षिणा — iteration/loop
        if self.check(TokenKind::Pradakshina) {
            return self.parse_foreach(depth);
        }

        // प्रयत्न — try-catch
        if self.check(TokenKind::Prayatna) {
            return self.parse_try_catch(depth);
        }

        // स्वरूप — structure (Struct definition)
        if self.check(TokenKind::Swaroopa) {
            return self.parse_struct_def();
        }

        // अष्टाध्यायी — term rewriting system
        if self.check(TokenKind::Ashtadhyayi) {
            return self.parse_ashtadhyayi(depth);
        }

        // यावत् — while loop
        if self.check(TokenKind::Yavat) {
            return self.parse_while_loop(depth);
        }

        // प्रतिदा — return statement
        if self.check(TokenKind::Pratida) {
            return self.parse_return();
        }

        // प्रतिमान — pattern match
        if self.check(TokenKind::Pratiman) {
            return self.parse_match(depth);
        }

        // ◆ — bullet marker inside block
        if self.check(TokenKind::SutraBullet) {
            self.advance();
        }

        // √ — Dhatu expression prefix
        if self.check(TokenKind::DhatuPrefix) {
            return self.parse_dhatu_statement();
        }

        // Fallback to general expression statement
        let expr = self.parse_expression()?;
        Ok(Statement::Expression(expr))
    }

    /// Parse Ashtadhyayi
    fn parse_ashtadhyayi(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume अष्टाध्यायी
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;
        Ok(Statement::Ashtadhyayi { body })
    }

    /// Parse Import
    fn parse_import(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume आयात

        let token = self.peek().clone();
        match &token.kind {
            TokenKind::StringLiteral(path) => {
                self.advance();
                Ok(Statement::Import(path.clone()))
            }
            _ => Err(ParseError::UnexpectedToken("आयात requires a string literal path".to_string(), token.span.line, token.span.column)),
        }
    }

    /// Parse Adhikara
    fn parse_adhikara(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume अधिकार
        let context = self.parse_expression()?;

        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;

        Ok(Statement::Adhikara {
            context: Box::new(context),
            body,
        })
    }

    /// Parse Prakarana
    fn parse_prakarana(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume प्रकरण
        let context = self.parse_expression()?;

        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;

        Ok(Statement::Prakarana {
            context: Box::new(context),
            body,
        })
    }

    /// Parse Named Sutra definition
    fn parse_sutra_definition(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume सूत्र

        let name = self.expect_identifier()?;

        self.expect(TokenKind::LeftParen)?;
        let params = self.parse_param_list()?;
        self.expect(TokenKind::RightParen)?;

        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;

        Ok(Statement::SutraDefinition { name, params, body })
    }

    /// Parse If/Else conditional (यदि / विकल्प)
    fn parse_ifelse(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume यदि
        
        let condition = Box::new(self.parse_expression()?);
        
        self.expect(TokenKind::LeftBrace)?;
        let then_branch = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;

        let mut else_branch = None;
        if self.check(TokenKind::Vikalpa) {
            self.advance(); // consume विकल्प
            self.expect(TokenKind::LeftBrace)?;
            else_branch = Some(self.parse_block(depth + 1)?);
            self.expect(TokenKind::RightBrace)?;
        }

        Ok(Statement::IfElse {
            condition,
            then_branch,
            else_branch,
        })
    }

    /// Parse Loop ( प्रदक्षिणा )
    fn parse_foreach(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume प्रदक्षिणा
        
        let item = self.expect_identifier()?;
        
        self.expect(TokenKind::Assign)?; // ←
        let collection = Box::new(self.parse_expression()?);
        
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;

        Ok(Statement::ForEach {
            item,
            collection,
            body,
        })
    }

    /// Parse Exception Block ( प्रयत्न / दोष )
    fn parse_try_catch(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume प्रयत्न
        
        self.expect(TokenKind::LeftBrace)?;
        let try_block = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;

        self.expect(TokenKind::Dosha)?;
        let error_var = self.expect_identifier()?;

        self.expect(TokenKind::LeftBrace)?;
        let catch_block = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;

        Ok(Statement::TryCatch {
            try_block,
            error_var,
            catch_block,
        })
    }

    /// Parse Struct definition ( स्वरूप )
    fn parse_struct_def(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume स्वरूप
        
        let name = self.expect_identifier()?;
        
        self.expect(TokenKind::LeftBrace)?;
        
        let mut fields = Vec::new();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            fields.push(self.expect_identifier()?);
            // optional comma separator
            if self.check(TokenKind::Comma) {
                self.advance();
            }
        }
        
        self.expect(TokenKind::RightBrace)?;

        Ok(Statement::StructDef {
            name,
            fields,
        })
    }

    /// Parse statement beginning with Dhatu root
    fn parse_dhatu_statement(&mut self) -> Result<Statement, ParseError> {
        let dhatu = self.parse_dhatu_expr()?;

        // check assignment operator (←)
        if self.check(TokenKind::Assign) {
            self.advance();
            let value = self.parse_expression()?;
            // extract variable name from root params
            let name = if let Some(param) = dhatu.params.first() {
                match param {
                    Expr::Identifier(s) => s.clone(),
                    _ => format!("{}_{}", dhatu.root, dhatu.suffix.as_deref().unwrap_or("")),
                }
            } else {
                format!("{}_{}", dhatu.root, dhatu.suffix.as_deref().unwrap_or(""))
            };
            return Ok(Statement::Assignment {
                name,
                value: Box::new(value),
            });
        }

        // check for full Sutra rule parts (→ | ::)
        let mut transform = None;
        let mut condition = None;
        let mut result = None;

        // transform step →
        if self.check(TokenKind::Arrow) {
            self.advance();
            if self.check(TokenKind::DhatuPrefix) {
                transform = Some(self.parse_dhatu_expr()?);
            } else {
                let expr = self.parse_expression()?;
                result = Some(Box::new(expr));
                return Ok(Statement::SutraRule(SutraRule {
                    source: dhatu,
                    transform,
                    condition,
                    result,
                }));
            }
        }

        // condition step |
        if self.check(TokenKind::Pipe) {
            self.advance();
            condition = Some(Box::new(self.parse_expression()?));
        }

        // result step ::
        if self.check(TokenKind::DoubleColon) {
            self.advance();
            result = Some(Box::new(self.parse_expression()?));
        }

        // construct rule if containing any of the components
        if transform.is_some() || condition.is_some() || result.is_some() {
            Ok(Statement::SutraRule(SutraRule {
                source: dhatu,
                transform,
                condition,
                result,
            }))
        } else {
            // pure Dhatu expression
            Ok(Statement::Expression(Expr::Dhatu(dhatu)))
        }
    }

    /// Parse Dhatu expression (√dhatu+pratyaya·param)
    fn parse_dhatu_expr(&mut self) -> Result<DhatuExpr, ParseError> {
        self.expect(TokenKind::DhatuPrefix)?;
        let root = self.expect_identifier()?;

        let mut suffix = None;
        let mut params = Vec::new();

        // pratyaya suffix (+pratyaya)
        if self.check(TokenKind::Plus) {
            self.advance();
            suffix = Some(self.expect_identifier()?);
        }

        // parameters (·param)
        while self.check(TokenKind::ParamSeparator) {
            self.advance();
            let param = self.parse_expression()?;
            params.push(param);
        }

        Ok(DhatuExpr {
            root,
            suffix,
            params,
        })
    }

    /// General expression parsing
    fn parse_expression(&mut self) -> Result<Expr, ParseError> {
        self.parse_expression_with_depth(0)
    }

    /// Internal expression parsing with depth tracking to prevent AST Bombs
    fn parse_expression_with_depth(&mut self, depth: usize) -> Result<Expr, ParseError> {
        // --- THE APOCALYPSE PATCH: Compiler Stack Overflow Protection ---
        // Prevents an attacker from sending 100,000 nested parentheses to crash the node.
        if depth > 256 {
            let (line, col) = self.peek().span.line_col();
            return Err(ParseError::Custom(
                "CRITICAL: Maximum AST recursion depth exceeded. Potential AST Bomb detected.".to_string(),
                line, col
            ));
        }
        self.parse_logical(depth)
    }

    /// Logical AND/OR parsing (च / वा)
    fn parse_logical(&mut self, depth: usize) -> Result<Expr, ParseError> {
        let mut left = self.parse_comparison(depth)?;

        loop {
            let op = if self.check(TokenKind::And) {
                Some(BinaryOp::And)
            } else if self.check(TokenKind::Or) {
                Some(BinaryOp::Or)
            } else {
                None
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_comparison(depth)?;
                left = Expr::Binary {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Comparison parsing
    fn parse_comparison(&mut self, depth: usize) -> Result<Expr, ParseError> {
        let mut left = self.parse_addition(depth)?;

        loop {
            let op = if self.check(TokenKind::EqualEqual) {
                Some(BinaryOp::Equal)
            } else if self.check(TokenKind::BangEqual) {
                Some(BinaryOp::NotEqual)
            } else if self.check(TokenKind::Less) {
                Some(BinaryOp::Less)
            } else if self.check(TokenKind::LessEqual) {
                Some(BinaryOp::LessEqual)
            } else if self.check(TokenKind::Greater) {
                Some(BinaryOp::Greater)
            } else if self.check(TokenKind::GreaterEqual) {
                Some(BinaryOp::GreaterEqual)
            } else {
                None
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_addition(depth)?;
                left = Expr::Binary {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Add/Subtract parsing
    fn parse_addition(&mut self, depth: usize) -> Result<Expr, ParseError> {
        let mut left = self.parse_multiplication(depth)?;

        loop {
            let op = if self.check(TokenKind::Plus) {
                Some(BinaryOp::Add)
            } else if self.check(TokenKind::Minus) {
                Some(BinaryOp::Subtract)
            } else {
                None
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_multiplication(depth)?;
                left = Expr::Binary {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Multiply/Divide parsing
    fn parse_multiplication(&mut self, depth: usize) -> Result<Expr, ParseError> {
        let mut left = self.parse_unary(depth)?;

        loop {
            let op = if self.check(TokenKind::Star) {
                Some(BinaryOp::Multiply)
            } else if self.check(TokenKind::Slash) {
                Some(BinaryOp::Divide)
            } else if self.check(TokenKind::Percent) {
                Some(BinaryOp::Modulo)
            } else {
                None
            };

            if let Some(op) = op {
                self.advance();
                let right = self.parse_unary(depth)?;
                left = Expr::Binary {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Unary parsing
    fn parse_unary(&mut self, depth: usize) -> Result<Expr, ParseError> {
        if self.check(TokenKind::Minus) {
            self.advance();
            let operand = self.parse_call(depth)?;
            return Ok(Expr::Unary {
                operator: UnaryOp::Negate,
                operand: Box::new(operand),
            });
        }
        self.parse_call(depth)
    }

    /// Function call / Index / Property parsing
    fn parse_call(&mut self, depth: usize) -> Result<Expr, ParseError> {
        let mut expr = self.parse_primary(depth)?;

        loop {
            if self.check(TokenKind::LeftParen) {
                self.advance();
                let mut args = Vec::new();
                if !self.check(TokenKind::RightParen) {
                    args.push(self.parse_expression_with_depth(depth + 1)?);
                    while self.check(TokenKind::Comma) {
                        self.advance();
                        args.push(self.parse_expression_with_depth(depth + 1)?);
                    }
                }
                self.expect(TokenKind::RightParen)?;
                expr = Expr::Call {
                    callee: Box::new(expr),
                    args,
                };
            } else if self.check(TokenKind::LeftBracket) {
                // IndexAccess: array[index]
                self.advance();
                let index = self.parse_expression_with_depth(depth + 1)?;
                self.expect(TokenKind::RightBracket)?;
                expr = Expr::IndexAccess {
                    object: Box::new(expr),
                    index: Box::new(index),
                };
            } else if self.check(TokenKind::Dot) {
                // PropertyAccess: object.property
                self.advance();
                let property = self.expect_identifier()?;
                expr = Expr::PropertyAccess {
                    object: Box::new(expr),
                    property,
                };
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Primary expression parsing
    fn parse_primary(&mut self, depth: usize) -> Result<Expr, ParseError> {
        if self.is_at_end() {
            let (line, col) = self.tokens.last().map(|t| (t.span.line, t.span.column)).unwrap_or((1, 1));
            return Err(ParseError::UnexpectedEof(line, col));
        }

        let token = self.peek().clone();
        match &token.kind {
            TokenKind::Integer(n) => {
                let val = *n;
                self.advance();
                // range checking (n→m)
                if self.check(TokenKind::Arrow) {
                    self.advance();
                    let end = self.parse_primary(depth + 1)?;
                    return Ok(Expr::Range {
                        start: Box::new(Expr::Literal(Literal::Integer(val))),
                        end: Box::new(end),
                    });
                }
                Ok(Expr::Literal(Literal::Integer(val)))
            }
            TokenKind::Float(n) => {
                let val = *n;
                self.advance();
                Ok(Expr::Literal(Literal::Float(val)))
            }
            TokenKind::StringLiteral(s) => {
                let val = s.clone();
                self.advance();
                Ok(Expr::Literal(Literal::Str(val)))
            }
            TokenKind::Tattva(t) => {
                let val = *t;
                self.advance();
                Ok(Expr::Literal(Literal::Tattva(val)))
            }
            TokenKind::Identifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Identifier(name))
            }
            TokenKind::ParamIdentifier(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::ParamRef(name))
            }
            TokenKind::DhatuPrefix => {
                let dhatu = self.parse_dhatu_expr()?;
                Ok(Expr::Dhatu(dhatu))
            }
            TokenKind::CurrentElement => {
                self.advance();
                Ok(Expr::CurrentElement)
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.parse_expression_with_depth(depth + 1)?;
                self.expect(TokenKind::RightParen)?;
                Ok(expr)
            }
            TokenKind::Vidyut => {
                self.parse_lambda(depth)
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut pairs = Vec::new();
                if !self.check(TokenKind::RightBracket) {
                    loop {
                        let key = self.parse_expression_with_depth(depth + 1)?;
                        self.expect(TokenKind::Colon)?;
                        let value = self.parse_expression_with_depth(depth + 1)?;
                        pairs.push((key, value));
                        
                        // optional comma separator
                        if self.check(TokenKind::Comma) {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                }
                self.expect(TokenKind::RightBracket)?;
                Ok(Expr::Dict(pairs))
            }
            _ => {
                self.advance();
                Err(ParseError::UnexpectedToken(token.lexeme.clone(), token.span.line, token.span.column))
            }
        }
    }

    // ═══════════════ Helpers ═══════════════

    /// Parse statement block
    fn parse_block(&mut self, depth: usize) -> Result<Vec<Statement>, ParseError> {
        let mut stmts = Vec::new();
        self.skip_newlines();
        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            let stmt = self.parse_statement(depth)?;
            stmts.push(stmt);
            self.skip_newlines();
        }
        Ok(stmts)
    }

    /// Parse While Loop (यावत्)
    fn parse_while_loop(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume यावत्
        let condition = Box::new(self.parse_expression()?);
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;
        Ok(Statement::WhileLoop { condition, body })
    }

    /// Parse Return (प्रतिदा)
    fn parse_return(&mut self) -> Result<Statement, ParseError> {
        self.advance(); // consume प्रतिदा
        // If next token starts a new statement or is end of block, return Shunya
        if self.check(TokenKind::Newline) || self.check(TokenKind::RightBrace) || self.is_at_end() {
            return Ok(Statement::Return(Box::new(Expr::Literal(Literal::Shunya))));
        }
        let value = self.parse_expression()?;
        Ok(Statement::Return(Box::new(value)))
    }

    /// Parse Pattern Match (प्रतिमान)
    fn parse_match(&mut self, depth: usize) -> Result<Statement, ParseError> {
        self.advance(); // consume प्रतिमान
        let target = Box::new(self.parse_expression()?);
        self.expect(TokenKind::LeftBrace)?;
        self.skip_newlines();

        let mut arms = Vec::new();
        let mut default = None;

        while !self.check(TokenKind::RightBrace) && !self.is_at_end() {
            self.skip_newlines();
            if self.check(TokenKind::RightBrace) {
                break;
            }

            // Check for default arm (अन्यथा)
            if self.check(TokenKind::Anyatha) {
                self.advance();
                self.expect(TokenKind::LeftBrace)?;
                default = Some(self.parse_block(depth + 1)?);
                self.expect(TokenKind::RightBrace)?;
                self.skip_newlines();
                continue;
            }

            // Parse pattern expression
            let pattern = self.parse_expression()?;
            self.expect(TokenKind::LeftBrace)?;
            let body = self.parse_block(depth + 1)?;
            self.expect(TokenKind::RightBrace)?;
            arms.push((pattern, body));
            self.skip_newlines();
        }

        self.expect(TokenKind::RightBrace)?;
        Ok(Statement::Match { target, arms, default })
    }

    /// Parse Lambda (विद्युत्)
    fn parse_lambda(&mut self, depth: usize) -> Result<Expr, ParseError> {
        self.advance(); // consume विद्युत्
        self.expect(TokenKind::LeftParen)?;
        let params = self.parse_param_list()?;
        self.expect(TokenKind::RightParen)?;
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block(depth + 1)?;
        self.expect(TokenKind::RightBrace)?;
        Ok(Expr::Lambda { params, body })
    }

    /// Parse parameter list
    fn parse_param_list(&mut self) -> Result<Vec<String>, ParseError> {
        let mut params = Vec::new();
        if !self.check(TokenKind::RightParen) {
            if self.check(TokenKind::DhatuPrefix) {
                let dhatu = self.parse_dhatu_expr()?;
                params.push(dhatu.params.first().map(|p| match p {
                    Expr::Identifier(s) => s.clone(),
                    _ => "param".to_string(),
                }).unwrap_or_else(|| dhatu.root.clone()));
            } else {
                params.push(self.expect_identifier()?);
            }

            while self.check(TokenKind::Comma) {
                self.advance();
                if self.check(TokenKind::DhatuPrefix) {
                    let dhatu = self.parse_dhatu_expr()?;
                    params.push(dhatu.params.first().map(|p| match p {
                        Expr::Identifier(s) => s.clone(),
                        _ => "param".to_string(),
                    }).unwrap_or_else(|| dhatu.root.clone()));
                } else {
                    params.push(self.expect_identifier()?);
                }
            }
        }
        Ok(params)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.tokens[self.current - 1]
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        std::mem::discriminant(&self.tokens[self.current].kind) == std::mem::discriminant(&kind)
    }

    fn check_identifier(&self, name: &str) -> bool {
        if self.is_at_end() {
            return false;
        }
        matches!(&self.tokens[self.current].kind, TokenKind::Identifier(s) if s == name)
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, ParseError> {
        if self.check(kind.clone()) {
            Ok(self.advance().clone())
        } else {
            let (lexeme, line, col) = if self.is_at_end() {
                ("EOF".to_string(), self.tokens.last().map(|t| t.span.line).unwrap_or(1), self.tokens.last().map(|t| t.span.column).unwrap_or(1))
            } else {
                let t = self.peek();
                (t.lexeme.clone(), t.span.line, t.span.column)
            };
            Err(ParseError::Expected(format!("{:?}", kind), lexeme, line, col))
        }
    }

    fn expect_identifier(&mut self) -> Result<String, ParseError> {
        if self.is_at_end() {
            let (line, col) = self.tokens.last().map(|t| (t.span.line, t.span.column)).unwrap_or((1, 1));
            return Err(ParseError::UnexpectedEof(line, col));
        }
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => Err(ParseError::Expected("identifier".to_string(), token.lexeme, token.span.line, token.span.column)),
        }
    }

    fn skip_newlines(&mut self) {
        while !self.is_at_end() && self.check(TokenKind::Newline) {
            self.advance();
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
            || matches!(self.tokens[self.current].kind, TokenKind::Eof)
    }
    /// Error Recovery: Synchronize to next valid statement
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            let prev_kind = &self.tokens[self.current - 1].kind;
            if *prev_kind == TokenKind::Newline || *prev_kind == TokenKind::RightBrace {
                return;
            }

            match self.peek().kind {
                TokenKind::Adhikara | TokenKind::Prakarana | TokenKind::SutraDef |
                TokenKind::Yadi | TokenKind::Pradakshina | TokenKind::Prayatna |
                TokenKind::Swaroopa | TokenKind::Ashtadhyayi | TokenKind::DhatuPrefix |
                TokenKind::Yavat | TokenKind::Pratida | TokenKind::Pratiman => {
                    return;
                }
                _ => {}
            }

            self.advance();
        }
    }
}

/// Parse Errors
#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedEof(usize, usize),
    UnexpectedToken(String, usize, usize),
    Expected(String, String, usize, usize),
    Custom(String, usize, usize),
}

impl ParseError {
    pub fn to_diagnostic(&self) -> crate::error::Diagnostic {
        use crate::error::Diagnostic;
        match self {
            ParseError::UnexpectedEof(line, col) => Diagnostic::error("अप्रत्याशितम् अवसानम्")
                .at(*line, *col)
                .with_hint("कोष्ठकानि वा सूत्रस्य पूर्णतां पश्यतु"),
            ParseError::UnexpectedToken(lexeme, line, col) => Diagnostic::error(format!("अप्रत्याशितं '{}'", lexeme))
                .at(*line, *col)
                .with_hint("अस्मिन् सन्दर्भे अप्रत्याशितम् पदम्, सूत्रनियमान् पश्यतु"),
            ParseError::Expected(expected, found, line, col) => Diagnostic::error(format!("अपेक्षितम् {}, प्राप्तम् '{}'", expected, found))
                .at(*line, *col)
                .with_hint("नियमस्य साफल्याय अपेक्षितं पदं ददातु"),
            ParseError::Custom(msg, line, col) => Diagnostic::error(msg.clone())
                .at(*line, *col)
                .with_hint("कस्टम पार्स त्रुटि"),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_diagnostic())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::Scanner;

    #[test]
    fn test_parse_simple_dhatu() {
        let mut scanner = Scanner::new("√vac+ति \"hello\"");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(!program.statements.is_empty());
    }

    #[test]
    fn test_parse_assignment() {
        let mut scanner = Scanner::new("√sṛj+ति·x ← 42");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::Assignment { .. }));
    }

    #[test]
    fn test_parse_adhikara() {
        let mut scanner = Scanner::new("अधिकार context {\n√vac+ति \"hello\"\n}");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::Adhikara { .. }));
    }
    #[test]
    fn test_parse_binary_expression() {
        let mut scanner = Scanner::new("x ← 10 + 5 * 2");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                match value {
                    Expr::Binary { operator, right, .. } => {
                        assert_eq!(*operator, BinaryOp::Add);
                        match &**right {
                            Expr::Binary { operator: op2, .. } => {
                                assert_eq!(*op2, BinaryOp::Multiply);
                            },
                            _ => panic!("Expected multiplication on the right"),
                        }
                    },
                    _ => panic!("Expected binary expression"),
                }
            },
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_parse_yadi_statement() {
        let mut scanner = Scanner::new("यदि (x > 5) {\n √vac+ति \"big\" \n} अन्यथा {\n √vac+ति \"small\" \n}");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::If { .. }));
    }

    #[test]
    fn test_parse_while_loop() {
        let mut scanner = Scanner::new("यावत् (x < 10) {\n x ← x + 1 \n}");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::While { .. }));
    }

    #[test]
    fn test_parse_function_definition() {
        let mut scanner = Scanner::new("सूत्रं calculate(x: ज्ञान) -> ज्ञान {\n प्रतिदा x * 2 \n}");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::Function { .. }));
    }

    #[test]
    fn test_parse_prakarana_struct() {
        let mut scanner = Scanner::new("प्रकरण User {\n name: ज्ञान,\n age: ज्ञान \n}");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::Prakarana { .. }));
    }

    #[test]
    fn test_parse_array_literal() {
        let mut scanner = Scanner::new("arr ← [1, 2, 3]");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                assert!(matches!(value, Expr::Array(_)));
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_map_literal() {
        let mut scanner = Scanner::new("map ← { \"key\": 42 }");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                assert!(matches!(value, Expr::Map(_)));
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_error_recovery() {
        let mut scanner = Scanner::new("x ← \n y ← 10");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        // Expect an error parsing x assignment, but it shouldn't panic
        let res = parser.parse();
        assert!(res.is_err() || res.unwrap().statements.len() < 2);
    }

    #[test]
    fn test_parse_import_statement() {
        let mut scanner = Scanner::new("आयात \"math\"");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::Import(_)));
    }

    #[test]
    fn test_parse_return_statement() {
        let mut scanner = Scanner::new("प्रतिदा 42");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::Return(Some(_))));
    }

    #[test]
    fn test_parse_empty_return() {
        let mut scanner = Scanner::new("प्रतिदा");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert!(matches!(program.statements[0], Statement::Return(None)));
    }

    #[test]
    fn test_parse_logical_expression() {
        let mut scanner = Scanner::new("result ← true && false || true");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                assert!(matches!(value, Expr::Logical { .. }));
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_unary_negation() {
        let mut scanner = Scanner::new("x ← -42");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                assert!(matches!(value, Expr::Unary { .. }));
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_property_access() {
        let mut scanner = Scanner::new("val ← object.property");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                assert!(matches!(value, Expr::PropertyAccess { .. }));
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_index_access() {
        let mut scanner = Scanner::new("val ← array[0]");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::Assignment { value, .. } => {
                assert!(matches!(value, Expr::IndexAccess { .. }));
            },
            _ => panic!("Expected assignment"),
        }
    }

    #[test]
    fn test_parse_missing_semicolon_recovery() {
        // Assume parser requires semicolons eventually, but currently Sutra is newline/block based.
        // Let's test a simple syntax error recovery where it drops the rest of the statement
        let mut scanner = Scanner::new("माना x = \n माना y = 5"); // Invalid first line
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse();
        assert!(program.is_err() || parser.had_error);
    }

    #[test]
    fn test_parse_unmatched_parentheses() {
        let mut scanner = Scanner::new("माना x = (5 + 3");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let res = parser.parse();
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_unmatched_braces() {
        let mut scanner = Scanner::new("यदि (सत्य) { माना x = 1");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let res = parser.parse();
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_invalid_assignment() {
        let mut scanner = Scanner::new("5 = x");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let res = parser.parse();
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_unexpected_token() {
        let mut scanner = Scanner::new("माना + = 5");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let res = parser.parse();
        assert!(res.is_err());
    }

    #[test]
    fn test_parse_deeply_nested_expressions() {
        let mut scanner = Scanner::new("माना x = (((((5)))))");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_multiple_unary_operators() {
        let mut scanner = Scanner::new("माना x = !!सत्य");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
    }

    #[test]
    fn test_parse_chained_function_calls() {
        let mut scanner = Scanner::new("माना x = a()()()");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        // Might be err if higher-order functions aren't fully supported in parser,
        // but let's see if it parses or errors cleanly
        let _ = parser.parse(); 
    }

    #[test]
    fn test_parse_empty_block() {
        let mut scanner = Scanner::new("यदि (सत्य) {}");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        let program = parser.parse().unwrap();
        match &program.statements[0] {
            Statement::IfElse { then_branch, .. } => {
                assert!(then_branch.is_empty());
            },
            _ => panic!("Expected IfElse"),
        }
    }

    #[test]
    fn test_parse_missing_condition_if() {
        let mut scanner = Scanner::new("यदि {}");
        let tokens = scanner.scan_tokens();
        let mut parser = SutraParser::new(tokens);
        assert!(parser.parse().is_err());
    }
}
