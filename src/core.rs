#[derive(Debug)]
pub struct CollatzSequence {
    pub hailstone: Vec<u128>,

    /// The number from which the sequence starts.
    pub starting_number: u128,
}

impl CollatzSequence {
    pub fn new(starting_number: u128) -> Self {
        let mut collatz_sequence = CollatzSequence {
            hailstone: Vec::new(),
            starting_number,
        };
        collatz_sequence.compute();
        collatz_sequence
    }

    pub fn compute(&mut self) {
        if self.starting_number == 0 {
            return;
        }

        let mut i = self.starting_number;
        while i != 1 {
            self.hailstone.push(i);
            i = if i % 2 == 0 { i / 2 } else { 3 * i + 1 };
        }
        self.hailstone.push(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_sequence_compute() {
        let mut cs = CollatzSequence::new(1);
        assert_eq!(cs.hailstone.len(), 1);
        assert_eq!(cs.hailstone, vec![1]);

        cs = CollatzSequence::new(2);
        assert_eq!(cs.hailstone.len(), 2);
        assert_eq!(cs.hailstone, vec![2, 1]);

        cs = CollatzSequence::new(5);
        assert_eq!(cs.hailstone.len(), 6);
        assert_eq!(cs.hailstone, vec![5, 16, 8, 4, 2, 1]);
    }
}
