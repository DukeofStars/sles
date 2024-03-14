use ariadne::{Label, Report, ReportKind};
use chumsky::prelude::*;
use chumsky::Stream;
use logos::Lexer;

use crate::token::{Float, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    BinOp(Box<Expr>, Op, Box<Expr>),
    Num(f64),
    Var(char),
    Constant(Constant),
    Equation(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Pi,
    E,
}

#[derive(Clone, Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Expr {
    pub fn terms(self) -> Vec<Expr> {
        let mut acc = Vec::new();
        fn terms_inner(expr: Expr, acc: &mut Vec<Expr>) {
            match expr {
                Expr::BinOp(lhs, Op::Add, rhs) => {
                    terms_inner(*lhs, acc);
                    terms_inner(*rhs, acc);
                }
                Expr::BinOp(lhs, Op::Sub, rhs) => {
                    terms_inner(*lhs, acc);

                    let mut acc2 = Vec::new();
                    terms_inner(*rhs, &mut acc2);

                    if acc2.len() != 1 {
                        panic!("The rhs should always be one expr");
                    }
                    let first = acc2.remove(0);
                    let expr = Expr::BinOp(Box::new(Expr::Num(0.0)), Op::Sub, Box::new(first));
                    acc.push(expr);
                }
                _ => acc.push(expr),
            }
        }

        terms_inner(self, &mut acc);

        acc
    }
}

fn handle_errors(errors: Vec<Simple<Token>>) -> Report<'static> {
    Report::build(ReportKind::Error, (), errors[0].span().start)
        .with_message("An error occurred while parsing")
        .with_labels(
            errors.into_iter().map(|error| {
                Label::new(error.span()).with_message("An error occurred while parsing")
            }),
        )
        .finish()
}

pub fn parse(tokens: Lexer<Token>) -> Result<Expr, Report> {
    let tokens = tokens
        .spanned()
        .into_iter()
        .filter(|(token, _)| token.is_ok())
        .collect::<Vec<_>>()
        .into_iter()
        .map(|(token, span)| (token.unwrap(), span.start..span.end));

    let parser = recursive(|top_level| {
        let atom = filter_map(|span, token: Token| {
            if let Token::Number(Float(num)) = token {
                Ok(Expr::Num(num))
            } else {
                Err(Simple::custom(span, "not a number"))
            }
        })
        .or(filter_map(|span, token: Token| {
            if let Token::Pronumeral(var) = token {
                Ok(Expr::Var(var))
            } else {
                Err(Simple::custom(span, "not a pronumeral"))
            }
        }))
        .or(just(Token::LParen)
            .ignore_then(top_level)
            .then_ignore(just(Token::RParen)))
        .or(just(Token::Pi).to(Expr::Constant(Constant::Pi)))
        .or(just(Token::E).to(Expr::Constant(Constant::E)));
        let exponentiation = atom
            .clone()
            .then(just(Token::Pow).to(Op::Pow).then(atom.clone()).repeated())
            .map(|(lhs, ops)| {
                let mut acc = lhs;
                for (op, rhs) in ops {
                    acc = Expr::BinOp(Box::new(acc), op, Box::new(rhs));
                }
                acc
            });

        let multiplication = just(Token::Sub)
            .or_not()
            .then(
                exponentiation
                    .clone()
                    .then(
                        just(Token::Div)
                            .to(Op::Div)
                            .or(just(Token::Mul).or_not().to(Op::Mul))
                            .then(exponentiation.clone())
                            .repeated(),
                    )
                    .map(|(lhs, ops)| {
                        let mut acc = lhs;
                        for (op, rhs) in ops {
                            acc = Expr::BinOp(Box::new(acc), op, Box::new(rhs));
                        }
                        acc
                    }),
            )
            .map(|(sub, rhs)| {
                if let Some(_) = sub {
                    Expr::BinOp(Box::new(Expr::Num(-1.0)), Op::Mul, Box::new(rhs))
                } else {
                    rhs
                }
            });
        let addition = multiplication
            .clone()
            .then(
                just(Token::Add)
                    .to(Op::Add)
                    .or(just(Token::Sub).to(Op::Sub))
                    .then(multiplication.clone())
                    .repeated(),
            )
            .map(|(lhs, ops)| {
                let mut acc = lhs;
                for (op, rhs) in ops {
                    acc = Expr::BinOp(Box::new(acc), op, Box::new(rhs));
                }
                acc
            });

        addition
    });

    let equation = parser
        .clone()
        .then_ignore(just(Token::Eq))
        .then(parser)
        .map(|(lhs, rhs)| Expr::Equation(Box::new(lhs), Box::new(rhs)));

    equation
        .parse(Stream::from_iter(0..tokens.len(), tokens.into_iter()))
        .map_err(handle_errors)
}

/*
GRAMMAR

EXPRESSION
    : ADDITION
    ;

ADDITION
    : ADDITION ('+' | '-') MULTIPLICATION
    | MULTIPLICATION
    ;

MULTIPLICATION
    : MULTIPLICATION ('*' | '/') EXPONENTIATION
    | EXPONENTIATION
    ;

EXPONENTIATION
    : EXPONENTIATION '^' BASIC
    | BASIC
    ;

BASIC
    : number
    | identifier
    | '"' EXPRESSION '"'
    | '(' EXPRESSION ')'
    ;

 */
