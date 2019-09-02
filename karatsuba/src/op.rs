use std::fmt;
use std::iter::Iterator;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sign {
    Positive,
    Negative,
}

#[derive(Debug)]
pub struct BigIntegerError {
    pub msg: &'static str,
}

impl fmt::Display for BigIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
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
            res.push((*i+48) as char);
        }
        write!(f, "{}", res)
    }
}

impl<'a> Sum<&'a BigInteger> for BigInteger {
    fn sum<I>(_iter: I) -> BigInteger
    where
        I: Iterator<Item = &'a BigInteger>,
    {
        let total = BigInteger::zero();
        total
    }
}

fn add_mag(mut a: BigInteger, b: &BigInteger) -> BigInteger {
    assert!(a.sign == b.sign);
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

fn diff_op(g: u8, s: u8, borrow: &mut bool) -> u8 {
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

fn get_digits(a_longer: bool, i: usize, a: &Vec<u8>, b: &Vec<u8>) -> (u8, u8) {
    let (s, g);
    if a_longer {
        s = if i < b.len() { b[i] } else { 0 };
        g = a[i];
    } else {
        s = if i < a.len() { a[i] } else { 0 };
        g = b[i];
    }
    (s, g)
}

fn diff_mag(mut a: BigInteger, b: &BigInteger) -> BigInteger {
    assert!(a.sign != b.sign);
    let a_longer = mag_greater(&a, b);
    let mut borrowing = false;
    let mut i = 0;
    while i < a.digits.len() {
        let (s, g) = get_digits(a_longer, i, &a.digits, &b.digits);
        a.digits[i] = diff_op(g, s, &mut borrowing);
        i += 1;
    }
    if !a_longer {
        while i < b.digits.len() {
            let (s, g) = get_digits(false, i, &a.digits, &b.digits);
            a.digits.push(diff_op(g, s, &mut borrowing));
            i += 1;
        }
        a.sign = b.sign;
    }
    // Trim leading zeros.
    while (a.digits.len() > 1 && *a.digits.last().unwrap() == 0) {
        a.digits.pop();
    }
    a
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
            let d = c as u8 - 48;
            assert!(d < 10, "Invalid input!"); 
            digits.push(d);
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
