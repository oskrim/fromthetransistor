use quickcheck::Arbitrary;
use std::fmt;

use crate::parser::Deparse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl Deparse for Op {
    fn deparse(&self) -> String {
        match self {
            Op::Add => "+".to_string(),
            Op::Sub => "-".to_string(),
            Op::Mul => "*".to_string(),
            Op::Div => "/".to_string(),
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
            _ => unreachable!(),
        }
    }
}
