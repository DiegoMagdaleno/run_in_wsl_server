extern crate dirs;
use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn find_at_path(path: &str, mime_type: &str) -> Option<Vec<String>> {
    let mut line_match = String::new();

    let mut match_vector: Vec<String> = Vec::new();

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    /* Find the line in the cache that allows us to open said filetype */
    for line in reader.lines() {
        let raw_line = line.unwrap();
        if raw_line.contains(mime_type) {
            line_match = raw_line
        }
    }
    
    /* Split and push values to vector */
    if !line_match.is_empty() {
        let replace_pattern = format!("{}=", mime_type);
        line_match = line_match.replace(&replace_pattern, "");
        for entry in line_match.trim().split(";") {
            if !entry.is_empty() {
                match_vector.push(entry.to_string());  
            }
        }
        Some(match_vector)
    } else {
        None
    }
}
