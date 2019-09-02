use std::fmt;
use std::iter::Iterator;
use std::iter::Sum;
use std::ops::Add;
use std::ops::Not;
use std::ops::Sub;
use std::str::FromStr;

#[derive(Debug)]
pub struct BigIntegerError {
    pub msg: &'static str,
}

impl fmt::Display for BigIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

impl Not for Sign {
    type Output = Sign;
    fn not(self) -> Self::Output {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive
        }
    }
}

pub struct BigInteger {
    pub digits: Vec<u8>,
    pub sign: Sign,
}

impl BigInteger {
    fn zero() -> BigInteger {
        BigInteger {
            digits: vec![0],
            sign: Sign::Positive,
        }
    }
}

impl fmt::Display for BigInteger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        if self.sign == Sign::Negative {
            res.push('-');
        }
        for i in self.digits.iter().rev() {
            res.push((*i + 48) as char);
        }
        write!(f, "{}", res)
    }
}

pub type Result = std::result::Result<BigInteger, BigIntegerError>;

impl FromStr for BigInteger {
    type Err = BigIntegerError;

    fn from_str(s: &str) -> Result {
        let mut ret_sign = Sign::Positive;
        let mut chars = s.chars().peekable();
        if chars.peek() == Some(&'-') {
            ret_sign = Sign::Negative;
            chars.next();
        }
        let mut digits: Vec<u8> = Vec::with_capacity(s.len());
        for c in chars {
            let d = c as u8;
            assert!(d >= 48 && d <= 57, "Invalid input!");
            digits.push(d - 48);
        }
        digits.reverse();
        if digits.len() == 0 {
            return Err(BigIntegerError {
                msg: "Empty input!",
            });
        } else {
            Ok(BigInteger {
                digits,
                sign: ret_sign,
            })
        }
    }
}

impl<'a> Sum<&'a BigInteger> for BigInteger {
    fn sum<I>(iter: I) -> BigInteger
    where
        I: Iterator<Item = &'a BigInteger>,
    {
        let mut total = BigInteger::zero();
        for i in iter {
            total = total + i;
        }
        total
    }
}

impl<'a> Add<&'a BigInteger> for BigInteger {
    type Output = Self;
    fn add(self, other: &'a BigInteger) -> Self::Output {
        if self.sign == other.sign {
            add_mag(self, other)
        } else {
            diff_mag(self, other)
        }
    }
}

impl<'a> Sub<&'a BigInteger> for BigInteger {
    type Output = Self;
    fn sub(self, other: &'a BigInteger) -> Self::Output {

        if self.sign != other.sign {
            add_mag(self, other)
        } else {
            diff_mag(self, other)
        }
    }
}

// For BigIntegers with matching signs, add the magnitudes.
fn add_mag(mut a: BigInteger, b: &BigInteger) -> BigInteger {
    let mut carryover: u8 = 0;
    let mut b_iter = b.digits.iter();
    let mut opt_b = b_iter.next();
    for a_digit in a.digits.iter_mut() {
        if let Some(b_digit) = opt_b {
            carryover += b_digit;
            opt_b = b_iter.next();
        }
        *a_digit += carryover % 10;
        carryover = carryover / 10;
    }
    while (opt_b.is_some()) || carryover > 0 {
        if let Some(b_digit) = opt_b {
            carryover += b_digit;
            opt_b = b_iter.next();
        }
        a.digits.push(carryover % 10);
        carryover = carryover / 10;
    }
    a
}

// Conduct addition for BigIntegers with differing signs.
// Resulting magnitude is the difference in magnitude of input.
// The sign is the sign of the greater magnitude input.
fn diff_mag(mut a: BigInteger, b: &BigInteger) -> BigInteger {
    let a_greater = mag_greater(&a, b);
    let mut borrowing = false;
    let mut i = 0; // Current place.
    while i < a.digits.len() {
        let (s, g) = get_digits(a_greater, i, &a.digits, &b.digits);
        a.digits[i] = diff_op(s, g, &mut borrowing);
        i += 1;
    }
    if !a_greater {
        while i < b.digits.len() {
            let (s, g) = get_digits(false, i, &a.digits, &b.digits);
            a.digits.push(diff_op(s, g, &mut borrowing));
            i += 1;
        }
        a.sign = if a.sign == b.sign { !b.sign } else { b.sign };
    }
    // Trim leading zeros.
    while a.digits.len() > 1 && *a.digits.last().unwrap() == 0 {
        a.digits.pop();
    }
    a
}

// True if a has greater magnitude than b.
fn mag_greater(a: &BigInteger, b: &BigInteger) -> bool {
    if a.digits.len() == b.digits.len() {
        if a.digits.last() > b.digits.last() {
            true
        } else {
            false
        }
    } else if a.digits.len() > b.digits.len() {
        true
    } else {
        false
    }
}

// Get digits occupying ith place.  Return ordering based on a_greater.
// First value returned is digit from smaller magnitude BigInteger.
fn get_digits(a_greater: bool, i: usize, a: &Vec<u8>, b: &Vec<u8>) -> (u8, u8) {
    let (s, g);
    if a_greater {
        s = if i < b.len() { b[i] } else { 0 };
        g = a[i];
    } else {
        s = if i < a.len() { a[i] } else { 0 };
        g = b[i];
    }
    (s, g)
}

// Subtract a single place's digits and update the borrow state.
//
// The borrow state, initially false, means that the previous
// place required a "borrow" from the current place for a
// non-negative result.
fn diff_op(s: u8, g: u8, borrow: &mut bool) -> u8 {
    if *borrow {
        if g == 0 {
            9 - s
        } else if (g - 1) < s {
            *borrow = true;
            g + 9 - s
        } else {
            *borrow = false;
            g - 1 - s
        }
    } else {
        if g < s {
            *borrow = true;
            g + 10 - s
        } else {
            g - s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn positive_add_mag() {
        let a : BigInteger =  FromStr::from_str("2011").unwrap();
        let b : BigInteger = FromStr::from_str("12345").unwrap();
        let res = add_mag(a, &b);
        assert_eq!(res.digits, vec![6,5,3,4,1]);
        assert_eq!(res.sign, Sign::Positive);
    }

    #[test]
    fn negative_add_mag() {
        let a : BigInteger =  FromStr::from_str("-2011").unwrap();
        let b : BigInteger = FromStr::from_str("12345").unwrap();
        let res = add_mag(a, &b);
        assert_eq!(res.digits, vec![6,5,3,4,1]);
        assert_eq!(res.sign, Sign::Negative); // Retains sign of a.
    }

    #[test]
    fn noinvert_diff_mag() {
        let a : BigInteger =  FromStr::from_str("-2011").unwrap();
        let b : BigInteger = FromStr::from_str("12345").unwrap();
        let res = diff_mag(a, &b, false);
        assert_eq!(res.digits, vec![4,3,3,0,1]);
        assert_eq!(res.sign, Sign::Positive); // Retains sign of a.
    }

    #[test]
    fn invert_diff_mag() {
        let a : BigInteger =  FromStr::from_str("-2011").unwrap();
        let b : BigInteger = FromStr::from_str("12345").unwrap();
        let res = diff_mag(a, &b, true);
        // TODO Is this invalid input?
        assert_eq!(res.digits, vec![4,3,3,0,1]);
        assert_eq!(res.sign, Sign::Negative); // Retains sign of a.
    }
}
