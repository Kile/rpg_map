use pyo3::prelude::*;

mod structs;

#[pymodule]
fn rpg_map(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<structs::map::Map>()?;
    m.add_class::<structs::map::MapType>()?;
    m.add_class::<structs::map::PathStyle>()?;
    m.add_class::<structs::travel::Travel>()?;
    m.add_class::<structs::map::PathDisplayType>()?;
    m.add_class::<structs::path::PathPoint>()?;
    m.add_class::<structs::map::PathProgressDisplayType>()?;

    Ok(())
}
