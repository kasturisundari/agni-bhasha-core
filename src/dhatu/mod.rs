#![allow(dead_code)]

pub mod roots;
pub mod pratyaya;
pub mod derivation;
pub mod extended_roots;
pub mod sandhi;

pub use roots::{DhatuRoot, DhatuRegistry, DhatuGana};
pub use pratyaya::{PratyayaSuffix, PratyayaEffect, PratyayaRegistry};
pub use derivation::{DerivedForm, DerivationEngine};
pub use sandhi::apply_sandhi;
