use std::fs::File;
use std::io::Write;

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

    pub fn to_dot_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filename: &str = &format!("tree_to_{}.dot", self.target);
        let mut file = File::create(filename)?;

        writeln!(&mut file, "digraph to_{} {{", self.target)?;
        writeln!(&mut file, "\trankdir=BT;")?;
        self.define_node_colors(&mut file)?;
        for (i, branches) in self.adjacency_matrix.iter().enumerate() {
            writeln!(&mut file, "\t{} -> {};", i + 1, branches.0)?;
            if let Some(odd_number) = branches.1 {
                writeln!(&mut file, "\t{} -> {};", i + 1, odd_number)?;
            }
        }
        writeln!(&mut file, "}}")?;
        file.flush()?;
        Ok(())
    }

    fn define_node_colors(
        &self,
        file: &mut File,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for i in 0..self.adjacency_matrix.len() {
            if (i + 1) % 2 == 0 {
                writeln!(file, "{} [color=green, style=filled];", i + 1)?;
            } else {
                writeln!(file, "{} [color=gold, style=filled];", i + 1)?;
            }
        }

        for node in self.adjacency_matrix.iter() {
            if node.0 > self.adjacency_matrix.len() as u128 {
                writeln!(file, "{} [color=green, style=filled];", node.0)?;
            }
            if let Some(odd) = node.1 {
                if odd > self.adjacency_matrix.len() as u128 {
                    writeln!(file, "{} [color=gold, style=filled];", odd)?;
                }
            }
        }
        Ok(())
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
