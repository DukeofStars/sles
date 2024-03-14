use std::fmt::{Display, Formatter};

use crate::expr::{Constant, Expr, Op};

#[derive(Debug)]
pub struct TermList {
    pub(crate) terms: Vec<Term>,
}

impl Display for TermList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.terms
                .iter()
                .map(|t| t.to_string())
                .collect::<Vec<_>>()
                .join(" + ")
        )
    }
}

#[derive(Debug)]
pub struct Term {
    pub coeff: f64,
    pub pronumerals: Vec<char>,
    pub constants: Vec<Constant>,
}

impl Display for Term {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            self.coeff,
            self.pronumerals.iter().collect::<String>(),
            self.constants
                .iter()
                .map(|c| match c {
                    Constant::Pi => "π",
                    Constant::E => "e",
                })
                .collect::<String>()
        )
    }
}
impl Term {
    pub fn get_approximate_coefficient(&self) -> f64 {
        let mut num = self.coeff;
        for constant in &self.constants {
            num *= match constant {
                Constant::Pi => std::f64::consts::PI,
                Constant::E => std::f64::consts::E,
            };
        }
        num
    }
}

impl TermList {
    pub fn from_expr(expr: Expr) -> TermList {
        let terms = expr.terms();
        TermList::simplify(terms)
    }

    /// Simplifies a list of expr into a list of terms.
    /// Note that this does not handle all cases, and is best used for simple terms like 5x or (8+3)x
    pub fn simplify(terms: Vec<Expr>) -> TermList {
        fn simplify_inner(
            expr: &Expr,
            pronumerals: &mut Vec<char>,
            constants: &mut Vec<Constant>,
        ) -> f64 {
            match expr {
                Expr::BinOp(lhs, op, rhs) => {
                    let mut term1 = Term {
                        coeff: 1.0,
                        pronumerals: Vec::new(),
                        constants: Vec::new(),
                    };
                    let mut term2 = Term {
                        coeff: 1.0,
                        pronumerals: Vec::new(),
                        constants: Vec::new(),
                    };

                    term1.coeff = simplify_inner(lhs, &mut term1.pronumerals, &mut term1.constants);
                    term2.coeff = simplify_inner(rhs, &mut term2.pronumerals, &mut term2.constants);

                    pronumerals.append(&mut term1.pronumerals);
                    pronumerals.append(&mut term2.pronumerals);
                    constants.append(&mut term1.constants);
                    constants.append(&mut term2.constants);
                    match op {
                        Op::Mul => term1.coeff * term2.coeff,
                        Op::Div => term1.coeff / term2.coeff,
                        Op::Add => term1.coeff + term2.coeff,
                        Op::Sub => term1.coeff - term2.coeff,
                        Op::Pow => term1.coeff.powf(term2.coeff),
                    }
                }
                Expr::Num(num) => *num,
                Expr::Var(c) => {
                    pronumerals.push(*c);
                    1.0
                }
                Expr::Constant(constant) => {
                    constants.push(constant.clone());
                    1.0
                }
                Expr::Equation(_, _) => {
                    panic!("Cannot handle equation")
                }
            }
        }
        let mut new_terms = Vec::new();
        for term in &terms {
            let mut pronumerals = Vec::new();
            let mut constants = Vec::new();
            let coeff = simplify_inner(term, &mut pronumerals, &mut constants);

            new_terms.push(Term {
                coeff,
                pronumerals,
                constants,
            });
        }
        TermList { terms: new_terms }
    }
}
