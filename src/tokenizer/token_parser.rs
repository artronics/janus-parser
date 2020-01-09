use super::Token::*;
use crate::tokenizer::Token;
use crate::util::*;
use nom::IResult;
use nom::{
    bytes::complete::{tag, take_while},
    error::{ParseError, VerboseError},
};

fn ws(i: &str) -> IResult<&str, &str, VerboseError<&str>> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn left_paren(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (i, _) = tag("(")(i)?;
    Ok((i, LeftParan))
}

fn right_paren(i: &str) -> IResult<&str, Token, VerboseError<&str>> {
    let (i, _) = tag(")")(i)?;
    Ok((i, RightParan))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::tokenizer::Token::*;
    use nom::error::convert_error;
    use nom::Err;
    use nom::Err::Incomplete;

    #[test]
    fn ws_t() {
        let result = ws(" \t\r\n|");
        assert_ok!(result, " \t\r\n")
    }
    #[test]
    fn left_paren_t() {
        let result = left_paren("(|");
        assert_ok!(result, LeftParan);
    }

    #[test]
    fn right_paren_t() {
        let result = right_paren(")|");
        assert_ok!(result, RightParan);
    }
}

// Test snippet
/*match result {
Ok((rem, item)) => {println!("{:?}", item)}
Err(Err::Error(e)) | Err(Err::Failure(e))  => { println!("{:?}", convert_error("sdf(", e)) }
Err(Incomplete(_)) => {println!("kir")}
}*/
