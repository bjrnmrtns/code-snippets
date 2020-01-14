use std::fmt::{Formatter};

type Result<T> = std::result::Result<T, BorError>;

#[derive(Debug)]
pub enum BorError {
    EqualsError,
    Parse(std::num::ParseIntError),
}

impl std::fmt::Display for BorError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            BorError::EqualsError => write!(f, "two values should not be equal"),
            BorError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl std::error::Error for BorError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            BorError::EqualsError => None,
            BorError::Parse(ref e) => Some(e),
        }
    }
}

impl From<std::num::ParseIntError> for BorError {
    fn from(err: std::num::ParseIntError) -> BorError {
        BorError::Parse(err)
    }
}

pub fn sum_if_not_equal(left: &str, right: &str) -> Result<i32>
{
    let left = left.parse::<i32>()?;
    let right = right.parse::<i32>()?;
    if left == right {
        Err(BorError::EqualsError)
    } else {
        Ok(left + right)
    }
}

fn main() {
    assert_eq!(sum_if_not_equal("3", "4").unwrap(), 7);
    assert!(sum_if_not_equal("3.0", "4").is_err());
    assert!(sum_if_not_equal("3", "3").is_err());

}