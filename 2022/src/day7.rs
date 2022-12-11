use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;
use std::{fs, vec};

pub fn day7() -> (String, String) {
    let content = fs::read_to_string("input/day7").unwrap();
    run(content)
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Rc<RefCell<Folder>>>,
}

impl Folder {
    fn printstd(&self, depth: usize) {
        println!("{}- {} (dir)", " ".repeat(depth), self.name);
        for file in &self.files {
            file.printstd(depth + 1);
        }
        for folder in &self.folders {
            folder.borrow().printstd(depth + 1);
        }
    }
}

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

impl File {
    fn printstd(&self, depth: usize) {
        println!(
            "{}- {} (file, size={})",
            " ".repeat(depth),
            self.name,
            self.size
        );
    }
}

fn run(content: String) -> (String, String) {
    let lines: Vec<&str> = content.trim().split("\n").collect();

    let root = Rc::new(RefCell::new(Folder {
        name: "/".to_owned(),
        files: vec![],
        folders: vec![],
    }));

    // Parse the file structure
    let mut current = vec![Rc::clone(&root)];
    let mut i = 0;
    while i < lines.len() {
        if lines[i] == "$ cd /" {
            current = vec![Rc::clone(&root)];
        } else if lines[i] == "$ ls" {
            while i + 1 < lines.len() && !lines[i + 1].starts_with("$") {
                if lines[i + 1].starts_with("dir") {
                    let folder_name = lines[i + 1].split(" ").last().unwrap();

                    let folder = Folder {
                        name: folder_name.to_owned(),
                        files: vec![],
                        folders: vec![],
                    };

                    current
                        .last()
                        .unwrap()
                        .borrow_mut()
                        .folders
                        .push(Rc::new(RefCell::new(folder)));
                } else {
                    let mut splitted = lines[i + 1].split(" ");

                    let file_size = splitted.next().unwrap().parse::<usize>().unwrap();
                    let file_name = splitted.next().unwrap();

                    let file = File {
                        name: file_name.to_string(),
                        size: file_size,
                    };

                    current.last().unwrap().borrow_mut().files.push(file);
                };
                i += 1;
            }
        } else if lines[i].starts_with("$ cd ") {
            let folder_name = lines[i].split(" ").last().unwrap();
            if folder_name == ".." {
                current.pop();
            } else {
                let folder = Rc::clone(
                    current
                        .last()
                        .unwrap()
                        .borrow()
                        .folders
                        .iter()
                        .find_or_first(|folder| folder.borrow().name == folder_name)
                        .expect(&format!("Unable to find folder {}", folder_name)),
                );

                current.push(folder);
            }
        }
        i += 1;
    }

    root.borrow().printstd(0);

    let (total_used_size, part1) = calculate_part1(&root);

    let size_to_delete = 30000000 - (70000000 - total_used_size);
    println!("Size to delete: {}", size_to_delete);

    let (_, part2) = find_target_folder(&root, size_to_delete);

    println!("day7 part 1: {:?}", part1);
    println!("day7 part 2: {:?}", part2);

    (part1.to_string(), part2.to_string())
}

fn calculate_part1(node: &Rc<RefCell<Folder>>) -> (usize, usize) {
    let mut part1_result = 0;

    let mut folder_size = node.borrow().files.iter().map(|file| file.size).sum();

    for folder in &node.borrow().folders {
        let (sub_folder_size, sub_part1_result) = calculate_part1(folder);

        folder_size += sub_folder_size;
        part1_result += sub_part1_result;
    }

    if folder_size < 100000 {
        part1_result += folder_size;
    }

    (folder_size, part1_result)
}

fn find_target_folder(node: &Rc<RefCell<Folder>>, target_size: usize) -> (usize, usize) {
    let mut part2_result = usize::MAX;

    let mut folder_size = node.borrow().files.iter().map(|file| file.size).sum();

    for folder in &node.borrow().folders {
        let (sub_folder_size, sub_part2_result) = find_target_folder(folder, target_size);

        folder_size += sub_folder_size;
        part2_result = usize::min(part2_result, sub_part2_result);
    }

    if folder_size > target_size {
        part2_result = usize::min(part2_result, folder_size);
    }

    (folder_size, part2_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day7_test() {
        let (part1, part2) = day7();
        assert_eq!(part1, "1118405");
        assert_eq!(part2, "12545514");
    }

    #[test]
    fn day7_example_test() {
        let input = "
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
7214296 k"
            .to_string();

        let (part1, part2) = run(input);
        assert_eq!(part1, "95437");
        assert_eq!(part2, "24933642");
    }
}
