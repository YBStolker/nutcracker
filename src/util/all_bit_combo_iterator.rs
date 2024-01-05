use super::bit_iterator::IntoFromRightBitIterator;

pub struct AllBitIterator {
    mask: u64,
    next: u64,
}

pub trait IntoAllBitIterator {
    fn iter_gosper(self, count: usize) -> AllBitIterator;
}

impl IntoAllBitIterator for u64 {
    /// Iterate over all the combination of bits set to 1 of size 'combo_size'.
    ///
    /// Uses a modified version of [gospers algorithm](https://programmingforinsomniacs.blogspot.com/2018/03/gospers-hack-explained.html).
    ///
    /// ### Steps
    /// 1. Find the rightmost 1-bit that can be moved left into a 0-bit. Move that 1-bit left one position.
    /// 2. Move all 1-bits that are to the right of that bit all the way to the right.
    ///
    ///
    fn iter_gosper(self, combo_size: usize) -> AllBitIterator {
        if combo_size > self.count_ones() as usize {
            panic!("combo_size ({combo_size}) larger than the amount of 1-bits {}", self.count_ones());
        }

        let current: u64 = self
            .iter_from_right()
            .take(combo_size)
            .fold(0, |acc, cur| acc | cur);

        AllBitIterator {
            mask: self,
            next: current,
        }
    }
}

impl Iterator for AllBitIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next;

        let mut cur_bit_p: Option<u64> = None;
        for (i, next_bit_p) in self.mask.iter_from_right().enumerate() {
            if let Some(cur_bit_p) = cur_bit_p {
                if cur_bit_p & self.next != 0 && next_bit_p & self.next == 0 {
                    // Here we know that bit_p is pointing to a bit in self.current that is filled,
                    // and we know that next_bit_p is pointing to the next bit in in self.current, and that it's empty.

                    // So we set next_bit_p to 1
                    self.next |= next_bit_p;
                    // and cur_bit_p to 0;
                    self.next ^= cur_bit_p;

                    let right_of_moved = self
                        .mask
                        .iter_from_right()
                        .take(i - 1)
                        .filter(|n| n & self.next != 0)
                        .count();

                    if right_of_moved > 0 {
                        for (i, n) in self.mask.iter_from_right().enumerate().take(i - 1) {
                            self.next |= n;
                            if i >= right_of_moved {
                                self.next ^= n;
                            }
                        }
                    }

                    return Some(current);
                }
            }
            cur_bit_p = Some(next_bit_p);
        }

        if current != 0 {
            self.next = 0;
            Some(current)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::engine::cards::Cards;
    use crate::engine::constants::*;

    use super::*;

    #[test]
    fn test_bit_shift() {
        let mut cursor: u64 = 1;
        let mut i: u32 = 0;

        while cursor > 0 && i < 65 {
            i += 1;
            cursor <<= 1;
            println!("{} {}", i, cursor)
        }
    }

    #[test]
    fn test_all_bit_iter() {
        let instant = Instant::now();

        for hand in FULL_DECK.iter_gosper(2) {
            for _ in (FULL_DECK ^ hand).iter_gosper(5) {}
            println!("{} {}", Cards::from(hand), instant.elapsed().as_secs_f32());
        }

        println!("total {}", instant.elapsed().as_secs_f32());
    }

    #[test]
    fn test_count() {
        let to_iter = FULL_DECK;
        let count = 5;

        let mask_count = to_iter.count_ones() as u128;
        let mask_factorial: u128 = ((mask_count - count as u128 + 1)..=mask_count).product();
        let count_factorial: u128 = (1..=count as u128).product();
        let total = mask_factorial / count_factorial;

        let iter = to_iter.iter_gosper(count);
        assert_eq!(total, iter.count() as u128);
    }

    #[test]
    fn test_perf() {
        let hand = (ACE & SPADE) | (ACE & HEART);
        let deck = FULL_DECK ^ hand;

        let mut instant = Instant::now();
        let mut i = 0u64;
        for table in deck.iter_gosper(5) {
            for _ in (deck ^ table).iter_gosper(2) {
                i += 1;
                if i % 10000000 == 0 {
                    println!("Elapsed {} {}", i, instant.elapsed().as_secs_f32());
                    instant = Instant::now();
                }
            }
        }

        println!("Done {}", instant.elapsed().as_secs_f32());
    }
}
