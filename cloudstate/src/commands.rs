
pub mod command {
    use std::process::Command;
    use std::fmt::Error;
    use std::collections::HashMap;

    const CLOUD_STATE_NAMESPACE: &str = "cloudstate";
    const CLOUD_STATE_OPERATOR_DEPLOYMENT: &str = "https://github.com/cloudstateio/cloudstate/releases/download/v0.4.3/cloudstate-0.4.3.yaml";

    pub fn init(){
        if let Ok(()) = create_namespace(CLOUD_STATE_NAMESPACE.parse().unwrap()) {
            init_operator(CLOUD_STATE_NAMESPACE.parse().unwrap());
        }
    }

    pub fn list_templates() {
        let mut templates = HashMap::new();
        templates.insert("java", "java, [maven | sbt]");
        templates.insert("node", "node");
        templates.insert("go", "go");
        templates.insert("dotnet", "dotnet");
        templates.insert("rust", "rust, cargo");
        templates.insert("python", "python, virtualenv");
        templates.insert("scala", "java, scala, sbt");

        println!("[Template Name]:[Dependencies]:[Resolved]");
        for (template, dependencies) in &templates {
            println!("[{}]:[{}]:[{}]", template, dependencies, resolve_dependencies(template));
        }
    }

    fn resolve_dependencies(template_name: &str) -> bool {
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