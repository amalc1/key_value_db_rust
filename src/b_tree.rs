// Define the BST node
pub struct Node {
    pub key: i32,
    pub value: String,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn new(key: i32, value: String) -> Self {
        Node {
            key,
            value,
            left: None,
            right: None,
        }
    }
}

// Define the BST itself
pub struct BST {
    pub root: Option<Box<Node>>,
}

impl BST {
    pub fn new() -> Self {
        BST { root: None }
    }

    // Insert a key-value pair into the BST
    pub fn put(&mut self, key: i32, value: String) {
        let new_node = Box::new(Node::new(key, value));

        if let Some(ref mut node) = self.root {
            Self::insert_node(node, new_node);
        } else {
            self.root = Some(new_node);
        }
    }

    fn insert_node(current: &mut Box<Node>, new_node: Box<Node>) {
        if new_node.key < current.key {
            if let Some(ref mut left_node) = current.left {
                Self::insert_node(left_node, new_node);
            } else {
                current.left = Some(new_node);
            }
        } else if new_node.key > current.key {
            if let Some(ref mut right_node) = current.right {
                Self::insert_node(right_node, new_node);
            } else {
                current.right = Some(new_node);
            }
        } else {
            // Update the value if the key already exists
            current.value = new_node.value; 
        }
    }

    // Get a value by key
    pub fn get(&self, key: i32) -> Option<&String> {
        Self::search_node(&self.root, key)
    }

    fn search_node<'a>(current: &'a Option<Box<Node>>, key: i32) -> Option<&'a String> {
        match current {
            Some(node) => {
                if key < node.key {
                    Self::search_node(&node.left, key)
                } else if key > node.key {
                    Self::search_node(&node.right, key)
                } else {
                    Some(&node.value)
                }
            }
            None => None,
        }
    }

    // Print all key-value pairs in order
    pub fn print_in_order(&self) {
        Self::print_node_in_order(&self.root);
    }

    fn print_node_in_order(node: &Option<Box<Node>>) {
        if let Some(ref node) = node {
            Self::print_node_in_order(&node.left);
            println!("{}: {}", node.key, node.value);
            Self::print_node_in_order(&node.right);
        }
    }
}