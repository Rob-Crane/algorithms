use std::cmp;
use std::fmt;
use std::iter;
use std::ops::Add;
use std::ops::Mul;
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
            Sign::Negative => Sign::Positive,
        }
    }
}

#[derive(Clone)]
pub struct BigInteger {
    pub digits: Vec<u8>,
    pub sign: Sign,
}

// Defines ordering of magnitude between two operands.
// Value is the operand with the greater magnitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MagnitudeOrder {
    First,
    Second,
    Equal,
}

impl BigInteger {
    fn zero() -> BigInteger {
        BigInteger {
            digits: vec![0],
            sign: Sign::Positive,
        }
    }

    fn add_op(mut self, other: &BigInteger, invert_other: bool) -> BigInteger {
        let same_sign = if invert_other {
            self.sign != other.sign
        } else {
            self.sign == other.sign
        };
        if same_sign {
            self.digits = add_mag(self.digits, &other.digits);
        } else {
            let (digits, mag_order) = diff_mag(self.digits, &other.digits);
            self.digits = digits;
            if mag_order == MagnitudeOrder::Second {
                self.sign = !self.sign;
            } else if mag_order == MagnitudeOrder::Equal {
                self.sign = Sign::Positive;
            }
        }
        self.trim_zeros()
    }

    fn trim_zeros(mut self) -> BigInteger {
        self.digits = trim_zeros(self.digits);
        if self.digits.len() == 1 && *self.digits.last().unwrap() == 0 {
            self.sign = Sign::Positive;
        }
        self
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

impl iter::Sum<BigInteger> for BigInteger {
    fn sum<I>(iter: I) -> BigInteger
    where
        I: iter::Iterator<Item = BigInteger>,
    {
        let mut total = BigInteger::zero();
        for i in iter {
            total = total + &i;
        }
        total
    }
}

impl<'a> Add<&'a BigInteger> for BigInteger {
    type Output = Self;
    fn add(self, other: &'a BigInteger) -> Self::Output {
        self.add_op(other, false)
    }
}

impl<'a> Sub<&'a BigInteger> for BigInteger {
    type Output = Self;
    fn sub(self, other: &'a BigInteger) -> Self::Output {
        self.add_op(other, true)
    }
}

impl<'a> Mul<&'a BigInteger> for BigInteger {
    type Output = Self;
    fn mul(mut self, other: &'a BigInteger) -> Self::Output {
        self.digits = multiply(&self.digits, &other.digits);
        self.sign = if self.sign == other.sign {
            Sign::Positive
        } else {
            Sign::Negative
        };
        self.trim_zeros()
    }
}

fn add_mag(mut a_digits: Vec<u8>, b_digits: &Vec<u8>) -> Vec<u8> {
    let mut carryover: u8 = 0;
    let mut b_iter = b_digits.iter();
    let mut opt_b = b_iter.next();
    for a_digit in a_digits.iter_mut() {
        carryover += *a_digit;
        if let Some(b_digit) = opt_b {
            carryover += b_digit;
            opt_b = b_iter.next();
        }
        *a_digit = carryover % 10;
        carryover /= 10;
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

fn gradeschool_multiply(a_digits: &[u8], b_digits: &[u8]) -> Vec<u8> {
    a_digits
        .iter()
        .enumerate()
        .map(|(i, c)| exp(scale(Vec::<u8>::from(b_digits), *c), i))
        .fold(vec![0], |a, v| add_mag(a, &v))
}

fn scale(mut digits: Vec<u8>, c: u8) -> Vec<u8> {
    let mut carryover: u8 = 0;
    for digit in digits.iter_mut() {
        let place_result = *digit * c + carryover;
        *digit = place_result % 10;
        carryover = place_result / 10;
    }
    if carryover != 0 {
        digits.push(carryover);
    }
    trim_zeros(digits)
}

fn exp(mut digits: Vec<u8>, e: usize) -> Vec<u8> {
    digits.reserve(e);
    digits.splice(0..0, iter::repeat(0).take(e));
    digits
}

// Trim leading zeros.
fn trim_zeros(mut digits: Vec<u8>) -> Vec<u8> {
    while digits.len() > 1 && *digits.last().unwrap() == 0 {
        digits.pop();
    }
    digits
}

const KARATSUBA_CUTOFF: usize = 20;

// Initial step of karatsuba multiplication requires generation
// of terms from multiplication of splits of input.
fn multiply(a_digits: &[u8], b_digits: &[u8]) -> Vec<u8> {
    let ret;
    let min_len = cmp::min(a_digits.len(), b_digits.len());
    if min_len < KARATSUBA_CUTOFF {
        ret = gradeschool_multiply(a_digits, b_digits);
    } else {
        let split = min_len / 2;
        let a0 = Vec::<u8>::from(&a_digits[..split]); // Lower order
        let a1 = Vec::<u8>::from(&a_digits[split..]); // Higher order
        let b0 = Vec::<u8>::from(&b_digits[..split]); // Lower order
        let b1 = Vec::<u8>::from(&b_digits[split..]); // Higher order

        println!(
            "Calling multiply with a: {:?} b: {:?}\na0: {:?} a1: {:?}\nb0: {:?} b1: {:?}\n",
            a_digits, b_digits, a0, a1, b0, b1
        );
        println!("add_mag(b0, b1): {:?}", add_mag(b0.clone(), &b1));

        let z0 = multiply(&a0, &b0);
        let z2 = multiply(&a1, &b1);
        let (z1, mag_order) = diff_mag(
            multiply(&add_mag(a0, &a1), &add_mag(b0, &b1)),
            &add_mag(z0.clone(), &z2),
        );
        assert_eq!(mag_order, MagnitudeOrder::First);
        ret = add_mag(add_mag(exp(z2, 2 * split), &exp(z1, split)), &z0);
    }
    trim_zeros(ret)
}

// Compute the difference in magnitude of a_digits and b_digits.  Return
// difference and flag indicating if a_digits had greater
// magnitude.
fn diff_mag(mut a_digits: Vec<u8>, b_digits: &Vec<u8>) -> (Vec<u8>, MagnitudeOrder) {
    let mag_order = mag_greater(&a_digits, b_digits);
    if mag_order == MagnitudeOrder::Equal {
        a_digits.clear();
        a_digits.push(0);
    } else {
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
    }
    (a_digits, mag_order)
}

fn mag_greater(a_digits: &Vec<u8>, b_digits: &Vec<u8>) -> MagnitudeOrder {
    if a_digits.len() == b_digits.len() {
        for (a_digit, b_digit) in a_digits.iter().rev().zip(b_digits.iter().rev()) {
            if a_digit > b_digit {
                return MagnitudeOrder::First;
            } else if b_digit > a_digit {
                return MagnitudeOrder::Second;
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
    fn add_mag_zero() {
        let a = vec![0];
        let b = vec![0];
        let res = add_mag(a, &b);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn add_mag_over() {
        let a = vec![6];
        let b = vec![5];
        let res = add_mag(a, &b);
        assert_eq!(res, vec![1, 1]);
    }

    #[test]
    fn add_mag_nonzero() {
        let a = vec![1, 1, 0, 2];
        let b = vec![5, 4, 3, 2, 1];
        let res = add_mag(a, &b);
        assert_eq!(res, vec![6, 5, 3, 4, 1]);
    }

    #[test]
    fn scale_zero() {
        let a = vec![1, 1, 0, 2];
        let res = scale(a, 0);
        assert_eq!(res, vec![0]);
    }

    #[test]
    fn scale_nonzero() {
        let a = vec![1, 1, 0, 2];
        let res = scale(a, 9);
        assert_eq!(res, vec![9, 9, 0, 8, 1]);
    }

    #[test]
    fn exp_zero() {
        let a = vec![1, 1, 0, 2];
        let res = exp(a, 0);
        assert_eq!(res, vec![1, 1, 0, 2]);
    }

    #[test]
    fn exp_nonzero() {
        let a = vec![1, 1, 0, 2];
        let res = exp(a, 2);
        assert_eq!(res, vec![0, 0, 1, 1, 0, 2]);
    }

    #[test]
    fn gradeschool_nonzero() {
        let a = vec![1, 1, 0, 2];
        let b = vec![3, 2, 1];
        let res = gradeschool_multiply(&a, &b);
        assert_eq!(res, vec![3, 5, 3, 7, 4, 2]);
    }

    #[test]
    fn diff_mag_zero() {
        let a = vec![1, 1, 0, 2];
        let b = vec![1, 1, 0, 2];
        let (ret, mag_order) = diff_mag(a, &b);
        assert_eq!(ret, vec![0]);
        assert_eq!(mag_order, MagnitudeOrder::Equal);
    }

    #[test]
    fn diff_mag_first() {
        let a = vec![0, 1, 1, 0, 2];
        let b = vec![1, 1, 0, 2];
        let (ret, _) = diff_mag(a, &b);
        assert_eq!(ret, vec![9, 9, 0, 8, 1]);
    }

    #[test]
    fn diff_mag_second() {
        let a = vec![1, 1, 0, 2];
        let b = vec![2, 1, 1, 0, 2];
        let (ret, _) = diff_mag(a, &b);
        assert_eq!(ret, vec![1, 0, 1, 8, 1]);
    }

    #[test]
    fn mag_greater_equal() {
        let a = vec![1, 1, 0, 2];
        let b = vec![1, 1, 0, 2];
        let ret = mag_greater(&a, &b);
        assert_eq!(ret, MagnitudeOrder::Equal);
    }

    #[test]
    fn mag_greater_first() {
        let a = vec![0, 1, 1, 0, 2];
        let b = vec![1, 1, 0, 2];
        let ret = mag_greater(&a, &b);
        assert_eq!(ret, MagnitudeOrder::First);
    }

    #[test]
    fn mag_greater_second() {
        let a = vec![1, 1, 0, 2];
        let b = vec![0, 1, 1, 0, 2];
        let ret = mag_greater(&a, &b);
        assert_eq!(ret, MagnitudeOrder::Second);
    }

    #[test]
    fn get_digits_first() {
        let mag_order = MagnitudeOrder::First;
        let i = 1;
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7];
        let (s, g) = get_digits(mag_order, i, &a, &b);
        assert_eq!(s, 6);
        assert_eq!(g, 2);
    }

    #[test]
    fn get_digits_second() {
        let mag_order = MagnitudeOrder::Second;
        let i = 1;
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7, 8];
        let (s, g) = get_digits(mag_order, i, &a, &b);
        assert_eq!(s, 2);
        assert_eq!(g, 6);
    }

    #[test]
    fn get_digits_past_s() {
        let mag_order = MagnitudeOrder::First;
        let i = 3;
        let a = vec![1, 2, 3, 4];
        let b = vec![5, 6, 7];
        let (s, g) = get_digits(mag_order, i, &a, &b);
        assert_eq!(s, 0);
        assert_eq!(g, 4);
    }

    #[test]
    fn diff_op_simple() {
        let g = 9;
        let s = 6;
        let mut borrow = false;
        let ret = diff_op(s, g, &mut borrow);
        assert_eq!(ret, 3);
        assert_eq!(borrow, false);
    }

    #[test]
    fn diff_op_need_borrow() {
        let s = 9;
        let g = 6;
        let mut borrow = false;
        let ret = diff_op(s, g, &mut borrow);
        assert_eq!(ret, 7);
        assert_eq!(borrow, true);
    }

    #[test]
    fn diff_op_had_borrow() {
        let g = 9;
        let s = 6;
        let mut borrow = true;
        let ret = diff_op(s, g, &mut borrow);
        assert_eq!(ret, 2);
        assert_eq!(borrow, false);
    }

    #[test]
    fn diff_op_had_borrow_and_need() {
        let g = 6;
        let s = 6;
        let mut borrow = true;
        let ret = diff_op(s, g, &mut borrow);
        assert_eq!(ret, 9);
        assert_eq!(borrow, true);
    }

    #[test]
    fn diff_op_zero() {
        let g = 6;
        let s = 6;
        let mut borrow = false;
        let ret = diff_op(s, g, &mut borrow);
        assert_eq!(ret, 0);
        assert_eq!(borrow, false);
    }

    #[test]
    fn multiply_nonzero() {
        let a = vec![4, 3, 2, 1];
        let b = vec![8, 7, 6, 5];
        let res = multiply(&a, &b);
        assert_eq!(res, vec![2, 5, 6, 6, 0, 0, 7]);
    }

    #[test]
    fn multiply_zero() {
        let a = vec![1, 1, 0, 2];
        let b = vec![0];
        let res = multiply(&a, &b);
        assert_eq!(res, vec![0]);
    }

}
