pub mod command {

    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::collections::HashMap;
    use crate::builders::{java::JavaBuilder, node::NodeBuilder, go::GoBuilder, dotnet::DotNetBuilder, rust::RustBuilder, python::PythonBuilder, scala::ScalaBuilder, ProjectBuilder, Application};
    use linked_hash_map::LinkedHashMap;
    use crate::{get_user_dir, get_templates, Emojis};

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

    pub fn destroy() {
        //kubectl delete all --all -n {namespace}
        println!("{} Destroying CloudState resources", Emojis::default().crying());
        let result = Command::new("kubectl")
            .arg("delete")
            .arg("all")
            .arg("--all")
            .arg("-n")
            .arg(CLOUD_STATE_NAMESPACE)
            .status();

        if result.is_ok() {
            println!("{} CloudState dead", Emojis::default().broken_heart());
        } else {
            println!("{} CloudState survivor", Emojis::default().stuck_out());
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
        let mut profiles = LinkedHashMap::new();
        profiles.insert("dotnet", "dotnet");
        profiles.insert("go", "go");
        profiles.insert("java", "java, [maven | sbt]");
        profiles.insert("node", "node");
        profiles.insert("python", "python, virtualenv");
        profiles.insert("rust", "rust, cargo");
        profiles.insert("scala", "java, scala, sbt");

        println!("{0: <10} | {1: <20} | {2: <10} | {3: <12} |", "Profile", "Dependencies", "Resolved", "Maturity Level");
        for (profile, dependencies) in &profiles {
            println!("{0: <10} | {1: <20} | {2: <10} | {3: <13} |", profile, dependencies, resolve_dependencies(profile), maturity_level(profile.clone()));
        }

        println!();
        println!("Subtitle:");
        println!("{} Stable for production usage", Emojis::default().stable());
        println!("{} Unstable but usable", Emojis::default().unstable());
        println!("{} Work in progress", Emojis::default().work_in_progress());
        println!("{} Unknown", Emojis::default().unknown());
    }

    fn maturity_level(profile: &str) -> char {
        match profile {
            "java"   => Emojis::default().stable(),
            "node"   => Emojis::default().stable(),
            "scala"  => Emojis::default().work_in_progress(),
            "go"     => Emojis::default().unstable(),
            "dotnet" => Emojis::default().work_in_progress(),
            "rust"   => Emojis::default().work_in_progress(),
            "python" => Emojis::default().work_in_progress(),
            _        => Emojis::default().unknown()
        }
    }

    fn resolve_dependencies(profile: &str) -> bool {
        match profile {
            "java"   => JavaBuilder{}.is_dependencies_ok(),
            "node"   => NodeBuilder{}.is_dependencies_ok(),
            "scala"  => ScalaBuilder{}.is_dependencies_ok(),
            "go"     => GoBuilder{}.is_dependencies_ok(),
            "dotnet" => DotNetBuilder{}.is_dependencies_ok(),
            "rust"   => RustBuilder{}.is_dependencies_ok(),
            "python" => PythonBuilder{}.is_dependencies_ok(),
            _        => false
        }
    }

    fn create_namespace(namespace: String) -> Result<(), String> {
        println!("{} Creating CloudState namespace...", Emojis::default().winking());
        if let result = Command::new("kubectl")
            .arg("create")
            .arg("namespace")
            .arg(namespace)
            .status()
            .is_ok() {

            println!("{} Success on create CloudState namespace", Emojis::default().smiling());
            return Ok(());
        };

        println!("{} Failure on create CloudState namespace", Emojis::default().screaming());
        return Err(String::from("Failure on create CloudState namespace"));
    }

    fn init_operator(namespace: String) -> Result<(), String> {
        println!("{} Initializing CloudState operator...", Emojis::default().rocket());
        if let result = Command::new("kubectl")
            .arg("apply")
            .arg("-n")
            .arg(namespace)
            .arg("-f")
            .arg(CLOUD_STATE_OPERATOR_DEPLOYMENT)
            .status()
            .is_ok() {

            println!("{} Success on installing CloudState operator", Emojis::default().success());
            return Ok(());
        };

        println!("{} Failure on installing CloudState operator", Emojis::default().crying());
        return Err(String::from("Failure on installing CloudState operator"))
    }
}