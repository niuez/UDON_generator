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
    Index(Box<Expression>),
    Slice(Slice),
}

impl Index {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, index, _, _)) = tuple((char('['), pyspace0,
            alt((
                map(Expression::parse, |e| Self::Index(Box::new(e))),
                map(Slice::parse, |s| Self::Slice(s)),
            )), pyspace0, char(']')
        ))(s)?;
        Ok((s, index))
    }
}

#[derive(Debug)]
pub struct Slice {
    lower: Option<Box<Expression>>,
    upper: Option<Box<Expression>>,
    stride: Option<Box<Expression>>,
}

impl Slice {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (lower, _, _, _, upper, stride)) = tuple((
            opt(map(Expression::parse, |e| Box::new(e))), pyspace0, char(':'), pyspace0,
            opt(map(Expression::parse, |e| Box::new(e))),
            opt(map(tuple((pyspace0, char(':'), pyspace0, Expression::parse)), |(_, _, _, s)| Box::new(s)))
        ))(s)?;
        Ok((s, Self { lower, upper, stride }))
    }
}
