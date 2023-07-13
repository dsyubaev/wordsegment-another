pub mod corpus_loader;
pub mod segmentator;

use crate::corpus_loader::Corpus;
use crate::segmentator::segment;
use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn wordsegment_another(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<Corpus>()?;
    m.add_function(wrap_pyfunction!(segment, m)?)?;
    Ok(())
}
