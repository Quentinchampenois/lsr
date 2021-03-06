use colored::Colorize;
use std::fs;
use human_bytes::human_bytes;
use std::os::unix::fs::PermissionsExt;
use std::env;
use std::fmt::Debug;

#[derive(Debug)]
struct FileFound {
    mode: u32,
    name: String,
    weight: f64,
}

impl FileFound {
    fn display_weight(&self) -> String {
        human_bytes(self.weight).parse().unwrap()
    }
    fn display(self) {
        if self.weight <= 1000.0 {
            println!("{} {} {}", unix_mode::to_string(self.mode), self.display_weight().green(), self.name)
        } else if self.weight <= 10000.0 {
            println!("{} {} {}", unix_mode::to_string(self.mode), self.display_weight().yellow(), self.name)
        } else if self.weight > 10000.0 {
            println!("{} {} {}", unix_mode::to_string(self.mode), self.display_weight().red(), self.name)
        } else {
            println!("{} {} {}", unix_mode::to_string(self.mode), self.display_weight().cyan(), self.name)
        }
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

fn target_directory(args: &[String]) -> &str {
    match args.len() {
        2 => {
            &args[1]
        }
        _ => {
            "."
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let target_path: &str = target_directory(&args);
    let mut files: Vec<FileFound> = vec![];

    let target_dir = match fs::read_dir(target_path) {
        Ok(file) => file,
        Err(e) => {
            println!("Unexpected error : {:?}", e);
            std::process::exit(1);
        }
    };

    for file in target_dir {
        let unwrap = file.unwrap();

        if unwrap.file_name() == "." || unwrap.file_name() == ".." {
            continue;
        }

        let metadata = unwrap.metadata().unwrap();

        let file_size: f64;
        let file_name: String;

        if metadata.file_type().is_dir() {
            file_size = recursive_sum(format!("{}", unwrap.path().display()));
            file_name = format!("{}/", unwrap.file_name().into_string().unwrap());
        } else {
            file_size = metadata.len() as f64;
            file_name = unwrap.file_name().into_string().unwrap();
        }

        files.push(FileFound {
            mode: metadata.permissions().mode(),
            name: file_name,
            weight: file_size,
        })
    }

    files.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap());

    for file in files { file.display() }
    std::process::exit(0);
}
