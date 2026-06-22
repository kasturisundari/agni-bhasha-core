/// # Cosmic Dhatu Library — Dhatu Library
///
/// Computational roots (Dhatus) from which actions are derived.
/// Each root represents a immutable computational concept.

use std::collections::HashMap;
use crate::shiva::ShivaState;

/// Single Dhatu Root
#[derive(Debug, Clone)]
pub struct DhatuRoot {
    /// Romanized name
    pub name: String,
    /// Devanagari name
    pub devanagari: String,
    /// General meaning description
    pub meaning: String,
    /// Computational action description
    pub computational_meaning: String,
    /// Associated Shiva state frequency
    pub shiva_state: ShivaState,
    /// Gana (Class)
    pub gana: DhatuGana,
    /// Whether it is a compiler builtin
    pub is_builtin: bool,
}

/// Gana classification
#[derive(Debug, Clone, PartialEq)]
pub enum DhatuGana {
    /// Creation and initialization
    Srishti,
    /// Query and information retrieval
    Jnana,
    /// Motion and loops
    Gati,
    /// State transformation
    Parivartan,
    /// Layout and structure
    Sanrachana,
    /// Networking and connectivity
    Sambandha,
    /// Security and validation
    Raksha,
    /// Math and profiling
    Ganana,
}

/// Dhatu registry
#[derive(Clone)]
pub struct DhatuRegistry {
    roots: HashMap<String, DhatuRoot>,
}

impl DhatuRegistry {
    /// Create registry containing builtin roots
    pub fn new() -> Self {
        let mut registry = Self {
            roots: HashMap::new(),
        };
        registry.register_computational_roots();
        crate::dhatu::extended_roots::register_extended_roots(&mut registry);
        registry
    }

    /// Insert a raw root directly
    pub fn insert_raw(&mut self, root: DhatuRoot) {
        self.roots.insert(root.name.clone(), root);
    }

    /// Register core computational roots (~30 prime roots)
    fn register_computational_roots(&mut self) {
        let roots = vec![
            // ═══ Srishti (Creation) ═══
            DhatuRoot {
                name: "sṛj".into(), devanagari: "सृज्".into(),
                meaning: "Creation / Manifestation".into(),
                computational_meaning: "Create variable, instantiate object, allocate".into(),
                shiva_state: ShivaState::Creation, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "bhū".into(), devanagari: "भू".into(),
                meaning: "Being / Existence".into(),
                computational_meaning: "Existence check, boolean evaluation, state assertion".into(),
                shiva_state: ShivaState::Creation, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "jan".into(), devanagari: "जन्".into(),
                meaning: "Birth / Generation".into(),
                computational_meaning: "Generate, spawn process, create instance".into(),
                shiva_state: ShivaState::Creation, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "kḷp".into(), devanagari: "कॢप्".into(),
                meaning: "Format / Configure".into(),
                computational_meaning: "Configure, format, shape data structure".into(),
                shiva_state: ShivaState::Creation, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "nirmā".into(), devanagari: "निर्मा".into(),
                meaning: "Build / Construct".into(),
                computational_meaning: "Compile, assemble, build from source".into(),
                shiva_state: ShivaState::Creation, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "āviṣ".into(), devanagari: "आविष्".into(),
                meaning: "Discover / Reveal".into(),
                computational_meaning: "Discover nodes, expose interface, reveal data".into(),
                shiva_state: ShivaState::Creation, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "utpad".into(), devanagari: "उत्पद्".into(),
                meaning: "Produce / Yield".into(),
                computational_meaning: "Yield from generator, produce event".into(),
                shiva_state: ShivaState::Creation, gana: DhatuGana::Srishti, is_builtin: true,
            },

            // ═══ Persistence (Binding) ═══
            DhatuRoot {
                name: "dhā".into(), devanagari: "धा".into(),
                meaning: "Hold / Store / Persist".into(),
                computational_meaning: "Store, save to database, persist state".into(),
                shiva_state: ShivaState::Binding, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "sthā".into(), devanagari: "स्था".into(),
                meaning: "Stand / Stay / Immutable".into(),
                computational_meaning: "Persist, make immutable, pin in memory".into(),
                shiva_state: ShivaState::Binding, gana: DhatuGana::Srishti, is_builtin: true,
            },
            DhatuRoot {
                name: "grah".into(), devanagari: "ग्रह्".into(),
                meaning: "Seize / Catch / Capture".into(),
                computational_meaning: "Hold reference, capture closure, cache".into(),
                shiva_state: ShivaState::Binding, gana: DhatuGana::Srishti, is_builtin: true,
            },

            // ═══ Jnana (Query & Knowledge) ═══
            DhatuRoot {
                name: "vid".into(), devanagari: "विद्".into(),
                meaning: "Knowledge / Finding".into(),
                computational_meaning: "Query, lookup, find, search".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "jñā".into(), devanagari: "ज्ञा".into(),
                meaning: "Cognition / Understanding".into(),
                computational_meaning: "Type inference, pattern recognition, AI cognition".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "dṛś".into(), devanagari: "दृश्".into(),
                meaning: "Viewing / Inspecting".into(),
                computational_meaning: "View, display, render, inspect".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "cint".into(), devanagari: "चिन्त्".into(),
                meaning: "Thought / Computation".into(),
                computational_meaning: "Process, evaluate, compute complex expression".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "anveṣ".into(), devanagari: "अन्वेष्".into(),
                meaning: "Search / Investigate".into(),
                computational_meaning: "Deep search, scan directory, grep".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "pramā".into(), devanagari: "प्रमा".into(),
                meaning: "Measure / Quantify".into(),
                computational_meaning: "Calculate size, measure performance, benchmark".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "vicār".into(), devanagari: "विचार".into(),
                meaning: "Analyze / Deliberate".into(),
                computational_meaning: "Static analysis, type checking, deliberate logic".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Jnana, is_builtin: true,
            },

            // ═══ Gati (Motion & Flow) ═══
            DhatuRoot {
                name: "gam".into(), devanagari: "गम्".into(),
                meaning: "Going / Transfer".into(),
                computational_meaning: "Move, transfer, navigate, goto".into(),
                shiva_state: ShivaState::Motion, gana: DhatuGana::Gati, is_builtin: true,
            },
            DhatuRoot {
                name: "car".into(), devanagari: "चर्".into(),
                meaning: "Iterating / Walking".into(),
                computational_meaning: "Iterate, loop, traverse, walk".into(),
                shiva_state: ShivaState::Motion, gana: DhatuGana::Gati, is_builtin: true,
            },
            DhatuRoot {
                name: "sṛ".into(), devanagari: "सृ".into(),
                meaning: "Flow / Streaming".into(),
                computational_meaning: "Stream, flow data, pipeline".into(),
                shiva_state: ShivaState::Motion, gana: DhatuGana::Gati, is_builtin: true,
            },
            DhatuRoot {
                name: "prāp".into(), devanagari: "प्राप्".into(),
                meaning: "Reaching / Awaiting".into(),
                computational_meaning: "Fetch, receive, await, get result".into(),
                shiva_state: ShivaState::Motion, gana: DhatuGana::Gati, is_builtin: true,
            },
            DhatuRoot {
                name: "dhāv".into(), devanagari: "धाव्".into(),
                meaning: "Running / Executing".into(),
                computational_meaning: "Run, execute process, sprint".into(),
                shiva_state: ShivaState::Motion, gana: DhatuGana::Gati, is_builtin: true,
            },
            DhatuRoot {
                name: "uḍ".into(), devanagari: "उड्".into(),
                meaning: "Flying / Elevating".into(),
                computational_meaning: "Elevate privileges, lift, jump over".into(),
                shiva_state: ShivaState::Motion, gana: DhatuGana::Gati, is_builtin: true,
            },
            DhatuRoot {
                name: "plu".into(), devanagari: "प्लु".into(),
                meaning: "Floating / Drifting".into(),
                computational_meaning: "Float value, background process, detach".into(),
                shiva_state: ShivaState::Motion, gana: DhatuGana::Gati, is_builtin: true,
            },

            // ═══ Sambandha (Connectivity & Communication) ═══
            DhatuRoot {
                name: "vac".into(), devanagari: "वच्".into(),
                meaning: "Speech / Printing".into(),
                computational_meaning: "Print, output, log, display text".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Sambandha, is_builtin: true,
            },
            DhatuRoot {
                name: "śru".into(), devanagari: "श्रु".into(),
                meaning: "Hearing / Subscribing".into(),
                computational_meaning: "Listen, receive input, subscribe, await event".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Sambandha, is_builtin: true,
            },
            DhatuRoot {
                name: "preṣ".into(), devanagari: "प्रेष्".into(),
                meaning: "Sending / Emitting".into(),
                computational_meaning: "Send message, emit event, broadcast, notify".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Sambandha, is_builtin: true,
            },
            DhatuRoot {
                name: "bandh".into(), devanagari: "बन्ध्".into(),
                meaning: "Binding / Connection".into(),
                computational_meaning: "Bind, connect, establish link, open connection".into(),
                shiva_state: ShivaState::Binding, gana: DhatuGana::Sambandha, is_builtin: true,
            },
            DhatuRoot {
                name: "prativac".into(), devanagari: "प्रतिवच्".into(),
                meaning: "Replying / Responding".into(),
                computational_meaning: "Reply to request, callback, respond".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Sambandha, is_builtin: true,
            },
            DhatuRoot {
                name: "āmantr".into(), devanagari: "आमन्त्र्".into(),
                meaning: "Inviting / Calling".into(),
                computational_meaning: "Invite, ping, invoke remote procedure".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Sambandha, is_builtin: true,
            },

            // ═══ Parivartan (Transformation) ═══
            DhatuRoot {
                name: "kṛ".into(), devanagari: "कृ".into(),
                meaning: "Doing / Executing".into(),
                computational_meaning: "Do, execute, perform action, apply".into(),
                shiva_state: ShivaState::Transform, gana: DhatuGana::Parivartan, is_builtin: true,
            },
            DhatuRoot {
                name: "vṛt".into(), devanagari: "वृत्".into(),
                meaning: "Turning / Converting".into(),
                computational_meaning: "Transform, map, convert type".into(),
                shiva_state: ShivaState::Transform, gana: DhatuGana::Parivartan, is_builtin: true,
            },
            DhatuRoot {
                name: "muc".into(), devanagari: "मुच्".into(),
                meaning: "Freeing / Releasing".into(),
                computational_meaning: "Release, delete, free, remove".into(),
                shiva_state: ShivaState::Dissolution, gana: DhatuGana::Parivartan, is_builtin: true,
            },
            DhatuRoot {
                name: "rūp".into(), devanagari: "रूप्".into(),
                meaning: "Forming / Casting".into(),
                computational_meaning: "Cast type, parse data, morph object".into(),
                shiva_state: ShivaState::Transform, gana: DhatuGana::Parivartan, is_builtin: true,
            },
            DhatuRoot {
                name: "saṁskṛ".into(), devanagari: "संस्कृ".into(),
                meaning: "Refining / Polishing".into(),
                computational_meaning: "Refine, optimize, trim, clean data".into(),
                shiva_state: ShivaState::Transform, gana: DhatuGana::Parivartan, is_builtin: true,
            },
            DhatuRoot {
                name: "viśleṣ".into(), devanagari: "विश्लेष्".into(),
                meaning: "Dismantling / Analyzing".into(),
                computational_meaning: "Dismantle, decompose, analyze, tear down".into(),
                shiva_state: ShivaState::Dissolution, gana: DhatuGana::Parivartan, is_builtin: true,
            },

            // ═══ Sanrachana (Structure) ═══
            DhatuRoot {
                name: "yuj".into(), devanagari: "युज्".into(),
                meaning: "Joining / Merging".into(),
                computational_meaning: "Join, merge, concatenate, union".into(),
                shiva_state: ShivaState::Structure, gana: DhatuGana::Sanrachana, is_builtin: true,
            },
            DhatuRoot {
                name: "bhid".into(), devanagari: "भिद्".into(),
                meaning: "Splitting / Partition".into(),
                computational_meaning: "Split, divide, partition, destructure".into(),
                shiva_state: ShivaState::Structure, gana: DhatuGana::Sanrachana, is_builtin: true,
            },
            DhatuRoot {
                name: "kram".into(), devanagari: "क्रम्".into(),
                meaning: "Sequencing / Sorting".into(),
                computational_meaning: "Sort, order, sequence, arrange".into(),
                shiva_state: ShivaState::Structure, gana: DhatuGana::Sanrachana, is_builtin: true,
            },
            DhatuRoot {
                name: "nirdeś".into(), devanagari: "निर्देश्".into(),
                meaning: "Pointing / Indexing".into(),
                computational_meaning: "Index array, pointer reference, mapping".into(),
                shiva_state: ShivaState::Structure, gana: DhatuGana::Sanrachana, is_builtin: true,
            },
            DhatuRoot {
                name: "varg".into(), devanagari: "वर्ग".into(),
                meaning: "Classifying / Grouping".into(),
                computational_meaning: "Group by, classify, categorize".into(),
                shiva_state: ShivaState::Structure, gana: DhatuGana::Sanrachana, is_builtin: true,
            },
            DhatuRoot {
                name: "stambh".into(), devanagari: "स्तम्भ्".into(),
                meaning: "Anchoring / Column".into(),
                computational_meaning: "Anchor layout, pin window, columnize".into(),
                shiva_state: ShivaState::Structure, gana: DhatuGana::Sanrachana, is_builtin: true,
            },

            // ═══ Raksha (Protection) ═══
            DhatuRoot {
                name: "rakṣ".into(), devanagari: "रक्ष्".into(),
                meaning: "Guarding / Validating".into(),
                computational_meaning: "Guard, protect, validate, authenticate".into(),
                shiva_state: ShivaState::Protection, gana: DhatuGana::Raksha, is_builtin: true,
            },
            DhatuRoot {
                name: "parīkṣ".into(), devanagari: "परीक्ष्".into(),
                meaning: "Testing / Verifying".into(),
                computational_meaning: "Test, assert, verify, check condition".into(),
                shiva_state: ShivaState::Protection, gana: DhatuGana::Raksha, is_builtin: true,
            },
            DhatuRoot {
                name: "pat".into(), devanagari: "पत्".into(),
                meaning: "Falling / Faulting".into(),
                computational_meaning: "Error, exception, fault, crash".into(),
                shiva_state: ShivaState::Force, gana: DhatuGana::Raksha, is_builtin: true,
            },
            DhatuRoot {
                name: "śuddh".into(), devanagari: "शुध्".into(),
                meaning: "Purifying / Sanitizing".into(),
                computational_meaning: "Sanitize input, pure function, zero out".into(),
                shiva_state: ShivaState::Protection, gana: DhatuGana::Raksha, is_builtin: true,
            },
            DhatuRoot {
                name: "āvaraṇ".into(), devanagari: "आवरण".into(),
                meaning: "Covering / Encrypting".into(),
                computational_meaning: "Encrypt, mask, hide, encapsulate".into(),
                shiva_state: ShivaState::Protection, gana: DhatuGana::Raksha, is_builtin: true,
            },
            DhatuRoot {
                name: "mudrā".into(), devanagari: "मुद्रा".into(),
                meaning: "Sealing / Signing".into(),
                computational_meaning: "Sign cryptographically, digital signature, seal".into(),
                shiva_state: ShivaState::Protection, gana: DhatuGana::Raksha, is_builtin: true,
            },

            // ═══ Ganana (Math & Measurement) ═══
            DhatuRoot {
                name: "gaṇ".into(), devanagari: "गण्".into(),
                meaning: "Counting / Computation".into(),
                computational_meaning: "Count, compute, calculate, mathematical operation".into(),
                shiva_state: ShivaState::Expansion, gana: DhatuGana::Ganana, is_builtin: true,
            },
            DhatuRoot {
                name: "mā".into(), devanagari: "मा".into(),
                meaning: "Measuring / Sizing".into(),
                computational_meaning: "Measure, benchmark, profile, sizeof".into(),
                shiva_state: ShivaState::Expansion, gana: DhatuGana::Ganana, is_builtin: true,
            },
            DhatuRoot {
                name: "kalana".into(), devanagari: "कलन".into(),
                meaning: "Calculus / Differentiating".into(),
                computational_meaning: "Differentiate, rate of change, delta".into(),
                shiva_state: ShivaState::Expansion, gana: DhatuGana::Ganana, is_builtin: true,
            },
            DhatuRoot {
                name: "samākalana".into(), devanagari: "समाकलन".into(),
                meaning: "Integration / Accumulating".into(),
                computational_meaning: "Integrate, sum over, reduce".into(),
                shiva_state: ShivaState::Expansion, gana: DhatuGana::Ganana, is_builtin: true,
            },
            DhatuRoot {
                name: "mūla".into(), devanagari: "मूल".into(),
                meaning: "Root / Base".into(),
                computational_meaning: "Square root, base index, foundational value".into(),
                shiva_state: ShivaState::Expansion, gana: DhatuGana::Ganana, is_builtin: true,
            },
            DhatuRoot {
                name: "as".into(), devanagari: "अस्".into(),
                meaning: "Being / Equality".into(),
                computational_meaning: "Is, equals, type check, state query".into(),
                shiva_state: ShivaState::Harmony, gana: DhatuGana::Jnana, is_builtin: true,
            },

            // ═══ Time & Bridge Adapters ═══
            DhatuRoot {
                name: "setu".into(), devanagari: "सेतु".into(),
                meaning: "Bridge / Adapter".into(),
                computational_meaning: "Bridge, cross-chain transfer, EVM adapter, data translation".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "kāla".into(), devanagari: "काल".into(),
                meaning: "Time / Timestamp".into(),
                computational_meaning: "Unix Timestamp, cosmic time, block time".into(),
                shiva_state: ShivaState::Flow, gana: DhatuGana::Ganana, is_builtin: true,
            },
            DhatuRoot {
                name: "nakṣatra".into(), devanagari: "नक्षत्र".into(),
                meaning: "Lunar Mansions / Astrolabe".into(),
                computational_meaning: "Cosmic resonance, Nakshatra frequency, mining constraint".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Jnana, is_builtin: true,
            },
            DhatuRoot {
                name: "nāda".into(), devanagari: "नाद".into(),
                meaning: "Sound / Resonance".into(),
                computational_meaning: "Hash resonance, data frequency modulo 27".into(),
                shiva_state: ShivaState::Resonance, gana: DhatuGana::Ganana, is_builtin: true,
            },

            // ═══ Purification ═══
            DhatuRoot {
                name: "śudh".into(), devanagari: "शुद्ध्".into(),
                meaning: "Purification / Filter".into(),
                computational_meaning: "Filter, purify, sanitize, clean data".into(),
                shiva_state: ShivaState::Purification, gana: DhatuGana::Parivartan, is_builtin: true,
            },
            DhatuRoot {
                name: "chid".into(), devanagari: "छिद्".into(),
                meaning: "Cutting / Truncation".into(),
                computational_meaning: "Trim, cut, truncate, reject".into(),
                shiva_state: ShivaState::Purification, gana: DhatuGana::Parivartan, is_builtin: true,
            },
        ];

        for root in roots {
            self.roots.insert(root.name.clone(), root);
        }
    }

    /// Find root by romanized name
    pub fn lookup(&self, name: &str) -> Option<&DhatuRoot> {
        self.roots.get(name)
    }

    /// Find root by Devanagari representation
    pub fn lookup_devanagari(&self, devanagari: &str) -> Option<&DhatuRoot> {
        self.roots.values().find(|r| r.devanagari == devanagari)
    }

    /// Get all roots under specific Gana
    pub fn by_gana(&self, gana: &DhatuGana) -> Vec<&DhatuRoot> {
        self.roots.values().filter(|r| &r.gana == gana).collect()
    }

    /// Get all roots under specific ShivaState
    pub fn by_shiva_state(&self, state: ShivaState) -> Vec<&DhatuRoot> {
        self.roots.values().filter(|r| r.shiva_state == state).collect()
    }

    /// Get total count of registered roots
    pub fn count(&self) -> usize {
        self.roots.len()
    }

    /// Get all root names
    pub fn all_names(&self) -> Vec<&str> {
        self.roots.keys().map(|s| s.as_str()).collect()
    }
}

impl Default for DhatuRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_has_roots() {
        let reg = DhatuRegistry::new();
        assert!(reg.count() >= 25, "Should have at least 25 computational roots");
    }

    #[test]
    fn test_lookup_vac() {
        let reg = DhatuRegistry::new();
        let vac = reg.lookup("vac").unwrap();
        assert_eq!(vac.devanagari, "वच्");
        assert_eq!(vac.shiva_state, ShivaState::Resonance);
    }

    #[test]
    fn test_by_gana() {
        let reg = DhatuRegistry::new();
        let creation = reg.by_gana(&DhatuGana::Srishti);
        assert!(creation.len() >= 3);
    }
}
