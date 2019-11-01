pub mod command {

    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::collections::HashMap;
    use crate::builders::{java::JavaBuilder, node::NodeBuilder, go::GoBuilder, dotnet::DotNetBuilder, rust::RustBuilder, python::PythonBuilder, scala::ScalaBuilder, ProjectBuilder, Application};

    use crate::{get_user_dir, get_templates};

    const CLOUD_STATE_NAMESPACE: &str = "cloudstate";
    const CLOUD_STATE_OPERATOR_DEPLOYMENT: &str = "https://raw.githubusercontent.com/cloudstateio/cloudstate/master/operator/cloudstate.yaml";

    pub fn init(){
        // First download templates
        let home_dir = get_user_dir();

        if Path::new(home_dir.as_str()).exists() {
            fs::remove_dir_all(home_dir.clone());
        }

        get_templates(home_dir);

        if let Ok(()) = create_namespace(CLOUD_STATE_NAMESPACE.parse().unwrap()) {
            init_operator(CLOUD_STATE_NAMESPACE.parse().unwrap());
        }
    }

    pub fn build(app: Application) {
        // Retrive project configuration
        match app.profile.as_str() {
            "java"   => JavaBuilder{}.build(app),
            "node"   => NodeBuilder{}.build(app),
            "go"     => GoBuilder{}.build(app),
            "dotnet" => DotNetBuilder{}.build(app),
            "rust"   => RustBuilder{}.build(app),
            "python" => PythonBuilder{}.build(app),
            "scala"  => ScalaBuilder{}.build(app),
            _        => println!("Invalid profile option")
        }

    }

    pub fn push(app: Application) {
        // Retrive project configuration
        match app.profile.as_str() {
            "java"   => JavaBuilder{}.push(app),
            "node"   => NodeBuilder{}.push(app),
            "go"     => GoBuilder{}.push(app),
            "dotnet" => DotNetBuilder{}.push(app),
            "rust"   => RustBuilder{}.push(app),
            "python" => PythonBuilder{}.push(app),
            "scala"  => ScalaBuilder{}.push(app),
            _        => println!("Invalid profile option")
        }

    }

    pub fn deploy(app: Application) {
        // Retrive project configuration
        match app.profile.as_str() {
            "java"   => JavaBuilder{}.deploy(app),
            "node"   => NodeBuilder{}.deploy(app),
            "go"     => GoBuilder{}.deploy(app),
            "dotnet" => DotNetBuilder{}.deploy(app),
            "rust"   => RustBuilder{}.deploy(app),
            "python" => PythonBuilder{}.deploy(app),
            "scala"  => ScalaBuilder{}.deploy(app),
            _        => println!("Invalid profile option")
        }

    }

    pub fn create_project(app: Application) {
        let home_dir = get_user_dir();

        if !(Path::new(home_dir.as_str()).exists()) {
            println!("You must first boot CloudState with cloudstate --init. See cloudstate --help for help");
        } else {

            match app.profile.as_str() {
                "java"   => JavaBuilder{}.create(app),
                "node"   => NodeBuilder{}.create(app),
                "go"     => GoBuilder{}.create(app),
                "dotnet" => DotNetBuilder{}.create(app),
                "rust"   => RustBuilder{}.create(app),
                "python" => PythonBuilder{}.create(app),
                "scala"  => ScalaBuilder{}.create(app),
                _        => println!("Invalid profile option")
            }

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

            println!("Success on create CloudState namespace");
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

            println!("Success on installing CloudState operator");
            return Ok(());
        };

        return Err(String::from("Failure on installing CloudState operator"))
    }
}