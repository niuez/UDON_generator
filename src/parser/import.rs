use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::bytes::complete::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    identifier::*,
    space::*,
};

#[derive(Debug)]
pub struct Import {
    module: Vec<Identifier>,
    as_id: Option<Identifier>,
}

impl Import {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, module, as_id)) = tuple((
            tag("import"), pyspace1,
            separated_list1(tuple((pyspace0, char('.'))), map(tuple((pyspace0, Identifier::parse)), |(_, id)| id)),
            opt(map(tuple((pyspace1, tag("as"), pyspace1, Identifier::parse)), |(_, _, _, id)| id))
        ))(s)?;
        Ok((s, Import { module, as_id }))
    }

    pub fn transpile(self) -> String {
        let id = self.as_id.map(|id| id.transpile()).unwrap_or_else(|| self.module[0].clone().transpile());
        let module = self.module.into_iter().map(|id| id.transpile()).collect::<Vec<_>>().join(".");
        format!("{} := __import__(\"{}\")", id, module)
    }
}

#[test]
fn parse_import_test() {
    println!("{:?}", all_consuming(Import::parse)("import numpy").unwrap());
    println!("{:?}", all_consuming(Import::parse)("import matplotlib.pyplot as plt").unwrap());
}
