pub mod java;
pub mod node;
pub mod go;
pub mod rust;
pub mod dotnet;
pub mod python;
pub mod scala;

pub struct Application {
    name: String,
    tag: String,
    work_dir: String,
    profile: String,
    namespace: String,
    group_id: Option<String>,
    repo: Option<String>,
    datastore: String,
    port: u16,
}

impl Default for Application {

    fn default() -> Self {
        Application {
            name: "shopping-cart".to_string(),
            tag: "0.0.1".to_string(),
            work_dir: String::from(""),
            profile: String::from(""),
            namespace: String::from("cloudstate"),
            group_id: None,
            repo: None,
            datastore: String::from("InMemory"),
            port: 8088
        }
    }
}

pub trait ProjectBuilder {
    fn build(self, name: &str);
}