use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn file_lines_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).expect("no such file");

    BufReader::new(file)
        .lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
