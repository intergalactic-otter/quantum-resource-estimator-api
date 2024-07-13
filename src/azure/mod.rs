use pyo3::{types::{PyModule}, PyResult, Python};
use pyo3::prelude::*;


pub fn version(py: Python<'_>) -> PyResult<String> {
    let sys = PyModule::import_bound(py, "sys")?;
    let version: String = sys.getattr("version")?.extract()?;
    println!("Python version: {}", version);
    Ok(version)
}

