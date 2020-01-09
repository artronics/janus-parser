#[macro_use]
pub mod util;

pub mod tokenizer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn te() {
        assert_eq!("foo", "foo")
    }
}
