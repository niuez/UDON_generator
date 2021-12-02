use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    expression::*,
    space::*,
};

/* https://docs.python.org/ja/3/reference/expressions.html#slicings */
#[derive(Debug)]
pub enum Index {
    Index(Expression),
    Slice(Slice),
}

impl Index {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, index, _, _)) = tuple((char('['), pyspace0,
            alt((
                map(Expression::parse, |e| Self::Index(e)),
                map(Slice::parse, |s| Self::Slice(s)),
            )), pyspace0, char(']')
        ))(s)?;
        Ok((s, index))
    }
}

#[derive(Debug)]
pub struct Slice {
    lower: Option<Expression>,
    upper: Option<Expression>,
    stride: Option<Expression>,
}

impl Slice {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (lower, _, _, _, upper, stride)) = tuple((
            opt(Expression::parse), pyspace0, char(':'), pyspace0,
            opt(Expression::parse),
            opt(map(tuple((pyspace0, char(':'), pyspace0, Expression::parse)), |(_, _, _, s)| s))
        ))(s)?;
        Ok((s, Self { lower, upper, stride }))
    }
}
