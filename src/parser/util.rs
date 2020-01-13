use nom::branch::alt;
use nom::character::complete::{multispace0, one_of};
use nom::combinator::map;
use nom::error::VerboseError;
use nom::multi::{many0, many1};
use nom::sequence::{preceded, tuple};
use nom::IResult;

use crate::parser::element::Token;
use crate::parser::util::Suffix::{Giga, Kilo, Mega, Micro, Milli, Nano, Pico};

pub fn upper(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}

pub fn lower(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    one_of("abcdefghijklmnopqrstuvwxyz")(i)
}

pub fn number(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    one_of("0123456789")(i)
}

pub fn identifier(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let remaining = alt((first_char_id, number));
    let id = preceded(multispace0, tuple((first_char_id, many0(remaining))));

    map(id, |r| Token::Identifier(concat_char_string(r)))(i)
}

fn first_char_id(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    alt((upper, lower, one_of("_")))(i)
}

// TODO: support floating point, suffix must be optional, sort out unwrap()
pub fn number_suffix(i: &str) -> IResult<&str, f64, VerboseError<&str>> {
    let suffix = map(one_of("pnumkMG"), |s| Suffix::from_char(s));

    let num = preceded(multispace0, tuple((many1(number), suffix)));
    map(num, |(n, s)| {
        s.apply_suffix(n.into_iter().collect::<String>().parse().unwrap())
    })(i)
}

pub fn concat_char_string((fst_char, remaining): (char, Vec<char>)) -> String {
    let fst_char = fst_char.to_string();
    let remaining = remaining.into_iter().collect::<String>();

    format!("{}{}", fst_char, remaining)
}

enum Suffix {
    Pico,  // p
    Nano,  // n
    Micro, // u
    Milli, // m
    Kilo,  // k
    Mega,  // M
    Giga,  // G
}

impl Suffix {
    fn apply_suffix(self, n: f64) -> f64 {
        match &self {
            Pico => n * 1e-12,
            Nano => n * 1e-9,
            Micro => n * 1e-6,
            Milli => n * 1e-3,
            Kilo => n * 1e+3,
            Mega => n * 1e+6,
            Giga => n * 1e+9,
        }
    }
    fn from_char(s: char) -> Suffix {
        match s {
            'p' => Pico,
            'n' => Nano,
            'u' => Micro,
            'm' => Milli,
            'k' => Kilo,
            'M' => Mega,
            'G' => Giga,
            _ => unreachable!("Invalid suffix: {}", s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn values_t() {
        let result = number_suffix("100k");
        assert_ok!(result, 100_000.0);
    }
}
