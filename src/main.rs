use std::{collections::VecDeque, io::Write};

use ariadne::Source;
use clap::{Parser, Subcommand};
use color_eyre::{
    eyre::{bail, Context},
    Result,
};
use logos::Logos;

use sles::{
    expr::{parse, Expr},
    matrix::MatrixForm,
    solve::{self, Method},
    standardform::StandardForm,
    termlist::TermList,
    token::Token,
};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(subcommand)]
    subcommand: Command,
}
#[derive(Subcommand, Debug)]
enum Command {
    Quit,
    Solve {
        #[clap(long, default_value = "matrix")]
        method: Method,
    },
    Terms,
    File,
}

struct Repl {
    exprs: Vec<Expr>,
}
impl Repl {
    fn run(&mut self, input: String) -> Result<()> {
        let mut args = input.trim().split_whitespace().collect::<VecDeque<_>>();
        args.push_front("placeholder");
        let cli = Cli::try_parse_from(args);
        if let Ok(cli) = cli {
            match cli.subcommand {
                // ":help" => {
                //     println!("Enter any equation in standard form (ax + by = c)");
                //     println!("When you are ready to solve, type :solve");
                //     println!("Note, you must have the same number of equations entered as the number of pronumerals, otherwise it is unsolvable");
                //     println!();
                //     println!("Commands:");
                //     println!("  :help  - Print this help");
                //     println!("  :quit  - Quit the program");
                //     println!("  :solve - Solve the system of equations");
                //     println!("  :terms - Print the terms of the system of equations");
                //     println!("  :file  - Solve the system of equations from a file");
                //     println!("           (equation.txt)");
                // }
                Command::Quit => {
                    std::process::exit(0);
                }
                Command::Solve { method } => {
                    {
                        if self.exprs.is_empty() {
                            bail!("No equations to solve.");
                        };

                        let exprs = std::mem::replace(&mut self.exprs, Vec::new());

                        let solution = solve::solve_with_method(exprs, method)?;

                        for (var, val) in solution {
                            println!("{var} = {val}");
                        }
                    }

                    self.exprs = Vec::new();
                }
                Command::File => {
                    let input = std::fs::read_to_string("equation.txt")
                        .wrap_err("Failed to read equation.txt")?;
                    let lines = input.lines();
                    let equations = lines
                        .map(|line| {
                            let tokens = Token::lexer(line);

                            let expr = match parse(tokens) {
                                Ok(expr) => expr,
                                Err(reports) => {
                                    for report in reports {
                                        let source = Source::from(&input);
                                        report.eprint(source).wrap_err(
                                            "Failed to write error to stdout (double error!)",
                                        )?;
                                    }
                                    bail!("Failed to parse");
                                }
                            };
                            Ok(expr)
                        })
                        .collect::<Result<Vec<_>, _>>()?;

                    let standard_eqs = equations
                        .into_iter()
                        .map(|expr| StandardForm::try_from(expr))
                        .collect::<Result<Vec<_>, _>>()?;

                    let matrix = MatrixForm::try_from(standard_eqs)?;
                    let solutions = matrix.solve();

                    for (var, val) in solutions {
                        println!("{var} = {val}");
                    }
                }
                Command::Terms => {
                    if self.exprs.is_empty() {
                        bail!("No equations to solve.");
                    };

                    let exprs = std::mem::replace(&mut self.exprs, Vec::new());
                    for expr in exprs {
                        let Expr::Equation(lhs, rhs) = expr else {
                            unreachable!("expr is guaranteed to be an equation by parser")
                        };

                        let lhs = TermList::from_expr(*lhs);
                        let rhs = TermList::from_expr(*rhs);
                        println!("{lhs:?} = {rhs:?}");
                    }

                    self.exprs = Vec::new();
                }
            }
        } else {
            let tokens = Token::lexer(&input);

            let expr = match parse(tokens) {
                Ok(expr) => expr,
                Err(reports) => {
                    for report in reports {
                        let source = Source::from(&input);
                        report.eprint(source)?;
                    }
                    bail!("Failed to parse");
                }
            };
            self.exprs.push(expr);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    println!("System of Linear Equations Solver");
    println!("Type :help to learn more.");

    let stdin = std::io::stdin();

    let mut repl = Repl { exprs: Vec::new() };

    loop {
        print!("> ");

        std::io::stdout().flush()?;

        let mut input = String::new();
        stdin.read_line(&mut input)?;

        if let Err(err) = repl.run(input) {
            eprintln!("{err}");
        }
    }
}
