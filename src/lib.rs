fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;
//    use pretty_assertions::{assert_eq, assert_ne};


    #[test]
    fn component_token() {
        assert_eq!("foo", "fox")
    }
}
