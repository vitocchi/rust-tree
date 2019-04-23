extern crate rustree;
use rustree::tree::build_node;

fn main() {
    let mut root = build_node(1);
    root.add_left_child(build_node(2));
    root.add_right_child(build_node(3));
    let mut rroot = build_node(4);
    rroot.add_right_child(root);
    rroot.add_left_child(build_node(5));
    println!("{}", rroot);
}