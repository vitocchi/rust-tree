use std::fmt;

#[derive(Debug)]
struct Node {
    value: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_tree(f, 0)
    }
}
impl Node {
    fn write_tree(&self, f: &mut fmt::Formatter, depth: u64) -> fmt::Result {
        if let Some(child) = &self.right {
            child.write_tree(f, depth + 1);
        }
        for x in 0..depth {
            write!(f, " ");
        }
        writeln!(f, "{}", self.value);
        if let Some(child) = &self.left {
            child.write_tree(f, depth + 1);
        }
        write!(f, "")
    }
    fn add_left_child(&mut self, child: Node) -> Result<(), String> {
        match &mut self.left {
            Some(_) => Err(String::from("right child already exists")),
            None => {
                self.left = Some(Box::new(child));
                Ok(())
            }
        }
    }
    fn add_right_child(&mut self, child: Node) -> Result<(), String> {
        match &mut self.right {
            Some(_) => Err(String::from("right child already exists")),
            None => {
                self.right = Some(Box::new(child));
                Ok(())
            }
        }
    }
    fn remove_left_child(&mut self) -> Result<(), String> {
        match &self.left {
            Some(_) => {
                self.left = None;
                Ok(())
            },
            None => {
                Err(String::from("left child does not exist"))
            }
        }
    }

    fn remove_right_child(&mut self) -> Result<(), String> {
        match self.right {
            Some(_) => {
                self.right = None;
                Ok(())
            },
            None => {
                Err(String::from("right child does not exist"))
            }
        }
    }
}

fn build_node(value: u64) -> Node {
    Node {
        value,
        left: None,
        right: None,
    }
}

fn main() {
    let mut root = build_node(1);
    root.add_left_child(build_node(2));
    root.add_right_child(build_node(3));
    let mut rroot = build_node(4);
    rroot.add_right_child(root);
    rroot.add_left_child(build_node(5));
    println!("{}", rroot);
}