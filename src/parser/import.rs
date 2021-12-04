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

#[derive(Debug)]
pub struct FromImport {
    module: Vec<Identifier>,
    imports: Vec<(Identifier, Option<Identifier>)>,
}

impl FromImport {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (_, _, module, _, _, _, imports)) = tuple((
            tag("from"), pyspace1,
            separated_list1(tuple((pyspace0, char('.'))), map(tuple((pyspace0, Identifier::parse)), |(_, id)| id)),
            pyspace1, tag("import"), pyspace1,
            separated_list1(
                tuple((pyspace0, char(','))),
                tuple((
                    map(tuple((pyspace0, Identifier::parse)), |(_, id)| id),
                    opt(map(tuple((pyspace1, tag("as"), pyspace1, Identifier::parse)), |(_, _, _, id)| id))
                )),
            ),
        ))(s)?;
        Ok((s, Self { module, imports }))
    }

    pub fn transpile(self) -> String {
        let module = self.module.into_iter().map(|id| id.transpile()).collect::<Vec<_>>().join(".");
        let from_list = self.imports.iter().map(|(id, _)| format!("\"{}\"", id.clone().transpile())).collect::<Vec<_>>().join(", ");
        let import = format!("udon_import := __import__(\"{}\", fromlist=[{}])", module, from_list);
        let assigns = self.imports.into_iter().map(|(module, as_id)| {
            let id = as_id.map(|id| id.transpile()).unwrap_or_else(|| module.clone().transpile());
            format!("{} := udon_import.{}", id, module.transpile())
        }).collect::<Vec<_>>().join(", ");
        format!("{}, {}", import, assigns)
    }
}

#[test]
fn parse_fromimport_test() {
    println!("{:?}", all_consuming(FromImport::parse)("from matplotlib import pyplot, pyplot as plt").unwrap());
}
