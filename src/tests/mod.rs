#[macro_use]
pub(crate) mod common {
    use crate as pyo3;
    include!("./common.rs");
}

/// Test macro hygiene - this is in the crate since we won't have
/// `pyo3` available in the crate root.
#[cfg(all(test, feature = "macros"))]
mod hygiene;
