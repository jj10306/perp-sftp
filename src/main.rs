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
                    let metadata = entry.metadata().unwrap();
                    let last_modified = get_last_modified_time(&entry);
                    let path = entry.path();
                    println!("{:?}, {:?}", path, last_modified);
                    let path_string = pathbuf_to_string(path);
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
                    node.last_modified = get_last_modified_time(&entry);
                    node.add_child(construct_tree(path_string, depth + 1));
                },
                Err(_) => println!("sumn went wrong")
            }
        } 
    }
    node
}


fn traverse_tree<'a>(root: &'a Node) {
    for child in &root.children {
        println!("{:?}", &child);
        traverse_tree(&child);
    }
}
fn get_last_modified_time(entry: &fs::DirEntry) -> DateTime<Utc> {
    std::convert::From::from(entry.metadata().unwrap().modified().unwrap())
}
// converting PathBuf to String is verbose bc PathBuf isn't necessarily UTF-8 Encoded 
fn pathbuf_to_string(pb: PathBuf) -> String {
    pb.into_os_string().into_string().unwrap()
}

fn main() {
    println!("Traverse FS:");
    traverse_from("../root".to_string());
    let root = construct_tree("../root".to_string(), 0);
    let tree = DirTree::new(root);
    println!("Traverse Tree:");
    traverse_tree(&tree.root);
}