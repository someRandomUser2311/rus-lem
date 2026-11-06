#[derive(Debug, PartialEq)]
pub enum Tree {
    Empty,
    Node(i32, Box<Tree>, Box<Tree>),
}

pub fn balanced(values: &[i32]) -> Tree {
    if values.is_empty() {
        return Tree::Empty;
    }
    let m = (values.len() - 1) / 2;
    Tree::Node(
        values[m],
        Box::new(balanced(&values[..m])),
        Box::new(balanced(&values[m + 1..])),
    )
}

pub fn inorder(tree: &Tree) -> Vec<i32> {
    match tree {
        Tree::Empty => Vec::new(),
        Tree::Node(v, left, right) => {
            let mut out = inorder(left);
            out.push(*v);
            out.extend(inorder(right));
            out
        }
    }
}

pub fn height(tree: &Tree) -> usize {
    match tree {
        Tree::Empty => 0,
        Tree::Node(_, left, right) => 1 + height(left).max(height(right)),
    }
}
