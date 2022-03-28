use colored::Colorize;
use std::fs;
use human_bytes::human_bytes;
use std::os::unix::fs::PermissionsExt;

#[derive(Debug)]
struct FileFound {
    mode: u32,
    name: String,
    weight: f64,
    weight_readable: String,
}

fn print_file(file: FileFound) {
    if file.weight <= 1000.0 {
        println!("{} {} {}", unix_mode::to_string(file.mode), file.weight_readable.to_string().green(), file.name)
    } else if file.weight <= 10000.0 {
        println!("{} {} {}", unix_mode::to_string(file.mode), file.weight_readable.to_string().yellow(), file.name)
    } else if file.weight > 10000.0 {
        println!("{} {} {}", unix_mode::to_string(file.mode), file.weight_readable.to_string().red(), file.name)
    } else {
        println!("{} {} {}", unix_mode::to_string(file.mode), file.weight_readable.to_string().cyan(), file.name)
    }
}

fn recursive_sum(path: String) -> f64 {
    let mut sum: f64 = 0.00;

    match fs::read_dir(&path) {
        Ok(v) => {
            for file in v {
                let unwrap = file.unwrap();

                let metadata = unwrap.metadata().unwrap();
                if metadata.file_type().is_dir() {
                    sum += recursive_sum(format!("{}", unwrap.path().display()))
                } else {
                    sum += metadata.len() as f64;
                }
            }
        }
        Err(e) => println!("{}: {}", &path, e),
    }

    sum
}

fn main() {
    let mut files_found: Vec<FileFound> = vec![];

    for file in fs::read_dir("./").unwrap() {
        let unwrap = file.unwrap();

        if unwrap.file_name() == "." || unwrap.file_name() == ".." {
            continue;
        }

        let metadata = unwrap.metadata().unwrap();

        #[allow(unused_assignments)]
            let mut file_size = 0.00;

        if metadata.file_type().is_dir() {
            file_size = recursive_sum(format!("{}", unwrap.path().display()));

            files_found.push(FileFound {
                mode: metadata.permissions().mode(),
                name: format!("{}/", unwrap.file_name().into_string().unwrap()),
                weight: file_size,
                weight_readable: human_bytes(file_size).parse().unwrap(),
            })
        } else {
            file_size = metadata.len() as f64;

            files_found.push(FileFound {
                mode: metadata.permissions().mode(),
                name: unwrap.file_name().into_string().unwrap(),
                weight: file_size,
                weight_readable: human_bytes(file_size).parse().unwrap(),
            })
        }
    }

    files_found.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

    for file in files_found { print_file(file) }
}
