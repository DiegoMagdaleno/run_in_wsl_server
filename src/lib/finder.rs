extern crate dirs;
use std::fs::File;
use std::io::{prelude::*, BufReader};


pub fn find_at_path(path: &str, mime_type: &str) -> Option<Vec<String>> {
    let mut our_matches: Vec<String> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let raw_line = line.unwrap();
        if raw_line.contains(mime_type) {
            our_matches.push(raw_line)
        }
    }

    if our_matches.len() > 0 {
        Some(our_matches)
    } else {
        None 
    }

}