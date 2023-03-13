use diffy::{create_patch, PatchFormatter};
use pyo3::prelude::*;
use std::fs;

#[pyfunction]
pub fn fibonacci(n: i32) -> u128 {
    let (mut a, mut b): (u128, u128) = (0, 1);
    for _ in 0..n {
        (a, b) = (b, a + b);
    }
    a
}

#[pyfunction]
pub fn diffy_unified_diff(f1: &str, f2: &str) {
    let contents1: String = fs::read_to_string(f1).expect("first file doesn't exist");
    let contents2: String = fs::read_to_string(f2).expect("second file doesn't exist");

    let patch = create_patch(&contents1, &contents2);
    let f = PatchFormatter::new().with_color();
    print!("{}", f.fmt_patch(&patch));
}

#[pymodule]
pub fn _rust_lib(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    m.add_function(wrap_pyfunction!(diffy_unified_diff, m)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_success() {
        let result = fibonacci(10);
        assert_eq!(result, 55);
    }
}
