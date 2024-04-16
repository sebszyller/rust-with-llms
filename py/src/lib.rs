use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn is_balanced(text: String, c_opening: char, c_closing: char) -> bool {
    let mut balance: u128 = 0;
    for cursor in text.chars() {
        if cursor == c_closing {
            if balance == 0 {
                return false;
            } else {
                balance -= 1;
            }
        }
        if cursor == c_opening {
            balance += 1;
        }
    }
    balance == 0
}

#[pymodule]
fn py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_balanced, m)?)?;
    Ok(())
}
