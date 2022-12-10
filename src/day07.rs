use std::{collections::{HashMap}, rc::Rc, cell::RefCell};

use anyhow::Ok;

struct FsEntry {
    parent: Option<Rc<RefCell<FsEntry>>>,
    path: String,
    content: FsContent,
}

enum FsContent {
    File(usize),
    Dir(Vec<Rc<RefCell<FsEntry>>>)
}

fn parse_fs_tree(input: &str) -> Rc<RefCell<FsEntry>> {
    let mut path: Vec<String> = vec!["/".to_string()];

    let root_dir = Rc::new(RefCell::new(FsEntry {
        parent: None,
        path: "/".to_string(),
        content: FsContent::Dir(vec![]),
    }));

    let mut curr_dir = root_dir.clone();

    // TODO: refactor this...
    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        match parts[0] {
            "$" => {
                match parts[1] {
                    "ls" => {},
                    "cd" => {
                        match parts[2] {
                            ".." => {
                                path.pop();
                                let parent = {
                                    curr_dir.borrow().parent.clone().unwrap()
                                };
                                curr_dir = parent
                            },
                            dir => {
                                path.push(dir.to_string());
                                let new_entry = Rc::new(RefCell::new(FsEntry{
                                    parent: Some(curr_dir.clone()),
                                    path: path.join("/"),
                                    content: FsContent::Dir(vec![]),
                                }));
                                {
                                    let mut cur_dir_mut = curr_dir.borrow_mut();
                                    if let FsContent::Dir(children) = &mut cur_dir_mut.content {
                                        children.push(new_entry.clone());
                                    } else {
                                        panic!("unexpected thingy");
                                    }
                                }
                                curr_dir = new_entry;
                            }
                        }
                    },
                    cmd => unreachable!("undexpected command: {}", cmd)
                }
            },
            "dir" => {},
            num => {
                let size: usize = num.parse().expect("expected number");
                let new_entry = Rc::new(RefCell::new(FsEntry{
                    parent: Some(curr_dir.clone()),
                    path: path.join("/"),
                    content: FsContent::File(size),
                }));
                let mut cur_dir_mut = curr_dir.borrow_mut();
                if let FsContent::Dir(children) = &mut cur_dir_mut.content {
                    children.push(new_entry);
                } else {
                    panic!("unexpected thingy");
                }
            }
        }
    }
    root_dir
}

fn entry_size(fs_entry: Rc<RefCell<FsEntry>>, sizes: &mut HashMap<String, usize>) -> usize {
    let entry = fs_entry.borrow();

    match &entry.content {
        FsContent::File(size) => *size,
        FsContent::Dir(ch) => {
            let sum = ch.iter().map(|e| {
                entry_size(e.clone(), sizes)
            }).sum::<usize>();
            *sizes.entry(entry.path.clone()).or_insert(0) = sum;
            sum
        },
    }
}

pub fn task_1(input: &str) -> anyhow::Result<usize> {
    let root_dir = parse_fs_tree(input);

    let mut sizes: HashMap<String, usize> = HashMap::new();
    entry_size(root_dir, &mut sizes);

    let mut sum = 0;
    for (_k, v) in sizes {
        if v <= 100000 {
            sum +=v;
        }
    }

    Ok(sum)
}

pub fn task_2(input: &str) -> anyhow::Result<usize> {
    let root_dir = parse_fs_tree(input);

    let mut sizes: HashMap<String, usize> = HashMap::new();
    let root_size = entry_size(root_dir, &mut sizes);

    let update_space = 30000000;
    let av_space = 70000000 - root_size;
    let need_space = update_space - av_space;
    
    let mut min_ok_size = root_size;
    let mut min_diff = root_size - need_space;

    for (_k, v) in sizes {
        let diff = v as i64 - need_space as i64;
        if diff >= 0 && diff < min_diff as i64 {
            min_diff = diff as usize;
            min_ok_size = v;
        }
    }

    Ok(min_ok_size)
}

#[cfg(test)]
mod test {
    use super::task_1;
    use super::task_2;

    const INPUT: &str = r"$ cd /
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
7214296 k";

    #[test]
    fn test_1() {
        assert_eq!(task_1(INPUT).expect("failed to run 1"), 95437);
    }

    #[test]
    fn test_2() {
        assert_eq!(task_2(INPUT).expect("failed to run 2"), 24933642);
    }
}
