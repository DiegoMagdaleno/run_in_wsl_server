extern crate mime_guess;
mod lib {
    pub mod parser;
    pub mod finder;
    pub mod models;
}

fn main() {
    let matches = lib::finder::get_capable_at_location("text/plain");
    
    if let Some(x) = matches {
        if let Some(p) = lib::parser::parse_entries(x) {
            println!("{:#?}", p);
        }
    }
}
