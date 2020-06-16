extern crate chrono;

use std::{fs, env, io, boxed::Box, vec::Vec, ffi::OsString, ffi::OsStr, path::Path, convert::AsRef};
use chrono::{Utc, DateTime};

#[derive(Debug)]
struct Node {
    name: OsString,
    last_modified: DateTime<Utc>,
    children: Vec<Node>,
    depth: u16
}

impl Node {
    fn new(name: OsString) -> Node {
        Node {
            name,
            last_modified: Utc::now(),
            children: Vec::new(),
            depth: 0
        }
    }
    fn add_child(&mut self, mut child: Node) {
        child.set_depth(self.depth + 1);
        self.children.push(child)
    }
    fn set_depth(&mut self, depth: u16) {
        self.depth = depth;
    }
    fn touch(&mut self) {
        self.last_modified = Utc::now();
    }
}
#[derive(Debug)]
struct DirTree {
    root: Node
}
impl DirTree {
    fn new(root: Node) -> DirTree {
        DirTree {
            root
        }
    }
}

fn traverse_from<T: AsRef<Path>>(entry_point: T) {
    if let Ok(dir_iter) = fs::read_dir(entry_point) {
        for entry in dir_iter {
            match entry {
                Ok(entry) => {
                    println!("{:?}", entry.file_name());
                    traverse_from(entry.path());
                },
                Err(_) => println!("sumn went wrong")
            }
        } 
    } 
}

fn construct_tree<T: AsRef<Path>>(entry_point: T) -> Node {
    let root = Node::new(
        OsStr::into_os_string(Box::new(*(entry_point.as_ref().file_name().unwrap())))
    );
    if let Ok(dir_iter) = fs::read_dir(entry_point) {
        for entry in dir_iter {
            match entry {
                Ok(entry) => {
                    root.add_child(construct_tree(entry.path()));
                },
                Err(_) => println!("sumn went wrong")
            }
        } 
    }
    root 

}
fn main() {
    // let dir_iter = fs::read_dir("../root").unwrap();
    // for entry in dir_iter {
    //     match entry {
    //         Ok(entry) => {
    //             let metadata = entry.metadata();
    //             println!("{:?}", entry);
    //         },
    //         Err(_) => println!("oops")
    //     }
    // }
    traverse_from("../root");
    // construct_tree()
}
// fn construct_tree() {
//     let mut root = Node::new();
//     let c1 = Node::new();
//     let c2 = Node::new();
//     root.add_child(c1);
//     root.add_child(c2);
//     let tree = DirTree::new(root);
//     println!("{:?}", tree);
// }
