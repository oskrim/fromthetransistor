use quickcheck::Arbitrary;
use std::fmt;

use crate::parser::Deparse;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
    Eq,
    Le,
    Ge,
    Lt,
    Gt,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
}

impl Deparse for Op {
    fn deparse(&self) -> String {
        match self {
            Op::Add => "+".to_string(),
            Op::Sub => "-".to_string(),
            Op::Mul => "*".to_string(),
            Op::Div => "/".to_string(),
            Op::Eq => "==".to_string(),
            Op::Le => "<=".to_string(),
            Op::Ge => ">=".to_string(),
            Op::Lt => "<".to_string(),
            Op::Gt => ">".to_string(),
            Op::Assign => "=".to_string(),
            Op::AddAssign => "+=".to_string(),
            Op::SubAssign => "-=".to_string(),
            Op::MulAssign => "*=".to_string(),
            Op::DivAssign => "/=".to_string(),
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
            4 => Op::Eq,
            5 => Op::Le,
            6 => Op::Ge,
            7 => Op::Lt,
            8 => Op::Gt,
            9 => Op::Assign,
            10 => Op::AddAssign,
            11 => Op::SubAssign,
            12 => Op::MulAssign,
            13 => Op::DivAssign,
            _ => unreachable!(),
        }
    }
}
