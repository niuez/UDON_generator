use nom::branch::*;
use nom::combinator::*;
use nom::IResult;

use crate::parser::{
    unary_expr::*,
};

#[derive(Debug)]
pub enum Expression {
    Unary(UnaryExpr),
}

impl Expression {
    pub fn parse(s: &str) -> IResult<&str, Expression> {
        alt((
            map(UnaryExpr::parse, |u| Self::Unary(u)),
        ))(s)
    }
}
