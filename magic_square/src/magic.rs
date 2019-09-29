// NxN square of [1-N2] digits.
#[derive(Clone, Copy)]
pub struct Square {
    digits : [u32; Square::N2 as usize],
    seen: u64
}

impl Square {
    const N: u32 = 3;
    const N2: u32 = Square::N*Square::N;
    const K: u32 = Square::N*(Square::N2+1)/2;

    pub fn new() -> Square {
        Square {
            digits : [0; Square::N2 as usize],
            seen: 0
        }
    }

    // Try adding digit at ind.  If position occupied or digit already used,
    // return false.
    fn set(&mut self, ind: usize, digit: u32) -> bool {
        let bit = 1 << (digit-1);
        if bit & self.seen != 0 || self.digits[ind] != 0 {
            return false;
        }
        self.digits[ind] = digit;
        self.seen |= bit;
        true
    }

    // Clear a digit from a position at ind.
    fn clear(&mut self, ind: usize) -> bool {
        self.print_digits();
        if self.digits[ind] == 0 {
            return false;
        }
        let mask: u64 = !(1 << self.digits[ind]-1);
        self.digits[ind] = 0;
        self.seen &= mask;
        self.print_digits();
        true
    }

    fn empty(&self) -> bool {
        self.seen == 0
    }

    // Verify all values occur once and fall in [1,N2].
    fn is_valid(&self) -> bool {
        let mut expected: u64 = 0;
        for i in 0..Square::N2 {
            expected |= 1 << i;
        }
        self.seen == expected
    }

    // Check if Square is a magic square.
    fn is_magic(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        let digits = &self.digits;
        const N : usize = Square::N as usize;

        // Verify rows sum to K.
        for row_begin in (0..N*N).step_by(N) {
            let row_end = row_begin + N;
            let row = &digits[row_begin..row_end];
            if row.iter().sum::<u32>() != Square::K {
                return false;
            }
        }
        // Verify columns sum to K.
        for col_begin in 0..N {
            if digits.iter()
                     .skip(col_begin)
                     .step_by(N)
                     .sum::<u32>() != Square::K {
                return false;
            }
        }

        // Verify diagonal starting in top left corner.
        let left : u32 = (0..N).map(|i| {digits[i*N + i]}).sum();
        if left != Square::K {
            return false;
        }
        // Verify diagonal starting in top right corner.
        let right : u32 = (1..N+1).map(|i| {digits[i*N - i]}).sum();
        if right != Square::K {
            return false;
        }
        true
    }

    pub fn print_digits(&self) {
        println!("square:");
        println!("{:?}", &self.digits[0..3]);
        println!("{:?}", &self.digits[3..6]);
        println!("{:?}", &self.digits[6..9]);
        println!("{:0b}", self.seen as u16);
    }
}

// Find 3x3 magic squares.
pub fn find_magic() -> Vec<Square> {
    assert!(Square::N == 3);
    fn complement(digit: u32) -> u32 {
        Square::K - 5 - digit
    }

    fn fill_edge(square: &mut Square, ind: usize, magic_squares: &mut Vec<Square>) {
        let compl_ind = 8-ind;
        for digit in 1..10 {
            let complement = complement(digit);
            if square.set(ind, digit) && square.set(compl_ind, complement) {
                if ind == 3 {
                    if square.is_magic() {
                        magic_squares.push(square.clone());
                    }
                } else {
                    fill_edge(square, ind+1, magic_squares);
                }
            }
            square.clear(ind);
            square.clear(compl_ind);
        }
    }

    let mut square = Square::new();
    square.set(4, 5);
    let mut magic_squares = Vec::<Square>::new();
    fill_edge(&mut square, 0, &mut magic_squares);
    magic_squares
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid() {
        let digits : [u32; 9] = [2, 7, 6, 9, 5, 1, 4, 3, 8];
        let mut square = Square::new();
        for (i, d) in digits.iter().enumerate() {
            square.set(i, *d);
        }
        assert!(square.is_valid());
    }

    #[test]
    fn test_not_is_valid() {
        let digits : [u32; 9] = [2, 2, 6, 9, 5, 1, 4, 3, 8];
        let mut square = Square::new();
        for (i, d) in digits.iter().enumerate() {
            square.set(i, *d);
        }
        assert!(!square.is_valid());
    }

    #[test]
    fn test_is_magic() {
        let digits : [u32; 9] = [2, 7, 6, 9, 5, 1, 4, 3, 8];
        let square = Square { digits, seen: 511 };
        assert!(square.is_magic());
    }

    #[test]
    fn test_clear() {
        let digits : [u32; 9] = [2, 7, 6, 9, 5, 1, 4, 3, 8];
        let mut square = Square { digits, seen: 511 };
        square.clear(0);
        assert_eq!(square.seen, 0b111111101);

    }

    #[test]
    fn test_find_magic() {
        let squares = find_magic();
        assert_eq!(squares.len(), 8)
    }
}
