use colored::Colorize;
use std::fs;

fn to_kilobytes(length: f64) -> f64 {
    length / 1000_f64
}

fn recursive_sum(path: String) -> f64 {
    let mut sum: f64 = 0.00;

    match fs::read_dir(&path) {
        Ok(v) => {
            for file in v {
                let unwrap = file.unwrap();

                match unwrap.file_name() {
                    // Allow to define different behaviour based on filename
                    _ => {
                        let metadata = unwrap.metadata().unwrap();
                        if metadata.file_type().is_dir() {
                            sum += recursive_sum(format!("{}", unwrap.path().display()))
                        } else {
                            sum += metadata.len() as f64;
                        }
                    }
                }
            }
        },
        Err(e) => println!("{}: {}", &path, e),
    }

    sum
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

        #[allow(unused_assignments)]
            let mut file_size = 0.00;

        if metadata.file_type().is_dir() {
            file_size = recursive_sum(format!("{}", unwrap.path().display()));

            files_found.push(FileFound {
                name: format!("{}/", unwrap.file_name().into_string().unwrap()),
                weight: to_kilobytes(file_size),
            })
        } else {
            file_size = metadata.len() as f64;
            files_found.push(FileFound {
                name: unwrap.file_name().into_string().unwrap(),
                weight: to_kilobytes(file_size),
            })
        }
    }

    files_found.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

    for file in files_found {
        if file.weight <= 10.0 {
            println!("{} ({})", file.name, format!("{}Kb", file.weight).green())
        } else if file.weight <= 1000.0 {
            println!("{} ({})", file.name, format!("{}Kb", file.weight).yellow())
        } else if file.weight > 1000.0 {
            println!("{} ({})", file.name, format!("{}Kb", file.weight).red())
        } else {
            println!("{} ({})", file.name, format!("{}Kb", file.weight).cyan())
        }
    }
}
