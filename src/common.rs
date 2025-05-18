use pyo3::prelude::*;

pub mod data_type;
pub mod df_schema;
pub mod function;
pub mod schema;

/// Initializes the `common` module to match the pattern of `datafusion-common` https://docs.rs/datafusion-common/18.0.0/datafusion_common/index.html
pub(crate) fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<df_schema::PyDFSchema>()?;
    m.add_class::<data_type::PyDataType>()?;
    m.add_class::<data_type::DataTypeMap>()?;
    m.add_class::<data_type::PythonType>()?;
    m.add_class::<data_type::SqlType>()?;
    m.add_class::<schema::SqlTable>()?;
    m.add_class::<schema::SqlSchema>()?;
    m.add_class::<schema::SqlView>()?;
    m.add_class::<schema::SqlStatistics>()?;
    m.add_class::<function::SqlFunction>()?;
    Ok(())
}
