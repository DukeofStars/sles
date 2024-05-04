use std::collections::BTreeMap;

use nalgebra::{DMatrix, MatrixXx1};
use thiserror::Error;

use crate::standardform::StandardForm;

#[derive(Debug, Error)]
pub enum Error {
    #[error("There are no equations to solve!")]
    NoEquations,
    #[error("There are not the same number of pronumerals as equations, this makes the equation unsolveable")]
    MismatchedPronumeralCount,
    #[error("The pronumeral {0} is found in equation 1, but not in equation {1}")]
    PronumeralNotCommon(char, usize),
}
pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct MatrixForm {
    coefficients: DMatrix<f64>,
    variables: MatrixXx1<char>,
    constants: MatrixXx1<f64>,
}

impl TryFrom<Vec<StandardForm>> for MatrixForm {
    type Error = Error;
    fn try_from(equations: Vec<StandardForm>) -> Result<Self> {
        if equations.is_empty() {
            return Err(Error::NoEquations);
        };

        let mut variables = Vec::new();
        let mut coefficients = Vec::new();
        let mut constants = Vec::new();

        for (variable, _) in &equations[0].terms {
            variables.push(*variable);
        }

        if equations.len() != variables.len() {
            return Err(Error::MismatchedPronumeralCount);
        };

        for variable in &variables {
            for (i, equation) in equations.iter().enumerate() {
                let coeff = equation
                    .terms
                    .get(variable)
                    .ok_or_else(|| Error::PronumeralNotCommon(*variable, i))?;
                coefficients.push(*coeff);
            }
        }
        for equation in &equations {
            constants.push(equation.constant);
        }

        let variables = MatrixXx1::from_vec(variables);
        let coefficients = DMatrix::from_vec(variables.len(), equations.len(), coefficients);
        let constants = MatrixXx1::from_vec(constants);

        Ok(MatrixForm {
            coefficients,
            variables,
            constants,
        })
    }
}

impl MatrixForm {
    pub fn solve(self) -> BTreeMap<char, f64> {
        if !self.coefficients.is_invertible() {
            println!("This system is not solvable. Approximating solution");
        }
        let inverse = self
            .coefficients
            .pseudo_inverse(0.000_000_000_01)
            .expect("Epsilon guaranteed to be non negative");

        let solution = inverse * self.constants;

        let mut solutions = BTreeMap::new();

        for (variable, value) in self.variables.into_iter().zip(solution.into_iter()) {
            solutions.insert(*variable, *value);
        }

        solutions
    }
}
