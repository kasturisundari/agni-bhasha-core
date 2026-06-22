use crate::evaluator::{Value, TattvaState, RuntimeError};

/// Cognitive Decision based on the 4 states of Chatushkoti
#[derive(Debug, Clone)]
pub enum CognitiveDecision {
    /// सत् (Sat) - Deterministically True / Exists
    Sat(Value),
    /// असत् (Asat) - Deterministically False / Does Not Exist
    Asat(Value),
    /// सदसत् (Sadasat) - Superposition (Both True and False depending on observation)
    Sadasat(Value),
    /// अव्यक्तम् (Avyaktam) - Unmanifest / Indeterminable / Undecidable
    Avyaktam,
}

pub struct ChatushkotiEngine;

impl ChatushkotiEngine {
    /// Evaluate a condition using 4D Vedic Logic (√bodh - Cognition)
    pub fn evaluate_cognition(input: &Value, threshold: f64) -> CognitiveDecision {
        match input {
            Value::Integer(n) => {
                if *n > threshold as i64 {
                    CognitiveDecision::Sat(input.clone())
                } else if *n < 0 {
                    CognitiveDecision::Asat(input.clone())
                } else {
                    CognitiveDecision::Sadasat(input.clone()) // Borderline state
                }
            }
            Value::Float(f) => {
                if *f > threshold {
                    CognitiveDecision::Sat(input.clone())
                } else if *f < 0.0 {
                    CognitiveDecision::Asat(input.clone())
                } else {
                    CognitiveDecision::Sadasat(input.clone())
                }
            }
            Value::Tattva(t) => {
                match t {
                    TattvaState::Sat => CognitiveDecision::Sat(input.clone()),
                    TattvaState::Asat => CognitiveDecision::Asat(input.clone()),
                    TattvaState::Sadasat => CognitiveDecision::Sadasat(input.clone()),
                    TattvaState::Avyaktam => CognitiveDecision::Avyaktam,
                }
            }
            Value::List(l) => {
                if l.is_empty() {
                    CognitiveDecision::Avyaktam
                } else if l.len() as f64 >= threshold {
                    CognitiveDecision::Sat(input.clone())
                } else {
                    CognitiveDecision::Sadasat(input.clone())
                }
            }
            _ => CognitiveDecision::Avyaktam, // Strings, Dicts, etc without clear truth context
        }
    }

    /// Resolve a cognitive decision into a strict boolean or throw an Avyaktam error (√nirṇay - Decision)
    pub fn resolve_decision(decision: CognitiveDecision) -> Result<Value, RuntimeError> {
        match decision {
            CognitiveDecision::Sat(v) => Ok(v),
            CognitiveDecision::Asat(_) => Ok(Value::Shunya),
            // Quantum collapse: Sadasat collapses to Sat if observed under strict resolution
            CognitiveDecision::Sadasat(v) => Ok(v),
            CognitiveDecision::Avyaktam => Err(RuntimeError::General(
                "अव्यक्तम्: Cognitive engine cannot determine a state (Indeterminable). Execution halted gracefully.".into()
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cognition_integer_sat() {
        let val = Value::Integer(10);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Sat(_)));
    }

    #[test]
    fn test_cognition_integer_asat() {
        let val = Value::Integer(-10);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Asat(_)));
    }

    #[test]
    fn test_cognition_integer_sadasat() {
        let val = Value::Integer(3);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Sadasat(_)));
    }

    #[test]
    fn test_cognition_float_sat() {
        let val = Value::Float(10.5);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Sat(_)));
    }

    #[test]
    fn test_cognition_float_asat() {
        let val = Value::Float(-1.5);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Asat(_)));
    }

    #[test]
    fn test_cognition_float_sadasat() {
        let val = Value::Float(2.5);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Sadasat(_)));
    }

    #[test]
    fn test_cognition_tattva_sat() {
        let val = Value::Tattva(TattvaState::Sat);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Sat(_)));
    }

    #[test]
    fn test_cognition_tattva_avyaktam() {
        let val = Value::Tattva(TattvaState::Avyaktam);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Avyaktam));
    }

    #[test]
    fn test_cognition_list_empty_avyaktam() {
        let val = Value::List(vec![]);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 2.0);
        assert!(matches!(decision, CognitiveDecision::Avyaktam));
    }

    #[test]
    fn test_cognition_list_sat() {
        let val = Value::List(vec![Value::Integer(1), Value::Integer(2)]);
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 2.0);
        assert!(matches!(decision, CognitiveDecision::Sat(_)));
    }

    #[test]
    fn test_cognition_string_avyaktam() {
        let val = Value::Str("test".to_string());
        let decision = ChatushkotiEngine::evaluate_cognition(&val, 5.0);
        assert!(matches!(decision, CognitiveDecision::Avyaktam));
    }

    #[test]
    fn test_resolve_sat() {
        let dec = CognitiveDecision::Sat(Value::Integer(100));
        let res = ChatushkotiEngine::resolve_decision(dec).unwrap();
        assert_eq!(res, Value::Integer(100));
    }

    #[test]
    fn test_resolve_asat() {
        let dec = CognitiveDecision::Asat(Value::Integer(100));
        let res = ChatushkotiEngine::resolve_decision(dec).unwrap();
        assert_eq!(res, Value::Shunya);
    }

    #[test]
    fn test_resolve_sadasat_collapses_to_value() {
        let dec = CognitiveDecision::Sadasat(Value::Integer(50));
        let res = ChatushkotiEngine::resolve_decision(dec).unwrap();
        assert_eq!(res, Value::Integer(50));
    }

    #[test]
    fn test_resolve_avyaktam_errors() {
        let dec = CognitiveDecision::Avyaktam;
        let res = ChatushkotiEngine::resolve_decision(dec);
        assert!(res.is_err());
        if let Err(e) = res {
            assert!(e.to_string().contains("अव्यक्तम्"));
        } else {
            panic!("Expected Avyaktam error");
        }
    }
}
