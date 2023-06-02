use pyo3::prelude::*;

mod machine;

/// A Python module implemented in Rust.
#[pymodule]
fn turing_machine_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<machine::State>()?;
    m.add_class::<machine::Moves>()?;
    m.add_class::<machine::Tuple>()?;
    m.add_class::<machine::Tape>()?;
    m.add_class::<machine::TuringMachine>()?;

    m.add_function(wrap_pyfunction!(machine::create_states, m)?)?;
    m.add_function(wrap_pyfunction!(machine::create_final_states, m)?)?;
    m.add_function(wrap_pyfunction!(machine::create_tuple, m)?)?;
    m.add_function(wrap_pyfunction!(machine::load_tape_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(machine::load_from_instance, m)?)?;

    Ok(())
}
