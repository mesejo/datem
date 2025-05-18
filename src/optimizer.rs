use std::sync::Arc;

use datafusion_expr::LogicalPlan;
use datafusion_optimizer::optimizer::Optimizer;
use datafusion_optimizer::{OptimizerContext, OptimizerRule};
use pyo3::prelude::PyModule;
use pyo3::prelude::*;
use pyo3::{pyclass, pyfunction, pymethods, wrap_pyfunction, PyResult};

use crate::sql::logical::PyLogicalPlan;

#[pyclass(name = "OptimizerContext", module = "let", subclass)]
#[derive(Clone, Default)]
pub struct PyOptimizerContext {
    pub(crate) context: Arc<OptimizerContext>,
}

#[pymethods]
impl PyOptimizerContext {
    #[pyo3(signature = ())]
    #[new]
    pub fn new() -> Self {
        Self {
            context: Arc::new(OptimizerContext::default()),
        }
    }
}

fn observe(_plan: &LogicalPlan, _rule: &dyn OptimizerRule) {}

#[pyfunction]
pub fn optimize_plan(plan: PyLogicalPlan, context_provider: PyOptimizerContext) -> PyLogicalPlan {
    let optimizer = Optimizer::new();
    let plan = plan.plan.as_ref().clone();
    let optimized_plan = optimizer
        .optimize(plan, context_provider.context.as_ref(), observe)
        .unwrap();
    PyLogicalPlan::from(optimized_plan)
}

pub(crate) fn init_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(optimize_plan))?;
    Ok(())
}
