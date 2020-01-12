#[macro_use]
#[cfg(test)]
macro_rules! assert_ok {
    ($r: expr, $tk: expr) => {
        match &$r {
            Ok((rem, item)) => {
                assert_eq!(*item, $tk, "Remaining: {:?}", rem);
            }
            Err(_) => {
                panic!("{:?}", $r);
            }
        }
    };
}
/*

    ($r: expr, $tk: expr) => {
        match &$r {
            Ok((rem, item)) => {
                assert_eq!(*item, $tk, "Remaining: {:?}", rem);
            }
            Err(Err::Error(e)) | Err(Err::Failure(e)) => {
                println!("{:?}", convert_error("kir", e));
            }
            Err(Incomplete(_)) => { println!("kirj tosh")}
        }
    };
*/
