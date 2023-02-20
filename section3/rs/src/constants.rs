use quickcheck::Arbitrary;
use std::fmt;

use crate::parser::Deparse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Le,
    Ge,
    Lt,
    Gt,
}

impl Deparse for Op {
    fn deparse(&self) -> String {
        match self {
            Op::Add => "+".to_string(),
            Op::Sub => "-".to_string(),
            Op::Mul => "*".to_string(),
            Op::Div => "/".to_string(),
            Op::Le => "<=".to_string(),
            Op::Ge => ">=".to_string(),
            Op::Lt => "<".to_string(),
            Op::Gt => ">".to_string(),
        }
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.deparse())
    }
}

impl Arbitrary for Op {
    fn arbitrary(g: &mut quickcheck::Gen) -> Self {
        match u32::arbitrary(g) % 4 {
            0 => Op::Add,
            1 => Op::Sub,
            2 => Op::Mul,
            3 => Op::Div,
            4 => Op::Le,
            5 => Op::Ge,
            6 => Op::Lt,
            7 => Op::Gt,
            _ => unreachable!(),
        }
    }
}
