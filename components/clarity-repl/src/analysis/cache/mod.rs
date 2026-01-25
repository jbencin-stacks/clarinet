//! Data structures used in multiple lints/analysis passes

use clarity::vm::analysis::ContractAnalysis;
use oxc_allocator::Allocator;

pub mod bindings;
pub mod constants;

use bindings::{BindingMap, BindingMapBuilder};
use constants::{ConstantMap, ConstantMapBuilder};

use crate::analysis::annotation::Annotation;

/// Container struct for all cached itemss
/// All fields are lazy-initialized, and only created if used in at least one pass
pub struct AnalysisCache<'a> {
    pub contract_analysis: &'a ContractAnalysis,
    pub annotations: &'a Vec<Annotation>,

    /// For data structures which we build from contract source code, we can use an allocator which is faster then the global allocator
    ///
    /// The assumptions we are making for this allocator...
    ///  - We don't need to free items, so we can use a bump allocator for fast allocation
    ///  - DoS resistance doesn't matter here, so we don't need a cryptographic hash function
    allocator: &'a Allocator,
    constants: Option<ConstantMap<'a>>,
    bindings: Option<BindingMap<'a>>,
}

impl<'a> AnalysisCache<'a> {
    pub fn new(
        allocator: &'a Allocator,
        contract_analysis: &'a ContractAnalysis,
        annotations: &'a Vec<Annotation>,
    ) -> Self {
        Self {
            allocator,
            contract_analysis,
            annotations,
            constants: None,
            bindings: None,
        }
    }

    /// Get allocator used by cache
    pub fn get_allocator(&self) -> &Allocator {
        self.allocator
    }

    /// Get map of constants defined in contract
    pub fn get_constants(&mut self) -> &ConstantMap<'a> {
        self.constants.get_or_insert(ConstantMapBuilder::build(
            self.allocator,
            self.contract_analysis.clarity_version,
            self.contract_analysis,
            self.annotations,
        ))
    }

    /// Get map of `let` bindings and function args defined in contract
    pub fn get_bindings(&mut self) -> &BindingMap<'a> {
        self.bindings.get_or_insert(BindingMapBuilder::build(
            self.allocator,
            self.contract_analysis.clarity_version,
            self.contract_analysis,
            self.annotations,
        ))
    }
}
