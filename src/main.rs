use std::io::Write;

use ariadne::Source;
use color_eyre::Result;
use logos::Logos;

use sles::expr::{parse, Expr};
use sles::matrix::MatrixForm;
use sles::standardform::StandardForm;
use sles::termlist::TermList;
use sles::token::Token;

fn main() -> Result<()> {
    println!("System of Linear Equations Solver");
    println!("Type :help to learn more.");

    let mut input = String::new();

    let mut exprs = Some(Vec::new());

    'rwloop: loop {
        input.clear();

        print!("> ");

        std::io::stdout().flush()?;

        let stdin = std::io::stdin();
        stdin.read_line(&mut input)?;

        match input.trim() {
            ":help" => {
                println!("Enter any equation in standard form (ax + by = c)");
                println!("When you are ready to solve, type :solve");
                println!("Note, you must have the same number of equations entered as the number of pronumerals, otherwise it is unsolvable");
                println!();
                println!("Commands:");
                println!("  :help  - Print this help");
                println!("  :quit  - Quit the program");
                println!("  :solve - Solve the system of equations");
                println!("  :terms - Print the terms of the system of equations");
                println!("  :file  - Solve the system of equations from a file");
                println!("           (equation.txt)");
            }
            ":quit" => {
                break 'rwloop;
            }
            ":solve" => {
                let standard_eqs = exprs
                    .take()
                    .map(|exprs| exprs.into_iter().map(|expr| StandardForm::try_from(expr)))
                    .unwrap()
                    .flatten()
                    .collect::<Vec<_>>();

                let matrix = MatrixForm::from(standard_eqs);
                let solutions = matrix.solve();

                for (var, val) in solutions {
                    println!("{var} = {val}");
                }

                exprs = Some(Vec::new());
            }
            ":file" => {
                let input = std::fs::read_to_string("equation.txt").unwrap();
                let lines = input.lines();
                let equations = lines
                    .map(|line| {
                        let source = Source::from(&input);
                        let tokens = Token::lexer(&line);

                        let expr = match parse(tokens) {
                            Ok(expr) => expr,
                            Err(report) => {
                                report.eprint(source).unwrap();
                                std::process::exit(1);
                            }
                        };
                        expr
                    })
                    .collect::<Vec<_>>();

                let standard_eqs = equations
                    .into_iter()
                    .map(|expr| StandardForm::try_from(expr).unwrap())
                    .collect::<Vec<_>>();

                let matrix = MatrixForm::from(standard_eqs);
                let solutions = matrix.solve();

                for (var, val) in solutions {
                    println!("{var} = {val}");
                }
            }
            ":terms" => {
                for expr in exprs.take().unwrap() {
                    let (lhs, rhs) = if let Expr::Equation(lhs, rhs) = expr {
                        (lhs, rhs)
                    } else {
                        panic!()
                    };

                    let lhs = TermList::from_expr(*lhs);
                    let rhs = TermList::from_expr(*rhs);
                    println!("{lhs:?} = {rhs:?}");
                }

                exprs = Some(Vec::new());
            }
            _ => {
                let source = Source::from(&input);
                let tokens = Token::lexer(&input);

                let expr = match parse(tokens) {
                    Ok(expr) => expr,
                    Err(report) => {
                        report.eprint(source)?;
                        continue;
                    }
                };
                if let Some(exprs) = &mut exprs {
                    exprs.push(expr);
                }
            }
        }
    }
    Ok(())
}
