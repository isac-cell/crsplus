use pyo3::prelude::*;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok(("Hello").to_string())
}

#[pymodule]
fn crsplus(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;

    Ok(())
}
