use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::{alpha0, char, multispace0};
use nom::character::complete::alpha1;
use nom::combinator::{cut, map};
use nom::error::{context, VerboseError};
use nom::IResult;
use nom::multi::separated_list;
use nom::sequence::{delimited, preceded, terminated};

use crate::tokenizer::{lower_id, Token, upper_id};

struct Component {
    nets: HashSet<String>,
    elements: Vec<String>
}
//fn component_nets(i: &str) -> IResult<&str, Ve>
fn elements(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let id = preceded(multispace0, lower_id)(i);

}
fn inside_const(i: &str) -> IResult<&str, Vec<Token>, VerboseError<&str>> {
    let upper_id_ws = preceded(multispace0, upper_id);
    context(
        "constructor",
        preceded(
            char('('),
            cut(terminated(
                separated_list(preceded(multispace0, char(',')), upper_id_ws),
                preceded(multispace0, char(')')),
            )),
        ),
    )(i)
}

fn inside_net_const(i: &str) -> IResult<&str, Vec<Token>, VerboseError<&str>> {
    let upper_id_ws = preceded(multispace0, upper_id);
    let ids_list = separated_list(preceded(multispace0, char(',')), upper_id_ws);
    let close = preceded(multispace0, char('>'));

    context(
        "net constructor",
        preceded(
            char('<'), cut(terminated(ids_list, close)),
        ),
    )(i)
}

#[cfg(test)]
mod tests {
    use nom::character::complete::alpha0;

    use super::*;

    #[test]
    fn constructor_parser() {
        let result = inside_const("(  \t\nVo \t\n,  \t\nVi \t\n)");
        assert_ok!(result, vec![
            Token::UpperIdentifier("Vo".to_string()),
            Token::UpperIdentifier("Vi".to_string())]
        );

        let result = inside_const("( \t\n )");
        assert_ok!(result, vec![]);
    }

    #[test]
    fn net_constructor_parser() {
        let result = inside_net_const("<  \t\nVo \t\n,  \t\nVi \t\n>");
        assert_ok!(result, vec![
            Token::UpperIdentifier("Vo".to_string()),
            Token::UpperIdentifier("Vi".to_string())]
        );

        let result = inside_net_const("< \t\n >");
        assert_ok!(result, vec![]);
    }
}

