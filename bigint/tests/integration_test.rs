#[cfg(test)]
use bigint::BigInteger;
use bigint::Sign;
use std::str::FromStr;

#[test]
fn positive_fromstr() {
    let a : BigInteger =  FromStr::from_str("123").unwrap();
    assert_eq!(a.digits, vec![3,2,1]);
}

#[test]
fn negative_fromstr() {
    let a : BigInteger =  FromStr::from_str("-123").unwrap();
    assert_eq!(a.sign, Sign::Negative);
}

// TODO Test invalid input.

#[test]
fn simple_add() {
    let a : BigInteger =  FromStr::from_str("2011").unwrap();
    let b : BigInteger = FromStr::from_str("12345").unwrap();
    let res = a + &b;
    assert_eq!(res.digits, vec![6,5,3,4,1]);
}

#[test]
fn negative_add() {
    let a : BigInteger =  FromStr::from_str("2011").unwrap();
    let b : BigInteger = FromStr::from_str("-12345").unwrap();
    let res = a + &b;
    assert_eq!(res.digits, vec![4,3,3,0,1]);
    assert_eq!(res.sign, Sign::Negative);
}

#[test]
fn add_to_zero() {
    let a : BigInteger =  FromStr::from_str("-9876").unwrap();
    let b : BigInteger = FromStr::from_str("9876").unwrap();
    let res = a + &b;
    assert_eq!(res.digits, vec![0]);
    assert_eq!(res.sign, Sign::Positive);
}
