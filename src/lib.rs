pub mod tree {
    use std::fmt;

#[derive(Debug)]
pub struct Node {
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
    pub fn add_left_child(&mut self, child: Node) -> Result<(), String> {
        match &mut self.left {
            Some(_) => Err(String::from("right child already exists")),
            None => {
                self.left = Some(Box::new(child));
                Ok(())
            }
        }
    }
    pub fn add_right_child(&mut self, child: Node) -> Result<(), String> {
        match &mut self.right {
            Some(_) => Err(String::from("right child already exists")),
            None => {
                self.right = Some(Box::new(child));
                Ok(())
            }
        }
    }
    pub fn remove_left_child(&mut self) -> Result<(), String> {
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

    pub fn remove_right_child(&mut self) -> Result<(), String> {
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

pub fn build_node(value: u64) -> Node {
    Node {
        value,
        left: None,
        right: None,
    }
}
}