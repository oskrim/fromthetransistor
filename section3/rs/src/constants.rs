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
    Ne,
    Le,
    Ge,
    Lt,
    Gt,
    Or,
    And,
    // Assign,
    // AddAssign,
    // SubAssign,
    // MulAssign,
    // DivAssign,
}

impl Deparse for Op {
    fn deparse(&self) -> String {
        match self {
            Op::Add => "+".to_string(),
            Op::Sub => "-".to_string(),
            Op::Mul => "*".to_string(),
            Op::Div => "/".to_string(),
            Op::Eq => "==".to_string(),
            Op::Ne => "!=".to_string(),
            Op::Le => "<=".to_string(),
            Op::Ge => ">=".to_string(),
            Op::Lt => "<".to_string(),
            Op::Gt => ">".to_string(),
            // Op::Assign => "=".to_string(),
            // Op::AddAssign => "+=".to_string(),
            // Op::SubAssign => "-=".to_string(),
            // Op::MulAssign => "*=".to_string(),
            // Op::DivAssign => "/=".to_string(),
            Op::Or => "||".to_string(),
            Op::And => "&&".to_string(),
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
            5 => Op::Ne,
            6 => Op::Le,
            7 => Op::Ge,
            8 => Op::Lt,
            9 => Op::Gt,
            10 => Op::Or,
            11 => Op::And,
            // 10 => Op::Assign,
            // 11 => Op::AddAssign,
            // 12 => Op::SubAssign,
            // 13 => Op::MulAssign,
            // 14 => Op::DivAssign,
            _ => unreachable!(),
        }
    }
}
