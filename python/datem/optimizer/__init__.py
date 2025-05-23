import ibis

from datem.internal import ContextProvider, OptimizerContext, optimizer
from datem.sql import parser
from datem.translate import plan_to_ibis


def optimize_sql(sql: str, catalog: dict, dialect: str = None) -> ibis.Expr:
    plan = parser.parse_sql(
        sql,
        ContextProvider({k: v.to_pyarrow() for k, v in catalog.items()}),
        dialect=dialect,
    )
    optimized_plan = optimizer.optimize_plan(plan, OptimizerContext())
    return plan_to_ibis(optimized_plan, catalog)


def optimize_ibis(expr: ibis.Expr, catalog: dict, dialect: str = None) -> ibis.Expr:
    sql = ibis.to_sql(expr, dialect=dialect)
    return optimize_sql(sql, catalog, dialect=dialect)
