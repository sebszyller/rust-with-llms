use std::collections::HashMap;

use pyo3::prelude::*;

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> String {
    (a + b).to_string()
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

#[pyfunction]
fn dict_and(
    ha: HashMap<String, Vec<String>>,
    hb: HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    ha.iter()
        .filter_map(|(ka, va)| match hb.get(ka) {
            Some(vb) => Some((ka.to_owned(), vec_intersection(va, vb))),
            None => None,
        })
        .collect::<HashMap<String, Vec<String>>>()
}

fn vec_intersection(va: &Vec<String>, vb: &Vec<String>) -> Vec<String> {
    va.iter().filter(|a| vb.contains(a)).cloned().collect()
}

#[pymodule]
fn py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(is_balanced, m)?)?;
    m.add_function(wrap_pyfunction!(dict_and, m)?)?;
    Ok(())
}
