extern crate chrono;

use std::{fs, vec::Vec, string::String, path::PathBuf};
use std::collections::VecDeque;
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
    fn get_iter(&self) -> DirTreeIter {
        DirTreeIter::new(&self.root)
    }
}

struct DirTreeIter<'a> {
    q: VecDeque<&'a Node>
}
impl<'a> DirTreeIter<'a> {
    fn new(root: &'a Node) -> DirTreeIter<'a> {
        let mut q: VecDeque<&Node> = VecDeque::new();
        q.push_front(root);
        DirTreeIter {
            q
        }
    }
}
impl<'a> std::iter::Iterator for DirTreeIter<'a> {
    type Item = &'a Node;

    fn next(&mut self) -> Option<&'a Node> {
        if self.q.is_empty() {
             None
        } else {
            let rtn = self.q.pop_back().unwrap();
            for child in &rtn.children {
                self.q.push_front(child);
            }
            Some(rtn)
        }
    }
}
#[allow(dead_code)]
fn traverse_from(entry_point: String) {
    if let Ok(dir_iter) = fs::read_dir(entry_point) {
        for entry in dir_iter {
            match entry {
                Ok(entry) => {
                    let _metadata = entry.metadata().unwrap();
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
                    let child_path = pathbuf_to_string(entry.path());
                    let mut child_node = construct_tree(child_path, depth + 1);
                    child_node.last_modified = get_last_modified_time(&entry);
                    node.add_child(child_node);
                },
                Err(_) => println!("sumn went wrong")
            }
        } 
    }
    node
}


fn traverse_tree<'a>(root: &'a Node) {
    for child in &root.children {
        println!("{:?}, {:?}", child.name, child.last_modified);
        traverse_tree(&child);
    }
}
// manual level order for the tree
fn level_order<'a>(root: &'a Node) {
    let mut q: VecDeque<&Node> = VecDeque::new();
    q.push_front(root);
    while !q.is_empty() {
        let prev = q.pop_back().unwrap();
        println!("{:?}", prev.name);
        for child in &prev.children {
            q.push_front(child);
        }
    }
}
// use the DirIter
fn order_level(root: String) {
    let mut q: VecDeque<String> = VecDeque::new();
    q.push_front(root);
    while !q.is_empty() {
        let prev = q.pop_back().unwrap();
        println!("{:?}", prev);
        if let Ok(dir_iter) = fs::read_dir(prev) {
            for entry in dir_iter {
                let child_path = pathbuf_to_string(entry.unwrap().path());
                q.push_front(child_path);
            }
        } 
    }
    // if let Ok(mut dir_iter) = fs::read_dir(root) {
    //     while let Some(next) = dir_iter.next() {
    //         println!("{:?}", next);
    //     }
    // } else {
    //     println!("no iter here rip");
    // }
}



fn get_last_modified_time(entry: &fs::DirEntry) -> DateTime<Utc> {
    std::convert::From::from(entry.metadata().unwrap().modified().unwrap())
}
// converting PathBuf to String is verbose bc PathBuf isn't necessarily UTF-8 Encoded 
fn pathbuf_to_string(pb: PathBuf) -> String {
    pb.into_os_string().into_string().unwrap()
}

fn main() {
    // println!("Traverse FS:");
    // traverse_from("../root".to_string());
    let root = construct_tree("../root".to_string(), 0);
    let tree = DirTree::new(root);
    // println!("Level Order:");
    // level_order(&tree.root);
    // println!("Order Level:");
    // order_level("../root".to_string());
    let mut iter = tree.get_iter();
    // println!("Traverse Tree:");
    // traverse_tree(&tree.root);
    // loop {
    //     traverse_from("../root".to_string());
    //     println!("\nSleepy....\n");
    //     std::thread::sleep_ms(3000);
    // }

}

// Cases: 
// 1. Tree and FS are in same structure, but timestamps differ
// 2. Tree and FS are out of sync, need to update tree to reflect changes