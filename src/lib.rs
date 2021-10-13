use pyo3::prelude::*;

#[pyfunction]
fn hello() -> PyResult<String> {
    Ok(("Hello").to_string())
}

#[pyfunction]
fn hi(name: &str) -> PyResult<String> {
    Ok(("Hi ".to_owned() + name).to_string())
}
#[pymodule]
fn crsplus(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(hi, m)?)?;

    Ok(())
}
