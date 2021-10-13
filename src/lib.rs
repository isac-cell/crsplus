use pyo3::prelude::*;

/// Says "Hello".
#[pyfunction]
fn hello() -> PyResult<String> {
    Ok(("Hello").to_string())
}

/// Says "Hi, [name]".
#[pyfunction]
fn hi(name: &str) -> PyResult<String> {
    Ok(("Hi ".to_owned() + name).to_string())
}

#[pymodule]
fn crsplus(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_function(wrap_pyfunction!(hello, m)?)?;
    m.add_function(wrap_pyfunction!(hi, m)?)?;

    Ok(())
}
