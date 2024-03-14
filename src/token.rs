use std::hash::{Hash, Hasher};
use std::str::FromStr;

use logos::Logos;

#[derive(Debug, Clone)]
pub struct Float(pub f64);

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state)
    }
}

impl PartialEq for Float {
    fn eq(&self, other: &Self) -> bool {
        self.0.total_cmp(&other.0).is_eq()
    }
}

impl FromStr for Float {
    type Err = <f64 as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Float(s.parse()?))
    }
}

impl Eq for Float {}

#[derive(Debug, Logos, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // Constants
    #[token("pi")]
    #[token("PI")]
    Pi,
    #[token("e", priority = 100)]
    E,

    // Values
    #[regex("[0-9]+", | lex | lex.slice().parse().ok())]
    #[regex("[0-9]+\\.[0-9]+", | lex | lex.slice().parse().ok())]
    Number(Float),
    #[regex("[a-zA-Z]", | lex | lex.slice().chars().next().unwrap())]
    Pronumeral(char),

    // Symbols
    #[token("+")]
    Add,
    #[token("-")]
    Sub,
    #[token("*")]
    Mul,
    #[token("/")]
    Div,
    #[token("^")]
    Pow,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("=")]
    Eq,
}
