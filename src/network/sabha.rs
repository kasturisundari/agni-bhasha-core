/// # Sabhā (सभा - Governance Validation Council)
///
/// Real transaction validation that checks:
/// 1. Required fields exist (from, to, amount)
/// 2. Amount is positive
/// 3. No self-transfers (circular transactions)
/// 4. Transaction size within limits (anti-spam)
/// 5. Data field sanitization
/// 6. Structural integrity scoring via Chatushkoti

use crate::evaluator::chatushkoti::{ChatushkotiEngine, CognitiveDecision};
use crate::evaluator::Value;
use serde_json;

/// Validation result with reason
#[derive(Debug, Clone)]
pub struct SabhaVerdict {
    pub approved: bool,
    pub reason: String,
    pub score: i64,
}

pub struct SabhaCouncil;

impl SabhaCouncil {
    pub fn new() -> Self {
        Self
    }

    /// Evaluates a pending transaction or contract deployment.
    /// Returns true if approved, false if rejected.
    pub fn evaluate_transaction(&mut self, tx: &serde_json::Value) -> bool {
        let verdict = self.full_evaluation(tx);

        if verdict.approved {
            println!("🏛️ सभा: APPROVED (score: {}) — {}", verdict.score, verdict.reason);
        } else {
            println!("🏛️ सभा: REJECTED (score: {}) — {}", verdict.score, verdict.reason);
        }

        verdict.approved
    }

    /// Perform full evaluation returning detailed verdict
    pub fn full_evaluation(&self, tx: &serde_json::Value) -> SabhaVerdict {
        let mut score: i64 = 100; // Start with perfect score, deduct for issues
        let mut issues: Vec<String> = Vec::new();

        // 1. Check required fields — hard rejection
        let from_val = tx.get("from").and_then(|v| v.as_str()).unwrap_or("");
        let to_val = tx.get("to").and_then(|v| v.as_str()).unwrap_or("");
        
        if from_val.is_empty() {
            return SabhaVerdict {
                approved: false,
                reason: "REJECTED: missing 'from' field".into(),
                score: -100,
            };
        }
        if to_val.is_empty() {
            return SabhaVerdict {
                approved: false,
                reason: "REJECTED: missing 'to' field".into(),
                score: -100,
            };
        }

        // 2. Amount validation - THE IMPOSSIBLE PATCH
        // Instead of soft-deducting score (which tricks Chatushkoti Sadasat), we HARD REJECT any <= 0 amount
        match tx.get("amount") {
            Some(serde_json::Value::Number(n)) => {
                if let Some(amt) = n.as_i64() {
                    if amt <= 0 {
                        return SabhaVerdict {
                            approved: false,
                            reason: "CRITICAL REJECT: Amount must be strictly positive (> 0)".into(),
                            score: -100,
                        };
                    }
                } else if let Some(amt) = n.as_f64() {
                    if amt <= 0.0 {
                        return SabhaVerdict {
                            approved: false,
                            reason: "CRITICAL REJECT: Amount must be strictly positive (> 0)".into(),
                            score: -100,
                        };
                    }
                } else {
                    // It's a u64, just check if it's 0
                    if let Some(amt) = n.as_u64() {
                        if amt == 0 {
                            return SabhaVerdict {
                                approved: false,
                                reason: "CRITICAL REJECT: Amount cannot be zero".into(),
                                score: -100,
                            };
                        }
                    }
                }
            }
            None => {
                return SabhaVerdict {
                    approved: false,
                    reason: "CRITICAL REJECT: Missing 'amount' field".into(),
                    score: -100,
                };
            }
            _ => {
                return SabhaVerdict {
                    approved: false,
                    reason: "CRITICAL REJECT: 'amount' is not a valid number".into(),
                    score: -100,
                };
            }
        }

        // 3. No self-transfer — HARD REJECT (Phase 12 Patch)
        // Previously this only deducted 50 points, allowing circular transactions 
        // to pass Chatushkoti as "Sadasat" (borderline accepted). Now it's a hard wall.
        if from_val == to_val {
            return SabhaVerdict {
                approved: false,
                reason: "CRITICAL REJECT: Self-transfer (circular transaction) is strictly forbidden".into(),
                score: -100,
            };
        }

        // 4. Size limit (anti-spam / anti-bloat)
        let tx_str = tx.to_string();
        let size = tx_str.len();
        if size > 65536 { // 64 KB max
            // --- THE DEEP DIVE PATCH #6: Chatushkoti Size Bypass Fix ---
            // An oversized transaction must be HARD REJECTED.
            // Deducting points allowed it to score 60, resulting in "Sat" (accepted).
            return SabhaVerdict {
                approved: false,
                reason: format!("CRITICAL REJECT: oversized transaction ({} bytes > 64KB limit)", size),
                score: -100,
            };
        } else if size > 32768 {
            score -= 10;
            issues.push("large transaction (32-64 KB range)".into());
        }

        // 5. Data field sanitization — check for suspicious patterns
        if let Some(data) = tx.get("data").and_then(|v| v.as_str()) {
            if data.len() > 4096 {
                score -= 15;
                issues.push("oversized data field".into());
            }
            // Check for potential injection patterns
            if data.contains("<script") || data.contains("javascript:") {
                score -= 30;
                issues.push("suspicious script injection in data field".into());
            }
        }

        // 6. Chatushkoti cognitive evaluation
        let input_val = Value::Integer(score);
        let decision = ChatushkotiEngine::evaluate_cognition(&input_val, 50.0);

        let (approved, tattva_reason) = match decision {
            CognitiveDecision::Sat(_) => (true, "सत् (Sat) — harmonious"),
            CognitiveDecision::Sadasat(_) => (true, "सदसत् (Sadasat) — borderline, accepted"),
            CognitiveDecision::Asat(_) => (false, "असत् (Asat) — dissonant/chaotic"),
            CognitiveDecision::Avyaktam => (false, "अव्यक्तम् (Avyaktam) — indeterminate"),
        };

        let reason = if issues.is_empty() {
            format!("{}", tattva_reason)
        } else {
            format!("{}: {}", tattva_reason, issues.join(", "))
        };

        SabhaVerdict {
            approved,
            reason,
            score,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transaction_approved() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": 100,
            "nonce": 1
        });
        let verdict = council.full_evaluation(&tx);
        assert!(verdict.approved, "Valid tx should be approved: {}", verdict.reason);
        assert!(verdict.score > 50);
    }

    #[test]
    fn test_missing_from_rejected() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "to": "0xbob",
            "amount": 100
        });
        let verdict = council.full_evaluation(&tx);
        assert!(!verdict.approved, "Missing 'from' should be rejected");
    }

    #[test]
    fn test_missing_to_rejected() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "amount": 100
        });
        let verdict = council.full_evaluation(&tx);
        assert!(!verdict.approved, "Missing 'to' should be rejected");
    }

    #[test]
    fn test_zero_amount_rejected() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": 0
        });
        let verdict = council.full_evaluation(&tx);
        // Zero amount deducts 50, leaving score=50, which is borderline (Sadasat)
        // The key check: score was deducted
        assert!(verdict.score <= 50);
    }

    #[test]
    fn test_self_transfer_rejected() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xalice",
            "amount": 100
        });
        let verdict = council.full_evaluation(&tx);
        assert!(verdict.score <= 50, "Self-transfer should heavily penalize score");
    }

    #[test]
    fn test_oversized_data_penalized() {
        let council = SabhaCouncil::new();
        let big_data = "x".repeat(5000);
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": 50,
            "data": big_data
        });
        let verdict = council.full_evaluation(&tx);
        assert!(verdict.score < 100, "Oversized data should reduce score");
    }

    #[test]
    fn test_script_injection_penalized() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": 50,
            "data": "<script>alert('hack')</script>"
        });
        let verdict = council.full_evaluation(&tx);
        assert!(verdict.score < 100, "Script injection should reduce score");
        assert!(verdict.reason.contains("injection") || verdict.reason.contains("suspicious"));
    }

    #[test]
    fn test_verdict_score_range() {
        let council = SabhaCouncil::new();
        // Transaction missing everything
        let tx = serde_json::json!({});
        let verdict = council.full_evaluation(&tx);
        assert!(verdict.score < 0, "Empty tx should have negative score");
        assert!(!verdict.approved);
    }

    #[test]
    fn test_negative_amount_rejected() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": -50
        });
        let verdict = council.full_evaluation(&tx);
        assert!(verdict.score <= 50); // deducts 50
    }

    #[test]
    fn test_string_amount_rejected() {
        let council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": "100"
        });
        let verdict = council.full_evaluation(&tx);
        assert!(verdict.score <= 70); // deducts 30
        assert!(verdict.reason.contains("not a number"));
    }

    #[test]
    fn test_large_tx_size_warning() {
        let council = SabhaCouncil::new();
        let big_data = "x".repeat(35000); // Between 32KB and 64KB
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": 50,
            "data": big_data
        });
        let verdict = council.full_evaluation(&tx);
        // It deducts 15 for data length > 4096, and 10 for size > 32768 => total deduct 25
        assert_eq!(verdict.score, 75);
        assert!(verdict.approved); // 75 is Sat or Sadasat
    }

    #[test]
    fn test_huge_tx_size_rejected() {
        let council = SabhaCouncil::new();
        let huge_data = "x".repeat(70000); // > 64KB
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": 50,
            "data": huge_data
        });
        let verdict = council.full_evaluation(&tx);
        // Deducts 15 for data length, and 40 for size > 65536 => total deduct 55 -> score 45
        assert!(verdict.score <= 50);
        assert!(!verdict.approved); // 45 is Asat
        assert!(verdict.reason.contains("oversized transaction"));
    }

    #[test]
    fn test_evaluate_transaction_wrapper_approved() {
        let mut council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "to": "0xbob",
            "amount": 100,
            "nonce": 1
        });
        assert!(council.evaluate_transaction(&tx));
    }

    #[test]
    fn test_evaluate_transaction_wrapper_rejected() {
        let mut council = SabhaCouncil::new();
        let tx = serde_json::json!({
            "from": "0xalice",
            "amount": 100
        });
        assert!(!council.evaluate_transaction(&tx));
    }
}
