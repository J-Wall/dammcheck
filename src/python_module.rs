use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use std::rc::Rc;
use std::str::FromStr;

use strum::VariantNames;

use super::{alphabets, quasigroups};

/// Calculates Damm's check digit, given an alphabet
#[pyfunction]
fn damm(s: &str, alphabet: Option<&str>, custom_alphabet: Option<&str>) -> PyResult<String> {
    let alpha: alphabets::Alphabet = match (alphabet, custom_alphabet) {
        (Some(a), None) => alphabets::Alphabets::from_str(a)
            .map_err(|_| {
                PyErr::new::<PyValueError, _>(format!(
                    "Unknown alphabet. Select one of {:?} or use 'custom_alphabet'.",
                    builtin_alphabets()
                ))
            })?
            .into(),
        (None, Some(a)) => alphabets::Alphabet::new(a.chars().collect::<Rc<[char]>>())
            .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))?,
        (None, None) => alphabets::Alphabets::Base10.into(),
        _ => Err(PyErr::new::<PyValueError, _>(
            "Only one of 'alphabet' or 'custom_alphabet' should be set.",
        ))?,
    };
    super::damm(s, alpha)
        .map(|c| c.to_string())
        .map_err(|e| PyErr::new::<PyValueError, _>(e.to_string()))
}

#[pyfunction]
fn builtin_alphabets() -> Vec<&'static str> {
    alphabets::Alphabets::VARIANTS.to_vec()
}

/// Applies a quasigroup operation in a given base
#[pyfunction]
fn apply_operation(a: usize, b: usize, base: usize) -> PyResult<u8> {
    let base: quasigroups::Order = base.try_into().map_err(|_| {
        PyErr::new::<PyValueError, _>(format!("No implementation exists for base {}", base))
    })?;
    Ok(quasigroups::apply(a, b, base))
}

/// A Python module implemented in Rust.
#[pymodule]
fn dammcheck(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(apply_operation, m)?)?;
    m.add_function(wrap_pyfunction!(damm, m)?)?;
    m.add_function(wrap_pyfunction!(builtin_alphabets, m)?)?;
    Ok(())
}
