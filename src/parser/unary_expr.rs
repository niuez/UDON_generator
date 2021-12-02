use nom::branch::*;
use nom::combinator::*;
use nom::IResult;

use crate::parser::{
    literal::*,
    identifier::*,
};

#[derive(Debug)]
pub enum UnaryExpr {
    Literal(Literal),
    Identifier(Identifier),
}

impl UnaryExpr {
    pub fn parse(s: &str) -> IResult<&str, UnaryExpr> {
        alt((
                map(Literal::parse, |l| UnaryExpr::Literal(l)),
                map(Identifier::parse, |i| UnaryExpr::Identifier(i))
        ))(s)
    }
}
