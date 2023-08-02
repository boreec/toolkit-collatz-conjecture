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

        self.hailstone.push(self.starting_number);

        let mut i = self.starting_number;
        while i != 1 {
            i = if i % 2 == 0 { i / 2 } else { 3 * i + 1 };
            self.hailstone.push(i);
        }
    }
}
