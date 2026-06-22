/// # Frequency Matrix — Cosmic Resonance
///
/// Each ShivaState has a specific frequency. When combining or sequencing states,
/// frequencies interact to produce logical resonance that dictates engine behaviors.

use super::sutras::ShivaState;

/// Frequency interaction matrix
/// Determines interaction output between two states
#[derive(Debug, Clone)]
pub struct FrequencyMatrix {
    /// 14x14 grid defining state frequency interactions
    interactions: [[FrequencyInteraction; 14]; 14],
}

/// Result of interaction between two frequencies
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrequencyInteraction {
    /// Frequencies resonate — operation proceeds smoothly
    Resonant,
    /// Frequencies oppose — operation requires conversion
    Dissonant,
    /// Frequencies are neutral — operation proceeds as is
    Neutral,
    /// Frequencies amplify — operation expands
    Amplified,
}

impl FrequencyMatrix {
    /// Create default interaction matrix based on natural relationships
    pub fn new() -> Self {
        let mut interactions = [[FrequencyInteraction::Neutral; 14]; 14];

        // Define baseline interactions
        for i in 0..14 {
            // Every frequency resonates with itself
            interactions[i][i] = FrequencyInteraction::Resonant;

            // Neighboring states are resonant
            if i + 1 < 14 {
                interactions[i][i + 1] = FrequencyInteraction::Resonant;
                interactions[i + 1][i] = FrequencyInteraction::Resonant;
            }
        }

        // Creation + Dissolution = Amplified (Complete cycle)
        interactions[0][13] = FrequencyInteraction::Amplified;
        interactions[13][0] = FrequencyInteraction::Amplified;

        // Force + Protection = Dissonant (Conflict)
        interactions[7][10] = FrequencyInteraction::Dissonant;
        interactions[10][7] = FrequencyInteraction::Dissonant;

        // Harmony + Structure = Amplified (Building)
        interactions[2][9] = FrequencyInteraction::Amplified;
        interactions[9][2] = FrequencyInteraction::Amplified;

        // Flow + Motion = Resonant (Natural momentum)
        interactions[1][4] = FrequencyInteraction::Resonant;
        interactions[4][1] = FrequencyInteraction::Resonant;

        // Binding + Dissolution = Dissonant (Contradiction)
        interactions[5][13] = FrequencyInteraction::Dissonant;
        interactions[13][5] = FrequencyInteraction::Dissonant;

        Self { interactions }
    }

    /// Query interaction between two states
    pub fn interaction(&self, a: ShivaState, b: ShivaState) -> FrequencyInteraction {
        self.interactions[a.index()][b.index()]
    }

    /// Calculate combined frequency of a sequence of states
    pub fn composite_frequency(&self, states: &[ShivaState]) -> f64 {
        if states.is_empty() {
            return 0.0;
        }
        if states.len() == 1 {
            return states[0].frequency();
        }

        let mut total = states[0].frequency();
        for i in 1..states.len() {
            let interaction = self.interaction(states[i - 1], states[i]);
            let freq = states[i].frequency();
            total = match interaction {
                FrequencyInteraction::Resonant => total + freq,
                FrequencyInteraction::Dissonant => (total - freq).abs(),
                FrequencyInteraction::Neutral => total + freq * 0.5,
                FrequencyInteraction::Amplified => total * freq,
            };
        }
        total
    }
}

impl Default for FrequencyMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_resonance() {
        let matrix = FrequencyMatrix::new();
        for state in ShivaState::all() {
            assert_eq!(
                matrix.interaction(*state, *state),
                FrequencyInteraction::Resonant
            );
        }
    }

    #[test]
    fn test_creation_dissolution_amplified() {
        let matrix = FrequencyMatrix::new();
        assert_eq!(
            matrix.interaction(ShivaState::Creation, ShivaState::Dissolution),
            FrequencyInteraction::Amplified
        );
    }

    #[test]
    fn test_composite_frequency_resonant() {
        let matrix = FrequencyMatrix::new();
        let states = vec![ShivaState::Creation, ShivaState::Flow];
        let freq = matrix.composite_frequency(&states);
        // Creation(1.0) + Flow(2.0) = 3.0 (resonant neighbors)
        assert_eq!(freq, 3.0);
    }

    #[test]
    fn test_composite_frequency_empty() {
        let matrix = FrequencyMatrix::new();
        let states: Vec<ShivaState> = vec![];
        let freq = matrix.composite_frequency(&states);
        assert_eq!(freq, 0.0);
    }

    #[test]
    fn test_composite_frequency_single() {
        let matrix = FrequencyMatrix::new();
        let states = vec![ShivaState::Protection]; // freq = 11.0
        let freq = matrix.composite_frequency(&states);
        assert_eq!(freq, 11.0);
    }

    #[test]
    fn test_interaction_force_protection_dissonant() {
        let matrix = FrequencyMatrix::new();
        assert_eq!(
            matrix.interaction(ShivaState::Force, ShivaState::Protection),
            FrequencyInteraction::Dissonant
        );
        assert_eq!(
            matrix.interaction(ShivaState::Protection, ShivaState::Force),
            FrequencyInteraction::Dissonant
        );
    }

    #[test]
    fn test_interaction_harmony_structure_amplified() {
        let matrix = FrequencyMatrix::new();
        assert_eq!(
            matrix.interaction(ShivaState::Harmony, ShivaState::Structure),
            FrequencyInteraction::Amplified
        );
    }

    #[test]
    fn test_interaction_binding_dissolution_dissonant() {
        let matrix = FrequencyMatrix::new();
        assert_eq!(
            matrix.interaction(ShivaState::Binding, ShivaState::Dissolution),
            FrequencyInteraction::Dissonant
        );
    }

    #[test]
    fn test_composite_dissonant() {
        let matrix = FrequencyMatrix::new();
        let states = vec![ShivaState::Binding, ShivaState::Dissolution]; 
        // Binding=6.0, Dissolution=14.0, Dissonant => abs(6.0 - 14.0) = 8.0
        let freq = matrix.composite_frequency(&states);
        assert_eq!(freq, 8.0);
    }

    #[test]
    fn test_composite_amplified() {
        let matrix = FrequencyMatrix::new();
        let states = vec![ShivaState::Harmony, ShivaState::Structure]; 
        // Harmony=3.0, Structure=10.0, Amplified => 3.0 * 10.0 = 30.0
        let freq = matrix.composite_frequency(&states);
        assert_eq!(freq, 30.0);
    }

    #[test]
    fn test_composite_neutral() {
        let matrix = FrequencyMatrix::new();
        // Creation=1.0, Binding=6.0 -> neutral since not neighbors and not explicit
        // Neutral logic: total + freq * 0.5 => 1.0 + 6.0*0.5 = 4.0
        let states = vec![ShivaState::Creation, ShivaState::Binding];
        let freq = matrix.composite_frequency(&states);
        assert_eq!(freq, 4.0);
    }

    #[test]
    fn test_matrix_default_is_new() {
        let m1 = FrequencyMatrix::default();
        let m2 = FrequencyMatrix::new();
        assert_eq!(m1.interaction(ShivaState::Creation, ShivaState::Flow), m2.interaction(ShivaState::Creation, ShivaState::Flow));
    }

    #[test]
    fn test_long_sequence_frequency() {
        let matrix = FrequencyMatrix::new();
        let states = vec![
            ShivaState::Creation, // 1.0
            ShivaState::Flow,     // 2.0 -> Resonant -> 3.0
            ShivaState::Harmony,  // 3.0 -> Resonant -> 6.0
            ShivaState::Structure,// 10.0 -> Amplified -> 6.0 * 10.0 = 60.0
            ShivaState::Force     // 8.0 -> Neutral -> 60.0 + 8.0*0.5 = 64.0
        ];
        let freq = matrix.composite_frequency(&states);
        assert_eq!(freq, 64.0);
    }

    #[test]
    fn test_interaction_symmetry() {
        let matrix = FrequencyMatrix::new();
        for s1 in ShivaState::all() {
            for s2 in ShivaState::all() {
                assert_eq!(matrix.interaction(*s1, *s2), matrix.interaction(*s2, *s1));
            }
        }
    }
}
