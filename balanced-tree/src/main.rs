#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }
}

impl Tree {
    fn new() -> Self {
        Tree { root: None }
    }

    fn insert(&mut self, value: i32) {
        match &mut self.root {
            Some(node) => Tree::insert_rec(value, node),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    fn insert_rec(value: i32, node: &mut Box<Node>) {
        if value > node.value {
            match &mut node.right {
                Some(right) => Tree::insert_rec(value, right),
                None => node.right = Some(Box::new(Node::new(value))),
            }
        } else {
            match &mut node.left {
                Some(left) => Tree::insert_rec(value, left),
                None => node.left = Some(Box::new(Node::new(value))),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create() {
        let mut tree = Tree::new();
        tree.insert(5);
        tree.insert(3);
        tree.insert(7);
        tree.insert(1);
        tree.insert(4);
        tree.insert(6);
        tree.insert(8);

        assert_eq!(tree.root.is_some(), true);
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(1);
    tree.insert(4);
    tree.insert(6);
    tree.insert(8);

    println!("{:?}", tree);
}
