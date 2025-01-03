use rustc_errors::IntoDiagArg;
use rustc_macros::{Diagnostic, LintDiagnostic};
use rustc_middle::ty::{self, Ty};
use rustc_span::Span;

pub struct Mutability(ty::Mutability);

impl From<ty::Mutability> for Mutability {
    fn from(mutability: ty::Mutability) -> Self {
        Self(mutability)
    }
}

impl IntoDiagArg for Mutability {
    fn into_diag_arg(self) -> rustc_errors::DiagArgValue {
        self.0.prefix_str().into_diag_arg()
    }
}

#[derive(Diagnostic)]
#[diag(rpl_patterns_unsound_slice_cast)]
pub struct UnsoundSliceCast<'tcx> {
    #[note]
    pub cast_from: Span,
    #[primary_span]
    pub cast_to: Span,
    pub ty: Ty<'tcx>,
    pub mutability: Mutability,
}

#[derive(Diagnostic)]
#[diag(rpl_patterns_use_after_drop)]
pub struct UseAfterDrop<'tcx> {
    #[note]
    pub drop_span: Span,
    #[primary_span]
    pub use_span: Span,
    pub ty: Ty<'tcx>,
}

#[derive(Diagnostic)]
#[diag(rpl_patterns_offset_by_one)]
pub struct OffsetByOne {
    #[primary_span]
    #[label(rpl_patterns_read_label)]
    pub read: Span,
    #[label(rpl_patterns_ptr_label)]
    pub ptr: Span,
    #[help]
    #[suggestion(code = "({len_local} - 1)")]
    pub len: Span,
    pub len_local: String,
}

// for cve_2018_21000
#[derive(Diagnostic)]
#[diag(rpl_patterns_misordered_parameters)]
pub struct MisorderedParameters {
    #[help]
    #[primary_span]
    pub span: Span,
}

// for cve_2020_35881
#[derive(Diagnostic)]
#[diag(rpl_patterns_wrong_assumption_of_fat_pointer_layout)]
#[help]
pub struct WrongAssumptionOfFatPointerLayout {
    #[primary_span]
    #[label(rpl_patterns_ptr_transmute_label)]
    pub ptr_transmute: Span,
    #[label(rpl_patterns_get_data_ptr_label)]
    pub data_ptr_get: Span,
}

// for cve_2019_15548
#[derive(LintDiagnostic)]
#[diag(rpl_patterns_rust_str_as_c_str)]
#[help]
pub struct RustStrAsCStr {
    #[label(rpl_patterns_label)]
    pub cast_from: Span,
    #[note]
    pub cast_to: Span,
}

// another pattern for cve_2019_15548
#[derive(LintDiagnostic)]
#[diag(rpl_patterns_lengthless_buffer_passed_to_extern_function)]
pub struct LengthlessBufferPassedToExternFunction {
    #[label(rpl_patterns_label)]
    pub ptr: Span,
}

// for cve_2021_27376
#[derive(Diagnostic)]
#[diag(rpl_patterns_wrong_assumption_of_layout_compatibility)]
#[help]
pub struct WrongAssumptionOfLayoutCompatibility {
    #[label(rpl_patterns_label)]
    #[note]
    pub cast_from: Span,
    #[primary_span]
    pub cast_to: Span,
    pub type_from: &'static str,
    pub type_to: &'static str,
}
