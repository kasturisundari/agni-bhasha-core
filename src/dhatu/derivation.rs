/// #   — Derivation Engine
///
///       .

use super::roots::{DhatuRoot, DhatuRegistry};
use super::pratyaya::{PratyayaSuffix, PratyayaEffect, PratyayaRegistry};

///   =  + 
#[derive(Debug, Clone)]
pub struct DerivedForm {
    pub root: DhatuRoot,
    pub suffix: PratyayaSuffix,
}

impl DerivedForm {
    ///    
    pub fn full_name(&self) -> String {
        format!("√{}+{}", self.root.name, self.suffix.name)
    }

    ///     
    pub fn is_async(&self) -> bool {
        self.suffix.computational_effect == PratyayaEffect::AsyncExecute
    }

    ///     
    pub fn is_collection(&self) -> bool {
        self.suffix.computational_effect == PratyayaEffect::Collection
    }
}

///  
#[derive(Clone)]
pub struct DerivationEngine {
    pub dhatu_registry: DhatuRegistry,
    pub pratyaya_registry: PratyayaRegistry,
}

impl DerivationEngine {
    pub fn new() -> Self {
        Self {
            dhatu_registry: DhatuRegistry::new(),
            pratyaya_registry: PratyayaRegistry::new(),
        }
    }

    ///     + 
    pub fn derive(&self, root_name: &str, suffix_name: &str) -> Option<DerivedForm> {
        let root = self.dhatu_registry.lookup(root_name)?;
        let suffix = self.pratyaya_registry.lookup(suffix_name)?;
        Some(DerivedForm {
            root: root.clone(),
            suffix: suffix.clone(),
        })
    }
}

impl Default for DerivationEngine {
    fn default() -> Self { Self::new() }
}
