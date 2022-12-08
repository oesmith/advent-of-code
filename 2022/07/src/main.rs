use std::env;
use std::io;

struct Folder {
    name: String,
    parent: Option<usize>,
    subfolders: Vec<usize>,
    files: Vec<usize>,
    size: Option<usize>,
}

struct File {
    name: String,
    size: usize,
}

fn main() {
    let mut folders: Vec<Folder> = vec![];
    let mut files: Vec<File> = vec![];

    folders.push(Folder {
        name: "/".to_string(),
        parent: None,
        files: vec![],
        subfolders: vec![],
        size: None,
    });

    let mut pwd: usize = 0;

    let lines = io::stdin().lines();
    for line in lines {
        let line_str = line.unwrap();
        if line_str == "$ cd /" {
            pwd = 0;
        } if line_str == "$ cd .." {
            pwd = folders[pwd].parent.unwrap();
        } else if line_str.starts_with("$ cd ") {
            let next = folders.len();
            let dirname = line_str.get(5..).unwrap().to_string();
            folders[pwd].subfolders.push(next);
            folders.push(Folder {
                name: dirname,
                parent: Some(pwd),
                files: vec![],
                subfolders: vec![],
                size: None,
            });
            pwd = next;
        } else if line_str.starts_with(|c: char| c.is_ascii_digit()) {
            let (size_str, name) = line_str.split_once(' ').unwrap();
            let next = files.len();
            files.push(File {
                name: name.to_string(),
                size: size_str.parse().unwrap(),
            });
            folders[pwd].files.push(next);
        }
    }

    for folder in (0..folders.len()).rev() {
        folders[folder].size = Some(calc_folder_size(&folders, &files, folder));
    }

    if env::args().any(|x| x == "free") {
        let outer_size = folders[0].size.unwrap();
        let free = 70000000 - outer_size;
        let to_free = 30000000 - free;
        let best = folders.iter()
            .map(|f| f.size.unwrap())
            .filter(|&s| s >= to_free)
            .min();
        print!("Best: {}\n", best.unwrap());
    } else {
        let total = folders.iter()
            .map(|f| f.size.unwrap())
            .filter(|&s| s <= 100000)
            .sum::<usize>();
        print!("Total: {}\n", total);
    }
}

fn calc_folder_size(folders: &Vec<Folder>, files: &Vec<File>, index: usize) -> usize {
    return match folders[index].size {
        Some(i) => i,
        None => {
            let mut size: usize = 0;
            for &f in &folders[index].files {
                size += files[f].size;
            }
            let subfolders = &folders[index].subfolders;
            for &f in &folders[index].subfolders {
                size += calc_folder_size(folders, files, f);
            }
            size
        }
    }
}
