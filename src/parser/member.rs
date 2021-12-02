use nom::character::complete::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    identifier::*,
    space::*,
};

#[derive(Debug)]
pub struct Member {
    id: Identifier,
}

impl Member {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, id)) = tuple((char('.'), pyspace0, Identifier::parse))(s)?;
        Ok((s, Self { id }))
    }
}

