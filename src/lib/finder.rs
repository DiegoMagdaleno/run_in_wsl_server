extern crate dirs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::Path;

fn find_at_path(path: &str, mime_type: &str) -> Option<Vec<String>> {
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

pub fn get_capable_at_location(mime_type: &str) -> Option<HashMap<&str, Vec<String>>> {
    let mut matches_map: HashMap<&str, Vec<String>> = HashMap::new();

    matches_map.insert("global", vec![]);
    matches_map.insert("local", vec![]);

    let global_path = Path::new("/usr/share/applications/mimeinfo.cache");
    if global_path.exists() {
        let matches_at_global = find_at_path(global_path.as_os_str().to_str().unwrap(), mime_type);
        if let Some(capable) = matches_at_global {
            let matches_of_gbl = matches_map.get_mut("global");
            if let Some(m) = matches_of_gbl {
                m.extend(capable);
            }
        }
    }

    let format_path = format!("{}/.local/share/applications/mimeinfo.cache",dirs::home_dir().unwrap().to_str().unwrap());
    let local_path = Path::new(&format_path);
    if local_path.exists() {
        let matches_at_local = find_at_path(local_path.as_os_str().to_str().unwrap(), mime_type);
        if let Some(capable) = matches_at_local {
            let matches_of_lcl = matches_map.get_mut("local");
            if let Some(m) = matches_of_lcl {
                m.extend(capable)
            }
        }
    }

    if matches_map.get("local").unwrap().len() > 0 || matches_map.get("global").unwrap().len() > 0{
        Some(matches_map)
    } else {
        None
    }
}
