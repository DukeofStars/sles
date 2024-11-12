use std::collections::BTreeMap;

use thiserror::Error;

use crate::{expr::Expr, matrix::MatrixForm, standardform::StandardForm};

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to convert the equation from an expression to standard form.")]
    ExprToStandardFormConversionFail(#[source] crate::standardform::Error),
    #[error("Failed to convert the equation from standard form to matrix form.")]
    StandardFormToMatrixFormConversionFail(#[source] crate::matrix::Error),
}
pub type Result<T> = core::result::Result<T, Error>;

pub type Solution = BTreeMap<char, f64>;

#[derive(Debug, Clone, clap::ValueEnum)]
pub enum Method {
    Matrix,
    General,
}

pub fn solve_with_method(exprs: Vec<Expr>, method: Method) -> Result<Solution> {
    match method {
        Method::Matrix => {
            let standard_eqs = exprs
                .into_iter()
                .map(StandardForm::try_from)
                .map(|r| r.map_err(Error::ExprToStandardFormConversionFail))
                .collect::<Result<Vec<_>>>()?;

            let matrix = MatrixForm::try_from(standard_eqs)
                .map_err(Error::StandardFormToMatrixFormConversionFail)?;
            Ok(matrix.solve())
        }
        Method::General => {
            todo!()
        }
    }
}
