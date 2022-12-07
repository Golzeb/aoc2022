use std::{
    cell::RefCell,
    fs::File,
    io::{BufRead, BufReader},
    rc::Rc,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Type {
    File,
    Directory,
}

#[derive(Clone)]
struct Node {
    name: String,
    parent: Option<Rc<RefCell<Node>>>,
    contents: Vec<Rc<RefCell<Node>>>,
    size: u32,
    node_type: Type,
}

impl Node {
    fn size(&self) -> u32 {
        self.size
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn parent(&self) -> Option<Rc<RefCell<Node>>> {
        self.parent.clone()
    }

    fn contents(&self) -> Vec<Rc<RefCell<Node>>> {
        self.contents.clone()
    }

    fn add_contents(&mut self, contents: Rc<RefCell<Node>>) {
        self.contents.push(contents);
    }

    fn get_type(&self) -> Type {
        self.node_type
    }
}

fn load_directory_structure(path: &'static str) -> Rc<RefCell<Node>> {
    let root: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
        name: "/".to_owned(),
        parent: None,
        contents: vec![],
        size: 0,
        node_type: Type::Directory,
    }));

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut current_node = root.clone();

    for line_result in reader.lines() {
        if let Ok(line) = line_result {
            let tokens = line.split(" ").collect::<Vec<&str>>();
            match tokens[0] {
                "$" => match tokens[1] {
                    "cd" => match tokens[2] {
                        ".." => {
                            current_node = match current_node.clone().borrow().parent() {
                                Some(addr) => addr.clone(),
                                None => current_node.clone(),
                            };
                        }
                        "/" => {
                            current_node = root.clone();
                        }
                        x => {
                            current_node = current_node
                                .clone()
                                .borrow()
                                .contents()
                                .iter()
                                .find(|&e| e.borrow().name() == x)
                                .unwrap()
                                .clone();
                        }
                    },
                    "ls" => {}
                    _ => {}
                },
                "dir" => {
                    current_node
                        .borrow_mut()
                        .add_contents(Rc::new(RefCell::new(Node {
                            name: tokens[1].to_owned(),
                            contents: vec![],
                            parent: Some(current_node.clone()),
                            size: 0,
                            node_type: Type::Directory,
                        })));
                }
                _ => {
                    let size = u32::from_str_radix(tokens[0], 10).unwrap();
                    current_node
                        .borrow_mut()
                        .add_contents(Rc::new(RefCell::new(Node {
                            name: tokens[1].to_owned(),
                            size,
                            parent: Some(current_node.clone()),
                            contents: vec![],
                            node_type: Type::File,
                        })));

                    let mut current_parent = Some(current_node.clone());
                    loop {
                        match current_parent.clone() {
                            Some(x) => {
                                x.borrow_mut().size += size;
                                current_parent = x.borrow().parent();
                            }
                            None => break,
                        }
                    }
                }
            }
        }
    }

    root
}

fn sum_of_at_most_100000(root: Rc<RefCell<Node>>) -> u32 {
    let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();
    stack.push(root.clone());

    let mut sum = 0;

    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        if top.borrow().get_type() == Type::Directory {
            for content in top.borrow().contents() {
                if content.borrow().node_type == Type::Directory {
                    stack.push(content.clone());
                }
            }
        }

        if top.borrow().size() <= 100000 {
            sum += top.borrow().size();
        }
        // println!("dir {} (size={})", top.borrow().name(), top.borrow().size());
    }

    sum
}

fn find_enough_smallest(root: Rc<RefCell<Node>>) -> u32 {
    let mut stack: Vec<Rc<RefCell<Node>>> = Vec::new();
    stack.push(root.clone());

    let space_needed = 30000000 - (70000000 - root.borrow().size());

    let mut least_enough = 0;
    let mut least_difference = u32::max_value();

    while !stack.is_empty() {
        let top = stack.pop().unwrap();
        if top.borrow().get_type() == Type::Directory {
            for content in top.borrow().contents() {
                if content.borrow().node_type == Type::Directory {
                    stack.push(content.clone());
                }
            }
        }

        if space_needed < top.borrow().size()
            && top.borrow().size() - space_needed < least_difference
        {
            least_difference = top.borrow().size() - space_needed;
            least_enough = top.borrow().size();
        }
    }

    least_enough
}

fn main() {
    let root = load_directory_structure("./day7/input.txt");

    println!(
        "Sum of directory sizes ofat most 100000 each: {}",
        sum_of_at_most_100000(root.clone())
    );

    println!(
        "Size of the smallest directory that is big enough: {}",
        find_enough_smallest(root.clone())
    );
}
