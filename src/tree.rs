use std::collections::HashMap;
use std::fs::File;
use std::io::Write;

pub struct CollatzTree {
    pub tree: HashMap<u128, (Option<u128>, Option<u128>)>,
    pub target: u128,
}

impl CollatzTree {
    pub fn new(n: u128) -> Self {
        let mut t = CollatzTree {
            tree: HashMap::new(),
            target: n,
        };
        t.build();
        t
    }

    fn build(&mut self) {
        self.tree.insert(1, (Some(2), None));
        let mut nodes_to_explore = vec![2];
        while let Some(node) = nodes_to_explore.pop() {
            if self.tree.contains_key(&node) {
                continue;
            }
            let p_1 = if node * 2 <= self.target {
                nodes_to_explore.push(node * 2);
                Some(node * 2)
            } else {
                None
            };
            let p_2 = if node % 6 == 4 {
                nodes_to_explore.push((node - 1) / 3);
                Some((node - 1) / 3)
            } else {
                None
            };
            self.tree.insert(node, (p_1, p_2));
        }
    }

    pub fn to_dot_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let filename: &str = &format!("tree_to_{}.dot", self.target);
        let mut file = File::create(filename)?;

        writeln!(&mut file, "digraph to_{} {{", self.target)?;
        writeln!(&mut file, "\trankdir=BT;")?;
        self.define_node_colors(&mut file)?;
        for (key, value) in self.tree.iter() {
            if let Some(p_1) = value.0 {
                writeln!(&mut file, "\t{} -> {:?};", key, p_1)?;
            }
            if let Some(p_2) = value.1 {
                writeln!(&mut file, "\t{} -> {:?};", key, p_2)?;
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
        let mut sorted_keys: Vec<_> = self.tree.keys().copied().collect();
        sorted_keys.sort();

        writeln!(file, "\t1 [color=gold, style=filled];")?;
        for key in sorted_keys.iter().skip(1) {
            if key % 2 == 0 {
                writeln!(file, "\t{} [color=green, style=filled];", key)?;
            } else {
                writeln!(file, "\t{} [color=gold, style=filled];", key)?;
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
        let mut expected = HashMap::new();
        expected.insert(1, (Some(2), None));
        expected.insert(2, (Some(4), None));
        expected.insert(4, (Some(8), Some(1)));
        expected.insert(8, (None, None));
        let built = CollatzTree::new(10).tree;
        assert_eq!(expected, built)
    }
}
