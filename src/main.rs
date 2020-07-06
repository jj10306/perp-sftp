extern crate chrono;

use std::{fs, vec::Vec, string::String, path::PathBuf};
use chrono::{Utc, DateTime};

#[derive(Debug)]
struct Node {
    name: String,
    last_modified: DateTime<Utc>,
    children: Vec<Node>,
    depth: u16
}

impl Node {
    fn new(name: String, depth: u16) -> Node {
        Node {
            name,
            last_modified: Utc::now(),
            children: Vec::new(),
            depth
        }
    }
    fn add_child(&mut self, child: Node) {
        self.children.push(child)
    }
    #[allow(dead_code)]
    fn touch(&mut self) {
        self.last_modified = Utc::now();
    }
}
#[derive(Debug)]
struct DirTree {
    root: Node
}
impl DirTree {
    #[allow(dead_code)]
    fn new(root: Node) -> DirTree {
        DirTree {
            root
        }
    }
}
#[allow(dead_code)]
fn traverse_from(entry_point: String) {
    if let Ok(dir_iter) = fs::read_dir(entry_point) {
        for entry in dir_iter {
            match entry {
                Ok(entry) => {
                    println!("{:?}", entry.file_name());
                    let path_string = pathbuf_to_string(entry.path());
                    traverse_from(path_string);
                },
                Err(_) => println!("sumn went wrong")
            }
        } 
    } 
}

fn construct_tree(root: String, depth: u16) -> Node {
    let mut node = Node::new(
        root.clone(),
        depth
    );
    if let Ok(dir_iter) = fs::read_dir(root.clone()) {
        for entry in dir_iter {
            match entry {
                Ok(entry) => {
                    let path_string = pathbuf_to_string(entry.path());
                    node.add_child(construct_tree(path_string, depth + 1));
                },
                Err(_) => println!("sumn went wrong")
            }
        } 
    }
    node

}
// converting PathBuf to String is verbose bc PathBuf isn't necessarily UTF-8 Encoded 
fn pathbuf_to_string(pb: PathBuf) -> String {
    pb.into_os_string().into_string().unwrap()
}

fn main() {
    let root = construct_tree("../root".to_string(), 0);
    println!("{:?}", root);
}