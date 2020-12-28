extern crate mime_guess;
mod lib {
    pub mod parser;
    pub mod finder;
    pub mod models;
}

fn main() {
    if let Some(whatever) = mime_guess::from_ext("cock").first_raw() {
        if let Some(x) = lib::finder::get_capable_at_location(whatever) {
            if let Some(p) = lib::parser::parse_entries(x) {
                println!("{:#?}", p);
            }
        }
    } else {
        println!("No matches");
    }


}
