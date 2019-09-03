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
        add_op(self, other, false)
    }
}

impl<'a> Sub<&'a BigInteger> for BigInteger {
    type Output = Self;
    fn sub(self, other: &'a BigInteger) -> Self::Output {
        add_op(self, other, true)
    }
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagnitudeOrder {
    First,
    Second,
    Equal
}

fn add_op(mut a: BigInteger, b: &BigInteger, invert_b: bool) -> BigInteger {
    let same_sign = if invert_b {a.sign != b.sign} else {a.sign == b.sign};
    if same_sign {
        a.digits = add_mag(a.digits, &b.digits);
    } else {
        let (digits, mag_order) = diff_mag(a.digits, &b.digits);
        a.digits = digits;
        if mag_order == MagnitudeOrder::Second {
            a.sign = !a.sign;
        } else if mag_order == MagnitudeOrder::Equal {
            a.sign = Sign::Positive;
        }
    }
    a
}

// For BigIntegers with matching signs, add the magnitudes.
fn add_mag(mut a_digits: Vec<u8>, b_digits: &Vec<u8>) -> Vec<u8> {
    let mut carryover: u8 = 0;
    let mut b_iter = b_digits.iter();
    let mut opt_b = b_iter.next();
    for a_digit in a_digits.iter_mut() {
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
        a_digits.push(carryover % 10);
        carryover = carryover / 10;
    }
    a_digits
}

// Compute the difference in magnitude of a_digits and b_digits.  Return
// difference and flag indicating if a_digits had greater
// magnitude.
fn diff_mag(mut a_digits: Vec<u8>, b_digits: &Vec<u8>) -> (Vec<u8>, MagnitudeOrder) {
    let mag_order = mag_greater(&a_digits, b_digits);
    let mut borrowing = false;
    let mut i = 0; // Current place.
    while i < a_digits.len() {
        let (s, g) = get_digits(mag_order, i, &a_digits, b_digits);
        a_digits[i] = diff_op(s, g, &mut borrowing);
        i += 1;
    }
    if mag_order == MagnitudeOrder::Second {
        while i < b_digits.len() {
            let (s, g) = get_digits(mag_order, i, &a_digits, b_digits);
            a_digits.push(diff_op(s, g, &mut borrowing));
            i += 1;
        }
    }
    // Trim leading zeros.
    while a_digits.len() > 1 && *a_digits.last().unwrap() == 0 {
        a_digits.pop();
    }
    (a_digits, mag_order)
}

fn mag_greater(a_digits: &Vec<u8>, b_digits: &Vec<u8>) -> MagnitudeOrder {
    if a_digits.len() == b_digits.len() {
        for (a_digit, b_digit) in a_digits.iter().zip(b_digits.iter()) {
            if a_digit > b_digit {
                return MagnitudeOrder::First
            } else if b_digit > a_digit {
                return MagnitudeOrder::Second
            }
        }
        MagnitudeOrder::Equal
    } else if a_digits.len() > b_digits.len() {
        MagnitudeOrder::First
    } else {
        MagnitudeOrder::Second
    }
}

// Get digits occupying ith place.  Return ordering based on a_greater.
// First value returned is digit from smaller magnitude BigInteger.
fn get_digits(mag_order: MagnitudeOrder, i: usize, a: &Vec<u8>, b: &Vec<u8>) -> (u8, u8) {
    let (s, g);
    if mag_order == MagnitudeOrder::First {
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

    #[test]
    fn zero_add_mag() {
        let a = vec![0];
        let b = vec![0];
        let res = add_mag(a, &b);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn nonzero_add_mag() {
        let a = vec![1,1,0,2];
        let b = vec![5,4,3,2,1];
        let res = add_mag(a, &b);
        assert_eq!(res, vec![6,5,3,4,1]);
    }
}
