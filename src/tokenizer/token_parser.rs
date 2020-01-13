use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, one_of},
    combinator::{cut, map},
    error::VerboseError,
    IResult,
    multi::many0,
    sequence::{preceded, terminated, tuple},
};
use nom::character::complete::{alpha1, alphanumeric0, alphanumeric1};

use super::Token;

// use multispace0
fn ws(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn comma(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tag(","), |_| Token::Comma)(i)
}

fn colon(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tag(":"), |_| Token::Colon)(i)
}

fn equal(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tag("="), |_| Token::Equal)(i)
}

fn open_paran(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tag("("), |_| Token::OpenParan)(i)
}

fn close_paran(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tag(")"), |_| Token::CloseParan)(i)
}

fn open_curly_brace(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tag("{"), |_| Token::OpenCurlyBrace)(i)
}

fn close_curly_brace(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tag("}"), |_| Token::CloseCurlyBrace)(i)
}

fn upper(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    one_of("ABCDEFGHIJKLMNOPQRSTUVWXYZ")(i)
}

fn lower(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    one_of("abcdefghijklmnopqrstuvwxyz")(i)
}

fn number(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    one_of("0123456789")(i)
}

fn id_char(i: &str) -> IResult<&str, char, VerboseError<&str>> {
    alt((lower, upper, number, one_of("_")))(i)
}

pub fn upper_id(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tuple((upper, many0(id_char))), |r| {
        Token::UpperIdentifier(concat_char_remaining(r))
    })(i)
}

pub fn lower_id(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    map(tuple((lower, many0(id_char))), |r| {
        Token::UpperIdentifier(concat_char_remaining(r))
    })(i)
}

pub fn identifier(i: &str) -> IResult<&str, String, VerboseError<&str>> {
    let first = alt((lower, upper, one_of("_")));
    let id = tuple((first, many0(id_char)));

    map(id, |r| concat_char_remaining(r))(i)
}

fn concat_char_remaining((fst_char, remaining): (char, Vec<char>)) -> String {
    let fst_char = fst_char.to_string();
    let remaining = remaining.into_iter().collect::<String>();

    format!("{}{}", fst_char, remaining)
}

#[cfg(test)]
mod tests {
    use nom::{Err, Err::*};
    use nom::error::convert_error;

    use super::*;

    #[test]
    fn identifier_t() {
        let result = identifier("_12sA");
//        assert_ok!(result, Token::Identifier("_12sA".to_string()))
        assert_ok!(result, "_12sA".to_string())
    }

    #[test]
    fn ws_t() {
        let result = ws(" \t\r\n|");
        assert_ok!(result, " \t\r\n")
    }

    #[test]
    fn comma_t() {
        let result = comma(",|");
        assert_ok!(result, Token::Comma)
    }

    #[test]
    fn left_paren_t() {
        let result = open_paran("(|");
        assert_ok!(result, Token::OpenParan);
    }

    #[test]
    fn right_paren_t() {
        let result = close_paran(")|");
        assert_ok!(result, Token::CloseParan);
    }

    #[test]
    fn id_t() {
        let result = upper_id("Vsse23_sd3");
        assert_ok!(result, Token::UpperIdentifier("Vsse23_sd3".to_string()))

        //        match result {
        //            Ok((rem, item)) => {println!("{:?}", item)}
        //            Err(Err::Error(e)) | Err(Err::Failure(e))  => { println!("{:?}", convert_error("sdf(", e)) }
        //            Err(Incomplete(_)) => {println!("kir")}
        //        }
    }
}

// Test snippet
/*match result {
Ok((rem, item)) => {println!("{:?}", item)}
Err(Err::Error(e)) | Err(Err::Failure(e))  => { println!("{:?}", convert_error("sdf(", e)) }
Err(Incomplete(_)) => {println!("kir")}
}*/
