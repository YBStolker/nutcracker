pub struct FormLeftBitIterator {
    cards: u64,
    bit_cursor: u64,
}

pub trait IntoFromLeftBitIterator {
    fn iter_from_left(self) -> FormLeftBitIterator;
}

impl IntoFromLeftBitIterator for u64 {
    fn iter_from_left(self) -> FormLeftBitIterator {
        let mut bit_cursor = 1u64.reverse_bits();
        if self != 0 {
            bit_cursor >>= self.leading_zeros();
        }

        FormLeftBitIterator {
            cards: self,
            bit_cursor,
        }
    }
}

impl Iterator for FormLeftBitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bit_cursor != 0 {
            if (self.cards & self.bit_cursor) != 0 {
                let result = Some(self.bit_cursor);
                self.bit_cursor >>= 1;
                return result;
            }
            self.bit_cursor >>= 1;
        }
        None
    }
}

pub struct FromRightBitIterator {
    cards: u64,
    bit_cursor: u64,
}

pub trait IntoFromRightBitIterator {
    fn iter_from_right(self) -> FromRightBitIterator;
}

impl IntoFromRightBitIterator for u64 {
    fn iter_from_right(self) -> FromRightBitIterator {
        let mut bit_cursor = 1;
        if self != 0 {
            bit_cursor <<= self.trailing_zeros();
        }

        FromRightBitIterator {
            cards: self,
            bit_cursor,
        }
    }
}

impl Iterator for FromRightBitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        while self.bit_cursor != 0 {
            if (self.cards & self.bit_cursor) != 0 {
                let result = Some(self.bit_cursor);
                self.bit_cursor <<= 1;
                return result;
            }
            self.bit_cursor <<= 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterate() {
        let base = 0b0010000010000100000100000010000000001001000100;

        assert_eq!(base, base.iter_from_left().sum::<u64>(), "lr sum");
        assert_eq!(base, base.iter_from_right().sum::<u64>(), "rl sum");

        for i in base.iter_from_right() {
            assert_eq!((base & i).count_ones(), 1);
        }

        for i in base.iter_from_left() {
            assert_eq!((base & i).count_ones(), 1);
        }

        let mut base_list_lr = base.iter_from_left().collect::<Vec<u64>>();
        base_list_lr.reverse();
        for (i, n) in base_list_lr.iter().enumerate() {
            assert_eq!(
                Some(*n),
                base.iter_from_right()
                    .enumerate()
                    .find(|(j, _)| &i == j)
                    .map(|(_, x)| x)
            );
        }
    }
}
