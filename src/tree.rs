pub struct CollatzTree {
    pub adjacency_matrix: Vec<(u128, Option<u128>)>,
    pub target: u128,
}

impl CollatzTree {
    pub fn new(n: u128) -> Self {
        let mut tree = CollatzTree {
            adjacency_matrix: vec![],
            target: n,
        };
        tree.build();
        tree
    }

    fn build(&mut self) {
        for i in 1..=self.target {
            self.adjacency_matrix.push((
                i * 2,                                             // even case
                if i % 6 == 4 { Some((i - 1) / 3) } else { None }, // odd case
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collatz_tree_build() {
        let expected = vec![
            (2, None),
            (4, None),
            (6, None),
            (8, Some(1)),
            (10, None),
            (12, None),
            (14, None),
            (16, None),
            (18, None),
            (20, Some(3)),
        ];
        let built = CollatzTree::new(10).adjacency_matrix;
        assert_eq!(expected, built)
    }
}
