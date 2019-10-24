
pub mod command {
    use std::process::Command;
    use std::collections::HashMap;
    use crate::builders::*;

    const CLOUD_STATE_NAMESPACE: &str = "cloudstate";
    const CLOUD_STATE_OPERATOR_DEPLOYMENT: &str = "https://github.com/cloudstateio/cloudstate/releases/download/v0.4.3/cloudstate-0.4.3.yaml";

    pub fn init(){
        if let Ok(()) = create_namespace(CLOUD_STATE_NAMESPACE.parse().unwrap()) {
            init_operator(CLOUD_STATE_NAMESPACE.parse().unwrap());
        }
    }

    pub fn create_project(name: &str, idiom: &str) {
        match idiom {
            "java"   => JavaBuilder{}.build(name),
            "node"   => NodeBuilder{}.build(name),
            "go"     => GoBuilder{}.build(name),
            "dotnet" => DotNetBuilder{}.build(name),
            "rust"   => RustBuilder{}.build(name),
            "python" => PythonBuilder{}.build(name),
            "scala"  => ScalaBuilder{}.build(name),
            _        => println!("invalid")
        }

    }

    pub fn list_idioms() {
        let mut idioms = HashMap::new();
        idioms.insert("java", "java, [maven | sbt]");
        idioms.insert("node", "node");
        idioms.insert("go", "go");
        idioms.insert("dotnet", "dotnet");
        idioms.insert("rust", "rust, cargo");
        idioms.insert("python", "python, virtualenv");
        idioms.insert("scala", "java, scala, sbt");

        println!("[Idiom Name]:[Dependencies]:[Resolved]");
        for (idiom, dependencies) in &idioms {
            println!("[{}]:[{}]:[{}]", idiom, dependencies, resolve_dependencies(idiom));
        }
    }

    fn resolve_dependencies(idiom: &str) -> bool {
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