use nom::character::complete::*;
use nom::combinator::*;
use nom::sequence::*;
use nom::bytes::complete::*;
use nom::multi::*;
use nom::IResult;

use crate::parser::{
    identifier::*,
    space::*,
    call::*,
    func_def::*,
};

#[derive(Debug)]
pub struct ClassDefinition {
    classname: Identifier,
    inheritance: Option<Call>,
    funcs: Vec<FuncDefinition>,
}

impl ClassDefinition {
    pub fn parse<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, Self> {
        move |s| {
            let (s, (_, _, classname, _, inheritance, _, _, _, _, funcs)) = tuple((
                tag("class"), pyspace1, Identifier::parse, pyspace0, opt(Call::parse),
                pyspace0, char(':'), pyspace0, pynewline,
                many0(map(tuple((pyindent(indent + 1), FuncDefinition::parse(indent + 1))), |(_, f)| f))
            ))(s)?;
            Ok((s, Self { classname, inheritance, funcs }))
        }
    }
    pub fn transpile(self) -> String {
        let funcs = self.funcs.into_iter().map(|f| {
            let (name, lambda) = f.transpile_name_def();
            format!("\"{}\": ({})", name, lambda)
        }).collect::<Vec<_>>().join(", ");
        let inheritance = self.inheritance.map(|i| i.transpile()).unwrap_or(format!("()"));
        format!("{0} := type(\"{0}\", {1}, {{{2}}})", self.classname.transpile(), inheritance, funcs)
    }
}
