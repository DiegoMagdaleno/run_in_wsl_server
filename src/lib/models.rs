
#[derive(Debug)]
pub struct Application {
    name: String,
    exec: String,
    description: String,
}

impl Application {
    pub fn new(name: &str, exec: &str, desc: &str) -> Application {
        Application { name: name.to_string(), exec: exec.to_string(), description: desc.to_string()}
    }
}