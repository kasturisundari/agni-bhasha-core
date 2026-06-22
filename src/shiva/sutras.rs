/// # Shiva Sutras — The 14 Harmonic States
///
/// Each sutra represents a cosmic state in the processor.
/// These states are the foundation on which all calculations are executed.

/// The 14 vibrational states derived from Maheshvara Sutras
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ShivaState {
    /// 1. a i u Ṇ — Creation
    Creation,
    /// 2. ṛ ḷ K — Flow
    Flow,
    /// 3. e o Ṅ — Harmony
    Harmony,
    /// 4. ai au C — Expansion
    Expansion,
    /// 5. ha ya va ra Ṭ — Motion
    Motion,
    /// 6. la Ṇ — Binding
    Binding,
    /// 7. ña ma ṅa ṇa na M — Resonance
    Resonance,
    /// 8. jha bha Ñ — Force
    Force,
    /// 9. gha ḍha dha Ṣ — Transform
    Transform,
    /// 10. ja ba ga ḍa da Ś — Structure
    Structure,
    /// 11. kha pha cha ṭha tha ca ṭa ta V — Protection
    Protection,
    /// 12. ka pa Y — Causation
    Causation,
    /// 13. śa ṣa sa R — Purification
    Purification,
    /// 14. ha L — Dissolution
    Dissolution,
}

impl ShivaState {
    /// Numerical index (0-13) for each state
    pub fn index(&self) -> usize {
        *self as usize
    }

    /// Retrieve state from numeric index
    pub fn from_index(i: usize) -> Option<Self> {
        match i {
            0 => Some(Self::Creation),
            1 => Some(Self::Flow),
            2 => Some(Self::Harmony),
            3 => Some(Self::Expansion),
            4 => Some(Self::Motion),
            5 => Some(Self::Binding),
            6 => Some(Self::Resonance),
            7 => Some(Self::Force),
            8 => Some(Self::Transform),
            9 => Some(Self::Structure),
            10 => Some(Self::Protection),
            11 => Some(Self::Causation),
            12 => Some(Self::Purification),
            13 => Some(Self::Dissolution),
            _ => None,
        }
    }

    /// Original Sanskrit phonemes for each sutra
    pub fn phonemes(&self) -> &'static [&'static str] {
        match self {
            Self::Creation => &["a", "i", "u"],
            Self::Flow => &["ṛ", "ḷ"],
            Self::Harmony => &["e", "o"],
            Self::Expansion => &["ai", "au"],
            Self::Motion => &["ha", "ya", "va", "ra"],
            Self::Binding => &["la"],
            Self::Resonance => &["ña", "ma", "ṅa", "ṇa", "na"],
            Self::Force => &["jha", "bha"],
            Self::Transform => &["gha", "ḍha", "dha"],
            Self::Structure => &["ja", "ba", "ga", "ḍa", "da"],
            Self::Protection => &["kha", "pha", "cha", "ṭha", "tha", "ca", "ṭa", "ta"],
            Self::Causation => &["ka", "pa"],
            Self::Purification => &["śa", "ṣa", "sa"],
            Self::Dissolution => &["ha"],
        }
    }

    /// Anubandha marker character for each sutra
    pub fn anubandha(&self) -> char {
        match self {
            Self::Creation => 'Ṇ',
            Self::Flow => 'K',
            Self::Harmony => 'Ṅ',
            Self::Expansion => 'C',
            Self::Motion => 'Ṭ',
            Self::Binding => 'Ṇ',
            Self::Resonance => 'M',
            Self::Force => 'Ñ',
            Self::Transform => 'Ṣ',
            Self::Structure => 'Ś',
            Self::Protection => 'V',
            Self::Causation => 'Y',
            Self::Purification => 'R',
            Self::Dissolution => 'L',
        }
    }

    /// English name for the state
    pub fn english_name(&self) -> &'static str {
        match self {
            Self::Creation => "Creation",
            Self::Flow => "Flow",
            Self::Harmony => "Harmony",
            Self::Expansion => "Expansion",
            Self::Motion => "Motion",
            Self::Binding => "Binding",
            Self::Resonance => "Resonance",
            Self::Force => "Force",
            Self::Transform => "Transform",
            Self::Structure => "Structure",
            Self::Protection => "Protection",
            Self::Causation => "Causation",
            Self::Purification => "Purification",
            Self::Dissolution => "Dissolution",
        }
    }

    /// Sanskrit Devanagari name for the state
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Creation => "सृष्टि",
            Self::Flow => "प्रवाह",
            Self::Harmony => "सामञ्जस्य",
            Self::Expansion => "विस्तार",
            Self::Motion => "गति",
            Self::Binding => "बन्ध",
            Self::Resonance => "अनुनाद",
            Self::Force => "बल",
            Self::Transform => "परिवर्तन",
            Self::Structure => "संरचना",
            Self::Protection => "रक्षा",
            Self::Causation => "हेतु",
            Self::Purification => "शोधन",
            Self::Dissolution => "लय",
        }
    }

    /// Computational role description
    pub fn computational_role(&self) -> &'static str {
        match self {
            Self::Creation => "Variable creation, object instantiation, memory allocation",
            Self::Flow => "Data streaming, pipeline continuation, iteration flow",
            Self::Harmony => "Type merging, union types, data joining",
            Self::Expansion => "Collection expansion, loop unrolling, recursion",
            Self::Motion => "Data transfer, function calls, message passing",
            Self::Binding => "Variable binding, reference creation, closure capture",
            Self::Resonance => "Event emission, signal propagation, inter-process communication",
            Self::Force => "Forced execution, assertion, panic/abort",
            Self::Transform => "Type conversion, data mapping, morphological transformation",
            Self::Structure => "Struct/class definition, schema creation, pattern matching",
            Self::Protection => "Validation, authentication, error guarding, type checking",
            Self::Causation => "Conditional logic, cause-effect chains, triggers",
            Self::Purification => "Filtering, sanitization, garbage collection",
            Self::Dissolution => "Deallocation, destruction, scope exit, resource release",
        }
    }

    /// Associated resonant frequency value
    pub fn frequency(&self) -> f64 {
        match self {
            Self::Creation => 1.0,
            Self::Flow => 2.0,
            Self::Harmony => 3.0,
            Self::Expansion => 5.0,
            Self::Motion => 8.0,
            Self::Binding => 13.0,
            Self::Resonance => 21.0,
            Self::Force => 34.0,
            Self::Transform => 55.0,
            Self::Structure => 89.0,
            Self::Protection => 144.0,
            Self::Causation => 233.0,
            Self::Purification => 377.0,
            Self::Dissolution => 610.0,
        }
    }

    /// List all states in order
    pub fn all() -> &'static [ShivaState] {
        &[
            Self::Creation,
            Self::Flow,
            Self::Harmony,
            Self::Expansion,
            Self::Motion,
            Self::Binding,
            Self::Resonance,
            Self::Force,
            Self::Transform,
            Self::Structure,
            Self::Protection,
            Self::Causation,
            Self::Purification,
            Self::Dissolution,
        ]
    }
}

impl std::fmt::Display for ShivaState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}[{}]", self, self.sanskrit_name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_14_states_exist() {
        assert_eq!(ShivaState::all().len(), 14);
    }

    #[test]
    fn test_index_roundtrip() {
        for state in ShivaState::all() {
            let idx = state.index();
            assert_eq!(ShivaState::from_index(idx), Some(*state));
        }
    }

    #[test]
    fn test_fibonacci_frequencies() {
        // Frequencies follow Fibonacci sequence
        let freqs: Vec<f64> = ShivaState::all().iter().map(|s| s.frequency()).collect();
        for i in 2..freqs.len() {
            assert_eq!(freqs[i], freqs[i - 1] + freqs[i - 2]);
        }
    }
}
