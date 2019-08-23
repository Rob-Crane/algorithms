use std::fmt;
use std::iter::Iterator;
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;
use std::slice::Iter;

#[derive(Debug, Clone, PartialEq)]
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
    pub digits: Vec<u32>,
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

impl<'a> Sum<&'a BigInteger> for BigInteger {
    fn sum<I>(iter: I) -> BigInteger
    where
        I: Iterator<Item = &'a BigInteger>,
    {
        let total = BigInteger::zero();
        //let mut digit_iters: Vec<Iter<u32>> = iter.map(|x| x.digits.iter()).collect();
        //let mut input_exhaust = false;
        //let mut place_sum : i32 = 0; // Running total for place.
        //let mut carryover : i32 = 0; // Amount of next place.
        //while !input_exhaust && carryover != 0 {
            //let sign_iter : Iter<Sign> = iter.map(|x| x.sign);
            //for i in &mut digit_iters { // zip here
                //input_exhaust = true;
                //let opt_digit = i.next();
                //if opt_digit.is_some() {

                    //input_exhaust = false;
                    //println!("val: {}", opt_digit.unwrap());
                //}
            //}
        //}
        total
    }
}


fn add_mag(a: &BigInteger, b: &BigInteger) -> BigInteger {
    assert!(a.sign == b.sign);
    let mut carryover: u32 = 0;
    let mut a_iter = a.iter();
    let mut b_iter = b.iter();
    let mut opt_a = a_iter.next();
    let mut opt_b = b_iter.next();
    let digits: Vec<u32> = Vec::new();
    while (opt_a.is_some() || opt_b.is_some()) && carryover > 0 {
        if let Some(a_digit) = opt_a {
            carryover += a_digit;
        }
        if let Some(b_digit) = opt_b {
            carryover += b_digit;
        }
        digits.push(carryover % 10)
        carryover = carryover / 10;
        opt_a = a_iter.next();
        opt_b = b_iter.next();
    }
    BigInteger {
        digits,
        sign: a.sign
    }
}

fn mag_order(a: &BigInteger, b: &BigInteger) -> (&BigInteger, &BigInteger) {

    let greater_mag: &BigInteger;
    let smaller_mag: &BigInteger;
    if a.digits.len() == b.digits.len() {
        if a.digits.last() > b.digits.last() {
            greater_mag = &a;
            smaller_mag = &b;
        } else {
            greater_mag = &b;
            smaller_mag = &a;
        }

    }
    else if (a.digits.len() > b.digits.len()) {
        greater_mag = &a;
        smaller_mag = &b;
    } else {
        greater_mag = &b;
        smaller_mag = &a;
    }
    smaller_mag, greater_mag
}

fn diff_mag(a: &BigInteger, b: &BigInteger) -> BigInteger {
    assert!(a.sign != b.sign);
    let smaller, greater = mag_order(a, b);

}

impl<'a> Add<&'a BigInteger> for BigInteger {
    type Output = Self;
    fn add(self, other: &'a BigInteger) -> Self::Output {
        let digits: Vec<u32>;
        let sign: Sign;
        if (self.sign == other.sign) {
          digits = add_mag(self.digits, other.digits);
          sign = self.sign;
        }
        if (self.sign != other.sign) {

        }
        self
    }
}

pub type Result = std::result::Result<BigInteger, BigIntegerError>;

impl FromStr for BigInteger {
    type Err = BigIntegerError;

    fn from_str(s: &str) -> Result {
        let mut ret_sign = Sign::Positive;
        let mut chars = s.chars().peekable();
        //TODO understand the ref to char syntax
        if chars.peek() == Some(&'-') {
            ret_sign = Sign::Negative;
            chars.next();
        }
        let mut digits: Vec<u32> = Vec::with_capacity(s.len());
        for (i, c) in chars.enumerate() {
            digits.push(c.to_digit(10).ok_or(BigIntegerError {
                msg: "Invalid char!",
            })?);
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
