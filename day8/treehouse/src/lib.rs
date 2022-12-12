struct Tree {
    height: u8,
    is_visible: bool,
}

pub struct Forest {
    trees: Vec<Vec<Tree>>,
    rows: usize,
    columns: usize,
}

impl Forest {
    pub fn from(input_str: &str) -> Self {
        let mut rows = 0;
        let mut trees = vec![];

        for row in input_str.lines() {
            let mut tree_row = vec![];
            for tree in row.chars() {
                let height = tree.to_string().parse::<u8>().unwrap();
                tree_row.push(Tree {
                    height,
                    is_visible: false,
                });
            }
            trees.push(tree_row);
            rows += 1;
        }

        let columns = trees[0].len();

        Forest {
            trees,
            rows,
            columns,
        }
    }

    pub fn determine_visibility(&mut self) {
        for row_idx in 0..self.rows {
            let row = self.trees.get_mut(row_idx).unwrap();
            for col_idx in 0..self.columns {
                let mut tree = row.get_mut(col_idx).unwrap();

                if row_idx == 0 || row_idx == self.rows - 1 {
                    // If a tree is in the first row or last row, it is visible
                    tree.is_visible = true;
                    continue;
                }

                if col_idx == 0 || col_idx == self.columns - 1 {
                    // If a tree in in the first column or last column, it is visible
                    tree.is_visible = true;
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create_from_input() {
        let input_data: &'static str = "\
30373
25512
65332
33549
35390";

        let forest = Forest::from(input_data);

        for row in forest.trees {
            let mut line = String::new();
            for tree in row {
                let tree_str = format!("{}", tree.height);
                line.push_str(tree_str.as_str());
            }
            println!("{line}");
        }
    }

    #[test]
    fn visibility() {
        let input_data: &'static str = "\
30373
25512
65332
33549
35390";

        let mut forest = Forest::from(input_data);
        forest.determine_visibility();

        for row in forest.trees {
            let mut line = String::new();
            for tree in row {
                let tree_str = if tree.is_visible { "/\\" } else { "  " };
                line.push_str(tree_str);
            }
            println!("{line}");
        }
    }
}
