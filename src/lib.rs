#[macro_use]
pub mod util;

pub mod tokenizer;
pub mod parser;

#[cfg(test)]
mod tests {

    #[test]
    fn te() {
        assert_eq!("foo", "foo")
    }
}
