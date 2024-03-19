use std::collections::BTreeMap;

use thiserror::Error;

use crate::{expr::Expr, termlist::TermList};

#[derive(Debug)]
pub struct StandardForm {
    pub terms: BTreeMap<char, f64>,
    pub constant: f64,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("The expr is not in standard form.")]
    NotStandardForm,
    #[error("Expected equation")]
    NotEquation,
}

impl TryFrom<Expr> for StandardForm {
    type Error = Error;

    fn try_from(value: Expr) -> Result<Self, Self::Error> {
        let (lhs, rhs) = if let Expr::Equation(lhs, rhs) = value {
            (lhs, rhs)
        } else {
            return Err(Error::NotEquation);
        };

        let lhs = TermList::from_expr(*lhs);

        let constant = {
            let rhs = TermList::from_expr(*rhs);
            if rhs.terms.len() != 1 {
                dbg!(&rhs);
                return Err(Error::NotStandardForm);
            }

            let rhs = rhs.terms.first().unwrap();
            if !rhs.pronumerals.is_empty() {
                dbg!();
                return Err(Error::NotStandardForm);
            }

            rhs.get_approximate_coefficient()
        };

        let mut variables = BTreeMap::<char, f64>::new();

        for term in lhs.terms {
            if term.pronumerals.len() != 1 {
                dbg!();
                return Err(Error::NotStandardForm);
            }
            let pronumeral = term.pronumerals.first().unwrap();
            if variables.contains_key(pronumeral) {
                dbg!();
                return Err(Error::NotStandardForm);
            }
            variables.insert(*pronumeral, term.get_approximate_coefficient());
        }

        // sort by pronumeral
        let mut variables = variables.into_iter().collect::<Vec<_>>();
        variables.sort_by(|(a, _), (b, _)| a.cmp(b));
        let variables = BTreeMap::from_iter(variables);

        Ok(StandardForm {
            terms: variables,
            constant,
        })
    }
}
