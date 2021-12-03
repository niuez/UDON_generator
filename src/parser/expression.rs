use nom::branch::*;
use nom::combinator::*;
use nom::bytes::complete::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    unary_expr::*,
    space::*,
};

#[derive(Debug)]
pub enum Expression {
    Not(Box<Expression>),
    Binaries(BinaryExpr),
}

impl Expression {
    pub fn parse(s: &str) -> IResult<&str, Expression> {
        alt((
            map(tuple((tag("not"), pyspace0, Expression::parse)), |(_, _, e)| Self::Not(Box::new(e))),
            map(BinaryExpr::parse, |b| Self::Binaries(b)),
        ))(s)
    }
}

#[derive(Debug)]
pub struct BinaryExpr {
    exprs: Vec<UnaryExpr>,
    opes: Vec<String>,
}

fn parse_operator(s: &str) -> IResult<&str, String> {
    let (s, ope) = alt((
        tag("*"), tag("@"), tag("//"), tag("/"), tag("%"),
        tag("+"), tag("-"), tag("<<"), tag(">>"), tag("&"),
        tag("^"), tag("|"), tag("<"), tag(">"), tag("=="),
        tag(">="), tag("<="), tag("!="), tag("or"), tag("and")
    ))(s)?;
    Ok((s, ope.to_string()))
}

impl BinaryExpr {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (mut s, head) = UnaryExpr::parse(s)?;
        let mut exprs = vec![head];
        let mut opes = Vec::new();
        while let Ok((ss, (_, ope, _, right))) = tuple((
            pyspace0, parse_operator, pyspace0, UnaryExpr::parse
        ))(s) {
            s = ss;
            exprs.push(right);
            opes.push(ope);
        }
        Ok((s, BinaryExpr { exprs, opes }))
    }
}

#[test]
fn parse_expression_test() {
    println!("{:?}", Expression::parse("1 + 2 + 3").unwrap());
    println!("{:?}", Expression::parse("not 1 == 2").unwrap());
}
