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

    pub fn get_constants(&mut self) -> &ConstantMap<'a> {
        self.constants.get_or_insert(ConstantMapBuilder::build(
            self.allocator,
            self.contract_analysis.clarity_version,
            self.contract_analysis,
            self.annotations,
        ))
    }

    pub fn get_bindings(&mut self) -> &BindingMap<'a> {
        self.bindings.get_or_insert(BindingMapBuilder::build(
            self.allocator,
            self.contract_analysis.clarity_version,
            self.contract_analysis,
            self.annotations,
        ))
    }
}
