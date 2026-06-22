use super::ast::*;

pub fn format_program(program: &Program) -> String {
    let mut output = String::new();
    for stmt in &program.statements {
        output.push_str(&format_statement(stmt, 0));
        output.push_str("\n");
    }
    output
}

fn indent(level: usize) -> String {
    "    ".repeat(level)
}

fn format_statement(stmt: &Statement, level: usize) -> String {
    match stmt {
        Statement::Expression(expr) => {
            format!("{}{}", indent(level), format_expr(expr))
        }
        Statement::Import(path) => {
            format!("{}आयात \"{}\"", indent(level), path)
        }
        Statement::Assignment { name, value } => {
            format!("{}√sṛj+ति·{} ← {}", indent(level), name, format_expr(value))
        }
        Statement::SutraRule(rule) => {
            let mut s = format!("{}{}", indent(level), format_dhatu(&rule.source));
            if let Some(trans) = &rule.transform {
                s.push_str(&format!(" → {}", format_dhatu(trans)));
            }
            if let Some(cond) = &rule.condition {
                s.push_str(&format!(" | {}", format_expr(cond)));
            }
            if let Some(res) = &rule.result {
                s.push_str(&format!(" :: {}", format_expr(res)));
            }
            s
        }
        Statement::Adhikara { context, body } => {
            let mut s = format!("{}अधिकार {} {{\n", indent(level), format_expr(context));
            for b in body {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
        Statement::Prakarana { context, body } => {
            let mut s = format!("{}प्रकरण {} {{\n", indent(level), format_expr(context));
            for b in body {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
        Statement::SutraDefinition { name, params, body } => {
            let params_str = params.join(", ");
            let mut s = format!("{}सूत्र {}({}) {{\n", indent(level), name, params_str);
            for b in body {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
        Statement::Ashtadhyayi { body } => {
            let mut s = format!("{}अष्टाध्यायी {{\n", indent(level));
            for b in body {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
        Statement::IfElse { condition, then_branch, else_branch } => {
            let mut s = format!("{}यदि {} {{\n", indent(level), format_expr(condition));
            for b in then_branch {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            
            if let Some(els) = else_branch {
                s.push_str(" विकल्प {\n");
                for b in els {
                    s.push_str(&format_statement(b, level + 1));
                    s.push_str("\n");
                }
                s.push_str(&format!("{}}}", indent(level)));
            }
            s
        }
        Statement::ForEach { item, collection, body } => {
            let mut s = format!("{}प्रदक्षिणा {} ← {} {{\n", indent(level), item, format_expr(collection));
            for b in body {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
        Statement::TryCatch { try_block, error_var, catch_block } => {
            let mut s = format!("{}प्रयत्न {{\n", indent(level));
            for b in try_block {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}} दोष {} {{\n", indent(level), error_var));
            for b in catch_block {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
        Statement::StructDef { name, fields } => {
            format!("{}स्वरूप {} {{ {} }}", indent(level), name, fields.join(", "))
        }
        Statement::WhileLoop { condition, body } => {
            let mut s = format!("{}यावत् {} {{\n", indent(level), format_expr(condition));
            for b in body {
                s.push_str(&format_statement(b, level + 1));
                s.push_str("\n");
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
        Statement::Return(expr) => {
            format!("{}प्रतिदा {}", indent(level), format_expr(expr))
        }
        Statement::Match { target, arms, default } => {
            let mut s = format!("{}प्रतिमान {} {{\n", indent(level), format_expr(target));
            for (pattern, body) in arms {
                s.push_str(&format!("{}    {} {{\n", indent(level), format_expr(pattern)));
                for b in body {
                    s.push_str(&format_statement(b, level + 2));
                    s.push_str("\n");
                }
                s.push_str(&format!("{}    }}\n", indent(level)));
            }
            if let Some(def_body) = default {
                s.push_str(&format!("{}    अन्यथा {{\n", indent(level)));
                for b in def_body {
                    s.push_str(&format_statement(b, level + 2));
                    s.push_str("\n");
                }
                s.push_str(&format!("{}    }}\n", indent(level)));
            }
            s.push_str(&format!("{}}}", indent(level)));
            s
        }
    }
}

fn format_expr(expr: &Expr) -> String {
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::Integer(n) => n.to_string(),
            Literal::Float(n) => n.to_string(),
            Literal::Str(s) => format!("\"{}\"", s),
            Literal::Tattva(t) => match t {
                crate::evaluator::TattvaState::Sat => "सत्".to_string(),
                crate::evaluator::TattvaState::Asat => "असत्".to_string(),
                crate::evaluator::TattvaState::Sadasat => "सदसत्".to_string(),
                crate::evaluator::TattvaState::Avyaktam => "अव्यक्तम्".to_string(),
            },
            Literal::Shunya => "शून्य".to_string(),
        },
        Expr::Identifier(s) => s.clone(),
        Expr::Dhatu(d) => format_dhatu(d),
        Expr::Binary { left, operator, right } => {
            let op_str = match operator {
                BinaryOp::Add => "+",
                BinaryOp::Subtract => "-",
                BinaryOp::Multiply => "*",
                BinaryOp::Divide => "/",
                BinaryOp::Modulo => "%",
                BinaryOp::Equal => "==",
                BinaryOp::NotEqual => "!=",
                BinaryOp::Less => "<",
                BinaryOp::LessEqual => "<=",
                BinaryOp::Greater => ">",
                BinaryOp::GreaterEqual => ">=",
                BinaryOp::Join => "++",
                BinaryOp::And => "च",
                BinaryOp::Or => "वा",
            };
            format!("{} {} {}", format_expr(left), op_str, format_expr(right))
        }
        Expr::Unary { operator, operand } => {
            let op_str = match operator {
                UnaryOp::Negate => "-",
                UnaryOp::Not => "!",
            };
            format!("{}{}", op_str, format_expr(operand))
        }
        Expr::Call { callee, args } => {
            let args_str: Vec<String> = args.iter().map(|a| format_expr(a)).collect();
            format!("{}({})", format_expr(callee), args_str.join(", "))
        }
        Expr::CurrentElement => "◈".to_string(),
        Expr::Range { start, end } => format!("{}→{}", format_expr(start), format_expr(end)),
        Expr::ParamRef(name) => format!(":{}", name),
        Expr::SutraExpr(rule) => {
            let mut s = format_dhatu(&rule.source);
            if let Some(trans) = &rule.transform {
                s.push_str(&format!(" → {}", format_dhatu(trans)));
            }
            if let Some(cond) = &rule.condition {
                s.push_str(&format!(" | {}", format_expr(cond)));
            }
            if let Some(res) = &rule.result {
                s.push_str(&format!(" :: {}", format_expr(res)));
            }
            s
        }
        Expr::Dict(pairs) => {
            let pairs_str: Vec<String> = pairs.iter().map(|(k, v)| format!("{}: {}", format_expr(k), format_expr(v))).collect();
            format!("[{}]", pairs_str.join(", "))
        }
        Expr::IndexAccess { object, index } => {
            format!("{}[{}]", format_expr(object), format_expr(index))
        }
        Expr::PropertyAccess { object, property } => {
            format!("{}.{}", format_expr(object), property)
        }
        Expr::Lambda { params, body } => {
            let mut s = format!("विद्युत्({}) {{\n", params.join(", "));
            for b in body {
                s.push_str(&format_statement(b, 1));
                s.push_str("\n");
            }
            s.push_str("}");
            s
        }
    }
}

fn format_dhatu(dhatu: &DhatuExpr) -> String {
    let mut s = format!("√{}", dhatu.root);
    if let Some(suf) = &dhatu.suffix {
        s.push_str(&format!("+{}", suf));
    }
    for p in &dhatu.params {
        s.push_str(&format!("·{}", format_expr(p)));
    }
    s
}
