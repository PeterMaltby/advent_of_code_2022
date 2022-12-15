use std::collections::HashMap;
use std::fs::File as FsFile;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Directory {
    parent: Option<Rc<Directory>>,
    name: String,
    sub_directories: RefCell<HashMap<String, Rc<Directory>>>,
    files: RefCell<HashMap<String, u32>>,
}

impl Directory {
    fn print(&self) {
        
        println!("{} directory contains {{",self.name);
        println!("{:?}",self.files.borrow());
        for sub_dir in self.sub_directories.borrow().values() {
            sub_dir.as_ref().print();
        }
        println!("}}-----------");
        
    }
}

pub fn day7_1() {
    let filename = "day7/testInput.txt";

    let file = FsFile::open(filename).unwrap();
    let reader = BufReader::new(file);

    let root = Rc::new(Directory {
        parent: None,
        name: String::from("/"),
        sub_directories: RefCell::new(HashMap::new()),
        files: RefCell::new(HashMap::new()),
    });

    let mut active_dir = Rc::clone(&root);

    for line in reader.lines() {
        let line = line.unwrap();
        let words = line.split(' ').collect::<Vec<&str>>();

        match (words[0], words[1]) {
            ("$", "ls") => {
                println!("ls so ignore?");
            }
            ("$", "cd") => {
                match words[2] {
                    "/" => active_dir = Rc::clone(&root),
                    ".." => active_dir = Rc::clone(&active_dir.parent.as_ref().unwrap()),
                    goto_dir => {
                        let new_active_dir = Rc::clone(&active_dir.sub_directories.borrow().get(goto_dir).unwrap());
                        active_dir = new_active_dir;
                    }
                };
            }
            ("dir", new_dir_name) => {
                let new_dir = Rc::new(Directory {
                        parent: Some(Rc::clone(&active_dir)),
                        name: new_dir_name.to_string(),
                        sub_directories: RefCell::new(HashMap::new()),
                        files: RefCell::new(HashMap::new()),
                    });

                active_dir.sub_directories.borrow_mut().insert(new_dir_name.to_string(), new_dir);

            }
            (filesize, filename) => {
                active_dir.files.borrow_mut().insert(filename.to_string(), filesize.parse().unwrap());
            }
        }
    }

    root.print();


    println!("not done yet");
}

pub fn day7_2() {
    let filename = "day7/input.txt";

    println!("not done yet!");
}
