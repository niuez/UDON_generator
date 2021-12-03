use nom::bytes::complete::*;
use nom::character::complete::*;
use nom::sequence::*;
use nom::multi::*;
use nom::IResult;

pub fn pyspace0(s: &str) -> IResult<&str, &str> {
    space0(s)
}

pub fn pyspace1(s: &str) -> IResult<&str, &str> {
    space1(s)
}

pub fn pynewline(s: &str) -> IResult<&str, ()> {
    let (s, _) = tuple((pyspace0, line_ending))(s)?;
    Ok((s, ()))
}

pub fn pyindent<'a>(indent: usize) -> impl FnMut(&'a str) -> IResult<&'a str, ()> {
    move |s| {
        many_m_n(indent, indent, tag("  "))(s).map(|(s, _)| (s, ()))
    }
}
