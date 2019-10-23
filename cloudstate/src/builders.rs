use std::process::Command;
use std::fs::File;

pub trait ProjectBuilder {
    fn build(self, name: &str);
}

pub struct RustBuilder;
pub struct JavaBuilder;
pub struct NodeBuilder;
pub struct GoBuilder;
pub struct DotNetBuilder;
pub struct PythonBuilder;
pub struct ScalaBuilder;

impl ProjectBuilder for RustBuilder {

    fn build(self, name: &str) {
        let status = Command::new("cargo")
            .arg("new")
            .arg("--bin")
            .arg(name)
            .status();

        if status.is_ok() {

            //TODO: Create Dockerfile
            println!("Creating Dockerfile");
            let mut file = File::create(name.to_owned() + "/" + "Dockerfile");

            //TODO: Create deployment.yml
            println!("Creating deployment.yml");
            let mut file = File::create(name.to_owned() + "/" + "deployment.yml");

            //TODO: Add CloudState Crate dependency

            println!("Project created!");
            Command::new("ls")
                .arg("-ltr")
                .arg(name)
                .spawn()
                .expect("Error during create Rust project");
        } else {
            println!("Error on create Rust project")
        }

    }
}

impl ProjectBuilder for JavaBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for NodeBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for GoBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for DotNetBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for PythonBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for ScalaBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}
