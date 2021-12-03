use nom::bytes::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::IResult;

use crate::parser::{
    identifier::*,
    expression::*,
    space::*,
};

#[derive(Debug)]
pub struct Assign {
    id: Option<Identifier>,
    expr: Expression,
}

impl Assign {
    pub fn parse(s: &str) -> IResult<&str, Assign> {
        let (s, (id, expr)) = tuple((
            opt(map(tuple((Identifier::parse, pyspace0, tag(":="), pyspace0)), |(id, _, _, _)| id)), Expression::parse))(s)?;
        Ok((s, Assign { id, expr }))
    }
    pub fn transpile(self) -> String {
        let id = self.id.map(|id| format!("{} := ", id.transpile())).unwrap_or(format!(""));
        format!("{}{}", id, self.expr.transpile())
    }
}
