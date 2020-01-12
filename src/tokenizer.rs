pub use token_parser::identifier;
pub use token_parser::lower_id;
pub use token_parser::upper_id;

mod token_parser;

#[derive(PartialEq, Debug)]
pub enum Token {
    Identifier(String),
    UpperIdentifier(String),
    LowerIdentifier(String),
    OpenParan,
    CloseParan,
    OpenCurlyBrace,
    CloseCurlyBrace,
    Comma,
    Colon,
    Equal,
}

