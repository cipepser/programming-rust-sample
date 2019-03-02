// An orderd collection of `T`'s
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

// A part of a BinaryTree
struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn add(&mut self, value: T) {
        match *self {
            BinaryTree::Empty =>
                *self = BinaryTree::NonEmpty(Box::new(TreeNode {
                    element: value,
                    left: BinaryTree::Empty,
                    right: BinaryTree::Empty,
                })),
            BinaryTree::NonEmpty(ref mut node) =>
                if value <= node.element {
                    node.left.add(value);
                } else {
                    node.right.add(value);
                }
        }
    }
}

fn main() {
    use self::BinaryTree::*;
    let jupiter_tree = NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: Empty,
        right: Empty,
    }));

    let mercury_tree = NonEmpty(Box::new(TreeNode {
        element: "Mercury",
        left: Empty,
        right: Empty,
    }));

    let venus_tree = NonEmpty(Box::new(TreeNode {
        element: "Venus",
        left: Empty,
        right: Empty,
    }));

    let mars_tree = NonEmpty(Box::new(TreeNode {
        element: "Mars",
        left: jupiter_tree,
        right: mercury_tree,
    }));

    let uranus_tree = NonEmpty(Box::new(TreeNode {
        element: "Uranus",
        left: Empty,
        right: venus_tree,
    }));

    let saturn_tree = NonEmpty(Box::new(TreeNode {
        element: "Saturn",
        left: mars_tree,
        right: uranus_tree,
    }));

    // after learned the way in later chapter, we can write like below:
    let mut tree = BinaryTree::Empty;
    tree.add("Jupiter");
    tree.add("Mercury");
    tree.add("Venus");
    tree.add("Mars");
    tree.add("Uranus");
    tree.add("Saturn");
}
