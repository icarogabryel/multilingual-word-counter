use std::collections::HashMap;
use pyo3::{prelude::*, IntoPyObjectExt};

#[pyfunction]
fn count_words(s: String) -> Result<Py<PyAny>, PyErr> {
    let mut hm = HashMap::new();

    for sub_str in s.split(' ') {
        let count = hm.entry(sub_str.to_string()).or_insert(0);
        *count += 1;
    }

    return Python::with_gil(|py| {
        hm.into_py_any(py)
    });
}

// A Python module implemented in Rust.
#[pymodule]
fn word_counter(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_words, m)?)?;
    Ok(())
}
