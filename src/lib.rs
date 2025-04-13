use pyo3::prelude::*;
#[cfg(feature = "stubgen")]
use pyo3_stub_gen::define_stub_info_gatherer;

mod structs;
#[cfg(test)]
mod tests;

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

#[cfg(feature = "stubgen")]
define_stub_info_gatherer!(stub_info);
