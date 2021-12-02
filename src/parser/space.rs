use nom::character::complete::*;
use nom::IResult;

pub fn pyspace0(s: &str) -> IResult<&str, &str> {
    space0(s)
}

pub fn pyspace1(s: &str) -> IResult<&str, &str> {
    space1(s)
}
