// 3x3 square of [0-9[ digits.
struct Square {
    digits : [u8; 9]
}

impl Square {
    const N: usize = 3;
    pub fn new() -> Square {
        Square {
            digits : [0; Square::N * Square::N]
        }
    }

    //pub fn rotated_clockwise(&self) -> Self {
    //}

    //pub fn reflected(&self) -> Self {
    //}

    // Verify all values fall in [1,9].
    //fn valid(&self) -> bool {
    //}

    fn is_magic(&self) -> bool {
        let digits = &self.digits;
        const N : usize = Square::N;
        const K : u8 = (N * (N * N + 1) / 2) as u8;

        for row_begin in (0..N*N).step_by(N) {
            let row_end = row_begin + N;
            let row = &digits[row_begin..row_end];
            if row.iter().sum::<u8>() != K {
                return false;
            }
        }
        if digits.iter().step_by(3).sum::<u8>() != K {
            return false;
        }
        if digits.iter().skip(1).step_by(3).sum::<u8>() != K {
            return false;
        }
        if digits.iter().skip(2).step_by(3).sum::<u8>() != K {
            return false;
        }
        if digits[0] + digits[4] + digits[8] != K {
            return false;
        }
        if digits[2] + digits[4] + digits[6] != K {
            return false;
        }
        true
    }
}

//// Tree search for a magic square.
//pub fn find_magic() -> Square {
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_magic() {
        let digits : [u8; 9] = [2, 7, 6, 9, 5, 1, 4, 3, 8];
        let square = Square { digits };
        assert!(square.is_magic());
    }
}
