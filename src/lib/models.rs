
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

    pub fn get_name(&self) -> &str {
        return &self.name;
    }

    pub fn get_exec(&self) -> &str {
        return &self.exec;
    }

    pub fn get_desc(&self) -> &str {
        return &self.description;
    }
}