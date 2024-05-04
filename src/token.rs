use std::fmt::Display;
use std::hash::{Hash, Hasher};
use std::str::FromStr;

use logos::Logos;

#[derive(Debug, Clone)]
pub struct Float(pub f64);

impl Hash for Float {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_string().hash(state);
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

impl Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
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
    #[regex("[a-zA-Z]", | lex | lex.slice().chars().next().expect("There should be something"))]
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

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Pi => write!(f, "pi"),
            Token::E => write!(f, "e"),
            Token::Number(n) => write!(f, "{n}"),
            Token::Pronumeral(c) => write!(f, "{c}"),
            Token::Add => write!(f, "Add"),
            Token::Sub => write!(f, "Sub"),
            Token::Mul => write!(f, "Mul"),
            Token::Div => write!(f, "Div"),
            Token::Pow => write!(f, "Pow"),
            Token::LParen => write!(f, "LParen"),
            Token::RParen => write!(f, "RParen"),
            Token::Eq => write!(f, "Eq"),
        }
    }
}
