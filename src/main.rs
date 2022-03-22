use colored::Colorize;
use std::fs;

fn to_kilobytes(length: f64) -> f64 {
    length / 1000 as f64
}

fn main() {
    for file in fs::read_dir("./").unwrap() {
        let unwrap = file.unwrap();
        let metadata = unwrap.metadata().unwrap();
        let file_size = metadata.len() as f64;
        let formatted = format!("{} ({})", format!("{}", unwrap.path().display()), format!("{:?} ko", to_kilobytes(file_size)));
        println!("{}", formatted.yellow());

        // Sort by weight
        // Display permissions
        // Owner + group ?
    }
}
