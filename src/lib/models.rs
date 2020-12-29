
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

pub fn load(applications_serialized: Vec<Application>) -> Vec<crate::server::server::runinwsl_proto::FileReply> {
    applications_serialized.into_iter()
    .map(|app| crate::server::server::runinwsl_proto::FileReply {
        name: app.get_name().to_string(),
        exec: app.get_exec().to_string(),
        description: app.get_desc().to_string(),
    }).collect()
}