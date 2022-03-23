use colored::Colorize;
use std::fs;

fn to_kilobytes(length: f64) -> f64 {
    length / 1000_f64
}

#[derive(Debug)]
struct FileFound {
    name: String,
    weight: f64,
}

fn main() {
    let mut files_found: Vec<FileFound> = vec![];

    for file in fs::read_dir("./").unwrap() {
        let unwrap = file.unwrap();

        if unwrap.file_name() == "." || unwrap.file_name() == ".." {
            continue;
        }

        let metadata = unwrap.metadata().unwrap();

        if metadata.file_type().is_dir() {
            continue;
        }

        let file_size = metadata.len() as f64;

        files_found.push(FileFound {
            name: format!("{}", unwrap.path().display()),
            weight: to_kilobytes(file_size),
        })
    }

    files_found.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

    for file in files_found {
        if file.weight <= 10.0 {
            println!("{} ({})", file.name, format!("{} kb", file.weight).green())
        } else if file.weight <= 1000.0 {
            println!("{} ({})", file.name, format!("{} kb", file.weight).yellow())
        } else if file.weight > 1000.0 {
            println!("{} ({})", file.name, format!("{} kb", file.weight).red())
        } else {
            println!("{} ({})", file.name, format!("{} kb", file.weight).cyan())
        }
    }
}
