use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn print_name(name: String) -> PyResult<String> {
    let user_name = format!("Hello {}",name);
    Ok(user_name.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pytnonServerTest(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_name, m)?)?;
    Ok(())
}
