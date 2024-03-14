use std::collections::BTreeMap;

use nalgebra::*;

use crate::standardform::StandardForm;

#[derive(Debug)]
pub struct MatrixForm {
    coefficients: DMatrix<f64>,
    variables: MatrixXx1<char>,
    constants: MatrixXx1<f64>,
}

impl From<Vec<StandardForm>> for MatrixForm {
    fn from(equations: Vec<StandardForm>) -> Self {
        if equations.is_empty() {
            panic!()
        }

        let mut variables = Vec::new();
        let mut coefficients = Vec::new();
        let mut constants = Vec::new();

        for (variable, _) in &equations[0].terms {
            variables.push(*variable);
        }

        if equations.len() != variables.len() {
            panic!();
        }

        for variable in &variables {
            for equation in &equations {
                let coeff = equation.terms.get(variable).unwrap();
                coefficients.push(*coeff);
            }
        }
        for equation in &equations {
            constants.push(equation.constant);
        }

        let variables = MatrixXx1::from_vec(variables);
        let coefficients = DMatrix::from_vec(variables.len(), equations.len(), coefficients);
        let constants = MatrixXx1::from_vec(constants);

        MatrixForm {
            coefficients,
            variables,
            constants,
        }
    }
}

impl MatrixForm {
    pub fn solve(self) -> BTreeMap<char, f64> {
        if !self.coefficients.is_invertible() {
            println!("This system is not solvable. Approximating solution");
        }
        let inverse = self.coefficients.pseudo_inverse(0.00000000001).unwrap();

        let solution = inverse * self.constants;

        let mut solutions = BTreeMap::new();

        for (variable, value) in self.variables.into_iter().zip(solution.into_iter()) {
            solutions.insert(*variable, *value);
        }

        solutions
    }
}
