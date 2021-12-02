use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    identifier::*,
    space::*,
    expression::*,
    assign::*,
};

/* https://docs.python.org/ja/3/reference/expressions.html#calls */
#[derive(Debug)]
pub struct Call {
    positional_args: Vec<PositionalArg>,
    starred_and_keywords: Vec<StarredAndKeywords>,
    keywords_args: Vec<KeywordsArgs>,
}

#[derive(Debug)]
pub enum PositionalArg {
    Assign(Assign),
    Starred(Expression),
}

impl PositionalArg {
    pub fn parse(s: &str) -> IResult<&str, PositionalArg> {
        alt((
            map(Assign::parse, |a| Self::Assign(a)),
            map(tuple((char('*'), pyspace0, Expression::parse)), |(_, _, e)| Self::Starred(e))
        ))(s)
    }
}

#[derive(Debug)]
pub enum StarredAndKeywords {
    Starred(Expression),
    Keyword(KeywordItem),
}

impl StarredAndKeywords {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((
            map(tuple((char('*'), pyspace0, Expression::parse)), |(_, _, e)| Self::Starred(e)),
            map(KeywordItem::parse, |k| Self::Keyword(k))
        ))(s)
    }
}

#[derive(Debug)]
pub enum KeywordsArgs {
    DoubleStarred(Expression),
    Keyword(KeywordItem),
}

impl KeywordsArgs {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((
            map(tuple((tag("**"), pyspace0, Expression::parse)), |(_, _, e)| Self::DoubleStarred(e)),
            map(KeywordItem::parse, |k| Self::Keyword(k))
        ))(s)
    }
}

#[derive(Debug)]
pub struct KeywordItem {
    id: Identifier,
    expr: Expression,
}

impl KeywordItem {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (id, _, _, _, expr)) = tuple((Identifier::parse, pyspace0, char('='), pyspace0, Expression::parse))(s)?;
        Ok((s, Self { id, expr }))
    }
}

impl Call {
    fn parse_positional(s: &str) -> IResult<&str, Self> {
        let (s, (_, positional_args, starred_and_keywords, keywords_args, _, _, _)) = tuple((
            char('('),
            separated_list1(tuple((pyspace0, char(','))), map(tuple((pyspace0, PositionalArg::parse)), |(_, p)| p)),
            many0(map(tuple((pyspace0, char(','), pyspace0, StarredAndKeywords::parse)), |(_, _, _, s)| s)),
            many0(map(tuple((pyspace0, char(','), pyspace0, KeywordsArgs::parse)), |(_, _, _, k)| k)),
            opt(tuple((pyspace0, char(',')))),
            pyspace0,
            char(')'),
        ))(s)?;
        Ok((s, Self { positional_args, starred_and_keywords, keywords_args }))
    }
    fn parse_starred(s: &str) -> IResult<&str, Self> {
        let (s, (_, starred_and_keywords, keywords_args, _, _, _)) = tuple((
            char('('),
            separated_list1(tuple((pyspace0, char(','))), map(tuple((pyspace0, StarredAndKeywords::parse)), |(_, p)| p)),
            many0(map(tuple((pyspace0, char(','), pyspace0, KeywordsArgs::parse)), |(_, _, _, k)| k)),
            opt(tuple((pyspace0, char(',')))),
            pyspace0,
            char(')'),
        ))(s)?;
        Ok((s, Self { positional_args: Vec::new(), starred_and_keywords, keywords_args }))
    }
    fn parse_keywords(s: &str) -> IResult<&str, Self> {
        let (s, (_, keywords_args, _, _, _)) = tuple((
            char('('),
            separated_list1(tuple((pyspace0, char(','))), map(tuple((pyspace0, KeywordsArgs::parse)), |(_, p)| p)),
            opt(tuple((pyspace0, char(',')))),
            pyspace0,
            char(')'),
        ))(s)?;
        Ok((s, Self { positional_args: Vec::new(), starred_and_keywords: Vec::new(), keywords_args }))
    }
    fn parse_empty(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, _)) = tuple((char('('), pyspace0, char(')')))(s)?;
        Ok((s, Self { positional_args: Vec::new(), starred_and_keywords: Vec::new(), keywords_args: Vec::new() }))
    }
    pub fn parse(s: &str) -> IResult<&str, Self> {
        alt((Self::parse_positional, Self::parse_starred, Self::parse_keywords, Self::parse_empty))(s)
    }
}
