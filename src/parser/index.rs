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
                map(Slice::parse, |s| Self::Slice(s)),
                map(Expression::parse, |e| Self::Index(Box::new(e))),
            )), pyspace0, char(']')
        ))(s)?;
        Ok((s, index))
    }
    pub fn transpile_var(self) -> String {
        match self {
            Self::Index(e) => (*e).transpile(),
            Self::Slice(s) => s.transpile(),
        }
    }
    pub fn transpile(self) -> String {
        format!("[{}]", self.transpile_var())
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
    pub fn transpile(self) -> String {
        let lower = self.lower.map(|e| (*e).transpile()).unwrap_or(format!("None"));
        let upper = self.upper.map(|e| (*e).transpile()).unwrap_or(format!("None"));
        let stride = self.stride.map(|e| (*e).transpile()).unwrap_or(format!("None"));
        format!("slice({}, {}, {})", lower, upper, stride)
    }
}
