pub mod common;

mod optimizer;

mod errors;
pub mod expr;

pub mod sql;

use crate::sql::{builder, parser};
use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn internal(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<sql::logical::PyLogicalPlan>()?;
    m.add_class::<parser::PyContextProvider>()?;
    m.add_class::<optimizer::PyOptimizerContext>()?;

    // Register `common` as a submodule. Matching `datafusion-common` https://docs.rs/datafusion-common/latest/datafusion_common/
    let common = PyModule::new(py, "common")?;
    common::init_module(&common)?;
    m.add_submodule(&common)?;

    // Register `expr` as a submodule. Matching `datafusion-expr` https://docs.rs/datafusion-expr/latest/datafusion_expr/
    let expr = PyModule::new(py, "expr")?;
    expr::init_module(&expr)?;
    m.add_submodule(&expr)?;

    let parser = PyModule::new(py, "parser")?;
    parser::init_module(&parser)?;
    m.add_submodule(&parser)?;

    let optimizer = PyModule::new(py, "optimizer")?;
    optimizer::init_module(&optimizer)?;
    m.add_submodule(&optimizer)?;

    let builder = PyModule::new(py, "builder")?;
    builder::init_module(&builder)?;
    m.add_submodule(&builder)?;

    Ok(())
}
