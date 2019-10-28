
pub mod command {
    use git2::Repository;
    use std::process::Command;
    use std::collections::HashMap;
    use crate::builders::{java::JavaBuilder, node::NodeBuilder, go::GoBuilder, dotnet::DotNetBuilder, rust::RustBuilder, python::PythonBuilder, scala::ScalaBuilder, ProjectBuilder};
    use std::path::Path;
    use std::fs;

    const CLOUD_STATE_NAMESPACE: &str = "cloudstate";
    const CLOUD_STATE_LANGUAGE_TEMPLATES: &str = "https://github.com/sleipnir/cloudstate-templates.git";
    const CLOUD_STATE_OPERATOR_DEPLOYMENT: &str = "https://github.com/cloudstateio/cloudstate/releases/download/v0.4.3/cloudstate-0.4.3.yaml";

    pub fn init(){
        // First download templates
        println!("Downloading languages templates...");
        let home_dir = get_work_dir();

        if Path::new(home_dir.as_str()).exists() {
            fs::remove_dir_all(home_dir.clone());
        }

        get_templates(home_dir);

        if let Ok(()) = create_namespace(CLOUD_STATE_NAMESPACE.parse().unwrap()) {
            init_operator(CLOUD_STATE_NAMESPACE.parse().unwrap());
        }
    }

    pub fn build(path: &str) {
        // Retrive project configuration

    }

    pub fn create_project(name: &str, profile: &str) {
        let home_dir = get_work_dir();

        if Path::new(home_dir.as_str()).exists() {

            match profile {
                "java"   => JavaBuilder{}.build(name),
                "node"   => NodeBuilder{}.build(name),
                "go"     => GoBuilder{}.build(name),
                "dotnet" => DotNetBuilder{}.build(name),
                "rust"   => RustBuilder{}.build(name),
                "python" => PythonBuilder{}.build(name),
                "scala"  => ScalaBuilder{}.build(name),
                _        => println!("Invalid profile option")
            }
        } else {
            println!("You must first boot CloudState with cloudstate --init. See cloudstate --help for help")
        }

    }

    pub fn list_profiles() {
        let mut profiles = HashMap::new();
        profiles.insert("java", "java, [maven | sbt]");
        profiles.insert("node", "node");
        profiles.insert("go", "go");
        profiles.insert("dotnet", "dotnet");
        profiles.insert("rust", "rust, cargo");
        profiles.insert("python", "python, virtualenv");
        profiles.insert("scala", "java, scala, sbt");

        println!("[Idiom Name]:[Dependencies]:[Resolved]");
        for (profile, dependencies) in &profiles {
            println!("[{}]:[{}]:[{}]", profile, dependencies, resolve_dependencies(profile));
        }
    }

    fn get_work_dir() -> String  {
        dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.cloudstate"
    }

    fn get_templates(home_dir: String) -> Repository {
        let repo = match Repository::clone(CLOUD_STATE_LANGUAGE_TEMPLATES, home_dir) {
            Ok(repo) => repo,
            Err(e) => panic!("Failed to clone: {}", e),
        };

        repo
    }

    fn resolve_dependencies(profile: &str) -> bool {
        //TODO: resolve dependencies her
        true
    }

    fn create_namespace(namespace: String) -> Result<(), String> {
        println!("Creating CloudState namespace...");
        if let result = Command::new("kubectl")
            .arg("create")
            .arg("namespace")
            .arg(namespace)
            .spawn()
            .is_ok() {
            return Ok(());
        };

        return Err(String::from("Failure on create CloudState namespace"));
    }

    fn init_operator(namespace: String) -> Result<(), String> {
        println!("Initializing CloudState operator...");
        if let result = Command::new("kubectl")
            .arg("apply")
            .arg("-n")
            .arg(namespace)
            .arg("-f")
            .arg(CLOUD_STATE_OPERATOR_DEPLOYMENT)
            .spawn()
            .is_ok() {
            return Ok(());
        };

        return Err(String::from("Failure on installing CloudState operator"))
    }
}