/*
--- Day 7: No Space Left On Device ---
You can hear birds chirping and raindrops hitting leaves as the expedition proceeds. Occasionally, you can even hear much louder sounds in the distance; how big do the animals get out here, anyway?

The device the Elves gave you has problems with more than just its communication system. You try to run a system update:

$ system-update --please --pretty-please-with-sugar-on-top
Error: No space left on device
Perhaps you can delete some files to make space for the update?

You browse around the filesystem to assess the situation and save the resulting terminal output (your puzzle input). For example:

$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
The filesystem consists of a tree of files (plain data) and directories (which can contain other directories or files). The outermost directory is called /. You can navigate around the filesystem, moving into or out of directories and listing the contents of the directory you're currently in.

Within the terminal output, lines that begin with $ are commands you executed, very much like some modern computers:

cd means change directory. This changes which directory is the current directory, but the specific result depends on the argument:
cd x moves in one level: it looks in the current directory for the directory named x and makes it the current directory.
cd .. moves out one level: it finds the directory that contains the current directory, then makes that directory the current directory.
cd / switches the current directory to the outermost directory, /.
ls means list. It prints out all of the files and directories immediately contained by the current directory:
123 abc means that the current directory contains a file named abc with size 123.
dir xyz means that the current directory contains a directory named xyz.
Given the commands and output in the example above, you can determine that the filesystem looks visually like this:

- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)
Here, there are four directories: / (the outermost directory), a and d (which are in /), and e (which is in a). These directories also contain files of various sizes.

Since the disk is full, your first step should probably be to find directories that are good candidates for deletion. To do this, you need to determine the total size of each directory. The total size of a directory is the sum of the sizes of the files it contains, directly or indirectly. (Directories themselves do not count as having any intrinsic size.)

The total sizes of the directories above can be found as follows:

The total size of directory e is 584 because it contains a single file i of size 584 and no other directories.
The directory a has total size 94853 because it contains files f (size 29116), g (size 2557), and h.lst (size 62596), plus file i indirectly (a contains e which contains i).
Directory d has total size 24933642.
As the outermost directory, / contains every file. Its total size is 48381165, the sum of the size of every file.
To begin, find all of the directories with a total size of at most 100000, then calculate the sum of their total sizes. In the example above, these directories are a and e; the sum of their total sizes is 95437 (94853 + 584). (As in this example, this process can count files more than once!)

Find all of the directories with a total size of at most 100000. What is the sum of the total sizes of those directories?
*/

use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, Read};
use std::path::PathBuf;
use std::rc::Rc;

fn read_input_file() -> String {
    let mut file = File::open("input.txt").unwrap();
    let mut file_content = String::new();
    file.read_to_string(&mut file_content);

    return file_content;
}

struct Tree {
    root: Rc<RefCell<dyn FsNode>>,
    map: HashMap<String, Rc<RefCell<NodeDir>>>,
}

impl<'a> Tree {
    fn new(root: Rc<RefCell<dyn FsNode>>) -> Tree {
        return Tree {
            root: root,
            map: HashMap::new(),
        };
    }
}

trait FsNode {
    fn is_dir(&self) -> bool;
    fn get_value(&self) -> u32;
    fn get_parent(&self) -> Option<Rc<RefCell<NodeDir>>>;
    fn as_any(&self) -> &dyn Any;
}

struct NodeDir {
    name: String,
    path2: std::path::PathBuf,
    path: String,
    childs: Vec<Rc<RefCell<dyn FsNode>>>,
    parent: Option<Rc<RefCell<NodeDir>>>,
}

impl NodeDir {
    fn new(name: &str, parent: Option<Rc<RefCell<NodeDir>>>) -> NodeDir {
        let mut path2 = std::path::PathBuf::from(name);
        let mut full_path = String::from("/");
        if (parent.is_some()) {
            let parent_path_buf = parent.as_ref().unwrap().borrow().path2.clone();
            full_path = String::from(
                parent_path_buf
                    .join(std::path::Path::new(name))
                    .to_str()
                    .unwrap(),
            );
            path2 = parent_path_buf.join(path2).to_path_buf();
        } else {
        }

        return NodeDir {
            name: String::from(name),
            childs: Vec::new(),
            path2: path2.to_path_buf(),
            parent: parent,
            path: full_path,
        };
    }
}

struct NodeFile {
    pub name: String,
    pub path2: PathBuf,
    pub path: String,
    pub size: u32,
    pub parent: Option<Rc<RefCell<NodeDir>>>,
}

impl FsNode for NodeDir {
    fn is_dir(&self) -> bool {
        return true;
    }

    fn get_value(&self) -> u32 {
        return self
            .childs
            .iter()
            .map(|x| {
                let x2 = &*x.borrow();
                return x2.get_value();
            })
            .sum();
    }

    fn get_parent(&self) -> Option<Rc<RefCell<NodeDir>>> {
        return self.parent.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> NodeFile {
    fn new(name: &str, size: u32, parent: Rc<RefCell<NodeDir>>) -> NodeFile {
        let mut path2 = std::path::PathBuf::from(name);
        let mut full_path = String::from("/");
        let parent_path_buf = parent.as_ref().borrow().path2.clone();
        full_path = String::from(
            parent_path_buf
                .join(std::path::Path::new(name))
                .to_str()
                .unwrap(),
        );
        path2 = parent_path_buf.join(path2).to_path_buf();

        return NodeFile {
            name: String::from(name),
            path2: path2,
            size: size,
            parent: Some(parent),
            path: full_path,
        };
    }
}
impl FsNode for NodeFile {
    fn is_dir(&self) -> bool {
        return false;
    }

    fn get_value(&self) -> u32 {
        return self.size;
    }

    fn get_parent(&self) -> Option<Rc<RefCell<NodeDir>>> {
        return self.parent.clone();
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn get_path(node: Rc<RefCell<NodeDir>>) -> String {
    return String::from(node.borrow().path.clone() + "/" + &node.borrow().name);
}

fn parse_ls_dir_line<'a>(line: &str, parent: Rc<RefCell<NodeDir>>) -> NodeDir {
    let mut split = line.split_whitespace();
    let dir_name = split.nth(1).unwrap();
    let new_node = NodeDir::new(dir_name, Some(parent));

    return new_node;
}

fn parse_ls_file_line<'a>(line: &str, parent: Rc<RefCell<NodeDir>>) -> NodeFile {
    let mut split = line.split_whitespace();
    let file_size = split.nth(0).unwrap().parse::<u32>().unwrap();
    let file_name = split.nth(0).unwrap();

    let new_node = NodeFile::new(file_name, file_size, parent);
    return new_node;
}

// fn read_dir_output<'a, T>(line_iter: &mut std::iter::Peekable<T>, node: Rc<RefCell<NodeDir>>)
// where
//     T: Iterator<Item = (usize, &'a str)>,
// {
//     return Rc::new(RefCell::new(NodeDir::new()));
// }

fn read_ls_output<'a, T>(
    line_iter: &mut std::iter::Peekable<T>,
    node: Rc<RefCell<NodeDir>>,
    tree: &mut Tree,
) where
    T: Iterator<Item = (usize, &'a str)>,
{
    while let Some((index, line)) = line_iter.next() {
        match line {
            x if x.starts_with("dir") => {
                let new_node = parse_ls_dir_line(&x, node.clone());
                let key = new_node.path.clone();
                let allo = Rc::new(RefCell::new(new_node));
                node.borrow_mut().childs.push(allo.clone());
                tree.map.insert(key, allo.clone());
            }
            _ => {
                let new_node = parse_ls_file_line(line, node.clone());
                node.borrow_mut()
                    .childs
                    .push(Rc::new(RefCell::new(new_node)));
            }
        }

        if let Some((_, peek_val)) = line_iter.peek() {
            if peek_val.starts_with("$") {
                break;
            }
        }
    }
}

fn parse_command_to_tree<'a>(input: &str) -> Rc<RefCell<NodeDir>> {
    let mut lines_iter = input.lines().into_iter().enumerate().peekable();
    lines_iter.next();
    let mut node = NodeDir::new("/", None);
    let rootNode = Rc::new(RefCell::new(node));
    let mut tree = Tree::new(rootNode.clone());
    let mut current_node = rootNode.clone();
    //let mut path_buf: String;
    while let Some((index, line)) = lines_iter.next() {
        println!("path: {}", current_node.borrow().path);
        match line {
            x if x.starts_with("$ cd") => {
                let folder_name = x.split_whitespace().nth(2).unwrap().trim();
                if (folder_name == "..") {
                    let tmp = current_node.clone();
                    current_node = tmp.borrow().get_parent().unwrap();
                } else {
                    let path_buf = make_path(folder_name, &current_node);
                    let key = path_buf.as_str();
                    let folder = match tree.map.get(key) {
                        Some(v) => v.clone(),
                        None => {
                            Rc::new(RefCell::new(NodeDir::new(folder_name, Some(current_node))))
                        }
                    };

                    tree.map.insert(path_buf, folder.clone());
                    current_node = folder.clone();
                }
            }
            x if x.starts_with("$ ls") => {
                read_ls_output(&mut lines_iter, current_node.clone(), &mut tree)
            }
            _ => println!("{} starts with something else", line),
        }
    }

    return rootNode.clone();
}

fn make_path(name: &str, parent: &Rc<RefCell<NodeDir>>) -> String {
    let path = std::path::PathBuf::from(name);
    let parent_path_buf = parent.as_ref().borrow().path2.clone();
    let final_path = parent_path_buf.join(path).to_path_buf();
    return final_path.to_str().unwrap().to_string();
}

// fn make_path_buf<'a>(name: &str, parent: &Rc<RefCell<NodeDir>>) -> &'a str {
//     let mut path2 = std::path::PathBuf::from(name);
//     let parent_path_buf = parent.as_ref().borrow().path2.clone();
//     path2 = parent_path_buf.join(path2).to_path_buf();
//     let path_str = path2.to_str().unwrap();
//     return &path_str;
// }

// fn make_path_buf<'a>(name: &str, parent: &Rc<RefCell<NodeDir>>) -> &'a str {
//     let mut path2 = std::path::PathBuf::from(name);
//     let parent_path_buf = parent.as_ref().borrow().path2.clone();
//     path2 = parent_path_buf.join(path2).to_path_buf();
//     return String::from(path2.to_str().unwrap()).as_str();
// }
/*
 fn traverse<F>(&self, f: &F, seen: &mut HashSet<&'static str>)
        where F: Fn(&'static str)
    {
        if seen.contains(&self.datum) {
            return;
        }
        f(self.datum);
        seen.insert(self.datum);
        for n in &self.edges {
            n.borrow().traverse(f, seen);
        }
    }

*/

fn traverse(n: Rc<RefCell<dyn FsNode>>, sum_all: &mut u32) {
    let node: &dyn FsNode = &*n.borrow();



    if (node.is_dir()) {
        let dir: &NodeDir = node
            .as_any()
            .downcast_ref::<NodeDir>()
            .expect("Should be dir");

        if(dir.get_value() <= 100000){
            *sum_all = *sum_all + dir.get_value();
        }

        println!("folderpath: {} {}", dir.path, dir.get_value());
        for c in &dir.childs {
            traverse(c.clone(), sum_all);
        }
    } else {
        let file: &NodeFile = node
            .as_any()
            .downcast_ref::<NodeFile>()
            .expect("should be file");
        println!("filepath: {} {}", file.path, file.size);
    }
}

// fn write_in_tree() {
//     let mut tree = Tree {
//         root: Rc::new(RefCell::new(NodeDir::new("/", None))),
//         map: HashMap::new(),
//     };

//     let file = std::fs::File::open("path/to/file.txt").unwrap();
//     let reader = std::io::BufReader::new(file);

//     for line in reader.lines() {
//         let line = line.unwrap();
//         let key = &line;
//         let value = Rc::new(RefCell::new(NodeDir::new(line.as_str(), None)));
//         tree.map.insert(key, value);
//     }
// }

fn main() {
    let content = read_input_file();
    let root = parse_command_to_tree(&content);
    let mut sum =0; 
    traverse(root, &mut sum);

    println!("answer: {}", sum);
}
