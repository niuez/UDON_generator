use nom::IResult;
use nom::branch::*;
use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::multi::*;
use nom::sequence::*;

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn parse(s: &str) -> IResult<&str, Self> {
        let (s, (head, tails)) = tuple((
                alt((alpha1, tag("_"))),
                many0(alt((alphanumeric1, tag("_"))))
            ))(s)?;
        let name = std::iter::once(head).chain(tails.into_iter()).collect::<Vec<_>>().join("");
        Ok((s, Identifier { name }))
    }
    pub fn transpile(self) -> String {
        self.name
    }
}

#[test]
fn parse_identifier_test() {
    println!("{:?}", Identifier::parse("a_12").unwrap());
    println!("{:?}", Identifier::parse("abc_____").unwrap());
}
