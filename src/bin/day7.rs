use itertools::*;
use std::collections::HashMap;
enum Node {
    File(usize),
    Dir(HashMap<String, Node>),
}
fn directory_sizes(files: &HashMap<String, usize>) -> HashMap<String, usize> {
    let mut directories = HashMap::new();

    // Loop through each file
    for (file_path, file_size) in files {
        // Split the file path into its directory components
        let dir_path: Vec<&str> = file_path.split('/').collect();

        // Keep track of the current directory path and total size
        let mut cur_path = vec![];
        let mut cur_size = *file_size;

        // Loop through each directory component
        for (i, component) in dir_path.iter().enumerate() {
            // Check if the current component is the last element in the file path,
            // which indicates that it is the file name rather than a directory
            if i == dir_path.len() - 1 {
                break;
            }

            // Update the current directory path
            cur_path.push(*component);

            // Update the size of the current directory in the directories map
            let cur_dir = cur_path.join("/");
            let entry = directories.entry(cur_dir).or_insert(0);
            *entry += cur_size;
        }
    }

    directories
}

fn main() {
    let input = include_str!("../../data/day7.txt")
        .split("\n")
        .skip(1)
        .collect::<Vec<&str>>();
    let mut tree = HashMap::new();
    let mut current_address = vec![""];
    for line in input {
        if line.starts_with("$ cd ..") {
            current_address.pop();
        } else if line.starts_with("$ cd") {
            current_address.push(line.split("$ cd ").nth(1).unwrap());
        } else if line.starts_with("$ ls") {
            continue;
        } else if line.starts_with("dir") {
            continue;
        } else {
            // We have a file with some length
            let size = line.split(" ").nth(0).unwrap().parse::<usize>().unwrap();
            let file_name = line.split(" ").nth(1).unwrap();
            let path = current_address.join("/") + "/" + file_name;
            tree.insert(path, size);
        }
    }

    dbg!(&tree);
    // Figure out the size of each directory
    let dir_sizes: HashMap<String, usize> = directory_sizes(&tree).into_iter().collect();
    dbg!(&dir_sizes);
    dbg!(dir_sizes.values().filter(|f| *f <= &100000).sum::<usize>());
    // Part 2
    let free_space = 70000000 - dir_sizes.values().max().unwrap();
    let space_to_free = 30000000 - free_space;
    dbg!(space_to_free);
    let dir_clear_space = dir_sizes
        .values()
        .sorted()
        .skip_while(|f| **f < space_to_free)
        .next()
        .unwrap();
    dbg!(dir_clear_space);
}
