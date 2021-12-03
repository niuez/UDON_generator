use nom::branch::*;
use nom::character::complete::*;
use nom::combinator::*;
use nom::multi::*;
use nom::sequence::*;
use nom::IResult;

#[derive(Debug)]
pub enum Literal {
    Integer(PyInteger),
    Float(PyFloat),
    String(PyString),
}

impl Literal {
    pub fn parse(s: &str) -> IResult<&str, Literal> {
        alt((PyInteger::parse, PyFloat::parse, PyString::parse))(s)
    }
}

#[derive(Debug)]
pub struct PyInteger {
    num: String,
}

impl PyInteger {
    fn parse_nonzero(s: &str) -> IResult<&str, Literal> {
        let (s, (head, tail)) = tuple((
                one_of("123456789"),
                many0(tuple((opt(char('_')), one_of("0123456789"))))
            ))(s)?;
        let num = std::iter::once(head).chain(tail.into_iter().map(|(_, c)| c)).collect::<String>();
        Ok((s, Literal::Integer(PyInteger { num })))
    }
    fn parse_zero(s: &str) -> IResult<&str, Literal> {
        let (s, (_head, _tail)) = tuple((
                char('0'),
                many0(tuple((opt(char('_')), char('0'))))
            ))(s)?;
        Ok((s, Literal::Integer(PyInteger { num: format!("0") })))
    }
    pub fn parse(s: &str) -> IResult<&str, Literal> {
        alt((Self::parse_nonzero, Self::parse_zero))(s)
    }
}

#[derive(Debug)]
pub struct PyFloat {
    num: String,
}

fn parse_digitpart(s: &str) -> IResult<&str, String> {
        let (s, (head, tail)) = tuple((
                one_of("0123456789"),
                many0(tuple((opt(char('_')), one_of("0123456789"))))
            ))(s)?;
        let num = std::iter::once(head).chain(tail.into_iter().map(|(_, c)| c)).collect::<String>();
        Ok((s, num))
}

fn parse_pointfloat(s: &str) -> IResult<&str, String> {
    let (s, (opt_digit, _, fraction)) = tuple((opt(parse_digitpart), char('.'), parse_digitpart))(s)?;
    let num = format!("{}.{}", opt_digit.unwrap_or(String::new()), fraction);
    Ok((s, num))
}

impl PyFloat {
    fn parse_pointfloat(s: &str) -> IResult<&str, Literal> {
        let (s, num) = parse_pointfloat(s)?;
        Ok((s, Literal::Float(PyFloat { num })))
    }
    fn parse_pointfloat_pointend(s: &str) -> IResult<&str, Literal> {
        let (s, (digit, _,)) = tuple((parse_digitpart, char('.')))(s)?;
        let num = format!("{}.", digit);
        Ok((s, Literal::Float(PyFloat { num })))
    }
    fn parse_exponentfloat(s: &str) -> IResult<&str, Literal> {
        let (s, (fl, exp, pm, expdigit)) = tuple((alt((parse_pointfloat, parse_digitpart)), one_of("eE"), opt(one_of("+-")), parse_digitpart))(s)?;
        let num = format!("{}{}{}{}", fl, exp, pm.map(|c| c.to_string()).unwrap_or(String::new()), expdigit);
        Ok((s, Literal::Float(PyFloat { num })))
    }
    pub fn parse(s: &str) -> IResult<&str, Literal> {
        alt((Self::parse_exponentfloat, Self::parse_pointfloat, Self::parse_pointfloat_pointend))(s)
    }
}

#[derive(Debug)]
pub struct PyString {
    string: String,
}

fn parse_nonescaped_character(s: &str) -> IResult<&str, String> {
    let (s, c) = none_of("\\\"\'")(s)?;
    Ok((s, c.to_string()))
}

fn parse_escaped_character(s: &str) -> IResult<&str, String> {
    let (s, (_, c)) = tuple((char('\\'), anychar))(s)?;
    Ok((s, format!("\\{}", c.to_string())))
}

impl PyString {
    fn parse_double_quote(s: &str) -> IResult<&str, Literal> {
        let (s, (_, chars, _)) = tuple((char('\"'), many0(alt((parse_escaped_character, parse_nonescaped_character))), char('\"')))(s)?;
        Ok((s, Literal::String(PyString { string: chars.join("") })))
    }
    
    fn parse_single_quote(s: &str) -> IResult<&str, Literal> {
        let (s, (_, chars, _)) = tuple((char('\''), many0(alt((parse_escaped_character, parse_nonescaped_character))), char('\'')))(s)?;
        Ok((s, Literal::String(PyString { string: chars.join("") })))
    }
    pub fn parse(s: &str) -> IResult<&str, Literal> {
        alt((Self::parse_double_quote, Self::parse_single_quote))(s)
    }
}

#[test]
fn test_integer_parser() {
    println!("{:?}", PyInteger::parse("12345").unwrap());
    println!("{:?}", PyInteger::parse("123_456_789_0").unwrap());
    println!("{:?}", PyInteger::parse("000_000_000_000").unwrap());
}

#[test]
fn test_float_parser() {
    println!("{:?}", PyFloat::parse("3.14").unwrap());
    println!("{:?}", PyFloat::parse("10.").unwrap());
    println!("{:?}", PyFloat::parse(".001").unwrap());
    println!("{:?}", PyFloat::parse("1e100").unwrap());
    println!("{:?}", PyFloat::parse("3.14e-10").unwrap());
    println!("{:?}", PyFloat::parse("0e0").unwrap());
    println!("{:?}", PyFloat::parse("3.14_15_93").unwrap());
}

#[test]
fn test_string_parser() {
    println!("{:?}", PyString::parse("\'he\\nre\'").unwrap());
}
