use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric0, char, multispace0, one_of};
use nom::combinator::{cut, map, opt};
use nom::error::{context, VerboseError};
use nom::multi::{many0, many1, separated_list};
use nom::sequence::{preceded, terminated, tuple};
use nom::IResult;

use crate::parser::util::*;

#[derive(PartialEq, Debug)]
struct Element {
    name: Token,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Identifier(String),
    DoubleColon,
}

fn element(i: &str) -> IResult<&str, Element, VerboseError<&str>> {
    // r1::R(10k, 20m)<N1, N2>
    let parts = tuple((identifier, equal, identifier, com_cons, net_cons));
    map(parts, |(rhs, _double_colon, _com, _value, _nets)| Element {
        name: rhs,
    })(i)
}

fn equal(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(preceded(multispace0, char('=')), |_| Token::DoubleColon)(i)
}

fn com_cons(i: &str) -> IResult<&str, Vec<f64>, VerboseError<&str>> {
    let ids_list = separated_list(preceded(multispace0, char(',')), number_suffix);
    let close = preceded(multispace0, char(')'));

    let cons = context(
        "component constructor",
        preceded(char('('), cut(terminated(ids_list, close))),
    );

    preceded(multispace0, cons)(i)
}

fn net_cons(i: &str) -> IResult<&str, Vec<Token>, VerboseError<&str>> {
    let ids_list = separated_list(preceded(multispace0, char(',')), identifier);
    let close = preceded(multispace0, char('>'));

    let cons = context(
        "net constructor",
        preceded(char('<'), cut(terminated(ids_list, close))),
    );

    preceded(multispace0, cons)(i)
}

#[cfg(test)]
mod tests {
    use nom::error::convert_error;
    use nom::Err;

    use super::*;

    #[test]
    fn element_t() {
        let result = element("r1 = R(10k)<N1, N2>");
        assert_ok!(
            result,
            Element {
                name: Token::Identifier("r1".to_string())
            }
        );

        let result = element("r1 \n= \t R ( 10k, 30m ) \n< N1 , \tN2 > \n");
        assert_ok!(
            result,
            Element {
                name: Token::Identifier("r1".to_string())
            }
        )
    }

    #[test]
    fn element_error() {
        let data = "r1 = R(10k)<N1 N2>";
        let result = element(data);
        match result {
            Err(Err::Error(e)) | Err(Err::Failure(e)) => println!("{}", convert_error(data, e)),
            _ => println!("kir"),
        }
        //        assert_ok!(result, Element {name: Token::Identifier("r1".to_string())})
    }
}

/*
component Divider(Vin, up_res = 10k)<Vin, Vout> {
    r1::R(10k)<N1 -> Vin, N2 -> Vout>
    r2::R(up_res)<Vin, Vout>

    com::PowerSupply()<>

}
*/
