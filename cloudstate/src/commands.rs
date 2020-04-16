pub mod command {
    extern crate inflector;

    use crate::builders::{
        csharp::CSharpBuilder, go::GoBuilder, java::JavaBuilder, node::NodeBuilder,
        python::PythonBuilder, rust::RustBuilder, scala::ScalaBuilder, Application, ProjectBuilder,
    };
    use crate::{check_command, get_templates, get_user_dir, Emojis};
    use clap::ArgMatches;
    use inflector::Inflector;
    use linked_hash_map::LinkedHashMap;
    use std::fs;
    use std::path::Path;
    use std::process::Command;
    use std::io::{self, Write};

    const PROXY_PORT: &str = "9000";
    const FUNCTION_PORT: &str = "8080";
    const CLOUD_STATE_NAMESPACE: &str = "cloudstate";
    const CLOUDSTATE_PROXY_DEV_MODE: &str = "cloudstateio/cloudstate-proxy-native-dev-mode:latest";
    const CLOUD_STATE_OPERATOR_DEPLOYMENT: &str =
        "https://raw.githubusercontent.com/cloudstateio/cloudstate/master/operator/cloudstate.yaml";

    pub fn upgrade() {
        let status = self_update::backends::github::Update::configure()
            .repo_owner("sleipnir")
            .repo_name("cloudstate-cli")
            .bin_name("cloudstate")
            .show_download_progress(true)
            .current_version(env!("CARGO_PKG_VERSION"))
            .build()
            .unwrap()
            .update()
            .unwrap();

        println!("Update status: `{}`!", status.version());
    }

    pub fn scale(args: &ArgMatches) {
        //kubectl scale --replicas=3 deployment/shopping-cart
    }

    pub fn logs(args: &ArgMatches) {
        let application = args.value_of("name").unwrap();

        let space = match args.value_of("namespace") {
            Some(namespace) => args.value_of("namespace").unwrap_or(CLOUD_STATE_NAMESPACE),
            _ => CLOUD_STATE_NAMESPACE,
        };

        log_container(
            application,
            space,
            args.is_present("tail"),
            args.is_present("all"),
            args.is_present("since"),
            args.value_of("since").unwrap_or("1m"),
        );
    }

    pub fn run(args: &ArgMatches) {       
        if args.is_present("only-proxy") {
            runProxy(&args);
        } else {
            runAll(&args);
        }
    }

    pub fn check() {
        let mut commands = LinkedHashMap::new();

        commands.insert(
            "docker",
            format!(
                "{} Docker not found in system path",
                Emojis::default().bomb()
            ),
        );
        commands.insert(
            "kubectl",
            format!(
                "{} Kubectl not found in system path",
                Emojis::default().bomb()
            ),
        );
        commands.insert(
            "minikube",
            format!(
                "{} Minikube not found in system path",
                Emojis::default().bomb()
            ),
        );

        commands.insert("dotnet", "Dependency .NET not found in system path. If you use csharp please proceed to install it.".parse().unwrap());
        commands.insert(
            "go",
            "Dependency GO not found in system path. If you use GO please proceed to install it."
                .parse()
                .unwrap(),
        );
        commands.insert("java", "Dependency Java not found in system path. If you use Java please proceed to install it.".parse().unwrap());
        commands.insert("mvn", "Dependency Java not found in system path. If you use Java please proceed to install it.".parse().unwrap());

        commands.insert("npm", "Dependency NPM not found in system path. If you use NodeJS please proceed to install it.".parse().unwrap());
        commands.insert("python", "Dependency Python not found in system path. If you use Python please proceed to install it.".parse().unwrap());
        commands.insert("cargo", "Dependency Rust not found in system path. If you use Rust please proceed to install it.".parse().unwrap());
        commands.insert("sbt", "Dependency Sbt not found in system path. If you use Scala please proceed to install it.".parse().unwrap());

        for (command, expect) in &commands {
            let result = check_command(&command);

            if result.unwrap() == 0 {
                println!(
                    "{0: <1} Dependency {1: <10} OK!",
                    Emojis::default().ok(),
                    command.to_title_case()
                )
            } else {
                println!("{} {}", Emojis::default().nok(), expect);
            }
        }
    }

    pub fn init() {
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
        println!(
            "{} Destroying CloudState resources",
            Emojis::default().fire()
        );
        let result = Command::new("kubectl")
            .arg("delete")
            .arg("all")
            .arg("--all")
            .arg("-n")
            .arg(CLOUD_STATE_NAMESPACE)
            .status();

        if result.is_ok() {
            println!("{} Deleted all resources", Emojis::default().crying());
            let destroy_result = Command::new("kubectl")
                .arg("delete")
                .arg("namespace")
                .arg(CLOUD_STATE_NAMESPACE)
                .status();

            if destroy_result.is_ok() {
                println!("{} CloudState dead", Emojis::default().broken_heart());
            } else {
                println!("{} CloudState survivor", Emojis::default().stuck_out());
            }
        } else {
            println!("{} CloudState survivor", Emojis::default().stuck_out());
        }
    }

    pub fn build(app: Application) {
        // Retrive project configuration
        match app.profile.as_str() {
            "java" => JavaBuilder {}.build(app),
            "kotlin" => JavaBuilder {}.build(app),
            "node" => NodeBuilder {}.build(app),
            "go" => GoBuilder {}.build(app),
            "csharp" => CSharpBuilder {}.build(app),
            "rust" => RustBuilder {}.build(app),
            "python" => PythonBuilder {}.build(app),
            "scala" => ScalaBuilder {}.build(app),
            _ => println!("Invalid profile option"),
        }
    }

    pub fn push(app: Application) {
        // Retrive project configuration
        match app.profile.as_str() {
            "java" => JavaBuilder {}.push(app),
            "kotlin" => JavaBuilder {}.push(app),
            "node" => NodeBuilder {}.push(app),
            "go" => GoBuilder {}.push(app),
            "csharp" => CSharpBuilder {}.push(app),
            "rust" => RustBuilder {}.push(app),
            "python" => PythonBuilder {}.push(app),
            "scala" => ScalaBuilder {}.push(app),
            _ => println!("Invalid profile option"),
        }
    }

    pub fn deploy(app: Application) {
        // Retrive project configuration
        match app.profile.as_str() {
            "java" => JavaBuilder {}.deploy(app),
            "kotlin" => JavaBuilder {}.deploy(app),
            "node" => NodeBuilder {}.deploy(app),
            "go" => GoBuilder {}.deploy(app),
            "csharp" => CSharpBuilder {}.deploy(app),
            "rust" => RustBuilder {}.deploy(app),
            "python" => PythonBuilder {}.deploy(app),
            "scala" => ScalaBuilder {}.deploy(app),
            _ => println!("Invalid profile option"),
        }
    }

    pub fn create_project(app: Application) {
        let home_dir = get_user_dir();

        if !(Path::new(home_dir.as_str()).exists()) {
            println!("You must first boot CloudState with cloudstate --init. See cloudstate --help for help");
        } else {
            match app.profile.as_str() {
                "java" => JavaBuilder {}.create(app),
                "kotlin" => JavaBuilder {}.create(app),
                "node" => NodeBuilder {}.create(app),
                "go" => GoBuilder {}.create(app),
                "csharp" => CSharpBuilder {}.create(app),
                "rust" => RustBuilder {}.create(app),
                "python" => PythonBuilder {}.create(app),
                "scala" => ScalaBuilder {}.create(app),
                _ => println!("Invalid profile option"),
            }
        }
    }

    pub fn list_profiles() {
        let mut profiles = LinkedHashMap::new();
        profiles.insert("csharp", "dotnet");
        profiles.insert("go", "go");
        profiles.insert("java", "java, [maven | sbt]");
        profiles.insert("kotlin", "kotlin, [maven | gradle]");
        profiles.insert("node", "node");
        profiles.insert("python", "python, virtualenv");
        profiles.insert("rust", "rust, cargo");
        profiles.insert("scala", "java, scala, sbt");

        println!(
            "{0: <10} | {1: <20} | {2: <10} | {3: <12} |",
            "Profile", "Dependencies", "Resolved", "Maturity Level"
        );
        for (profile, dependencies) in &profiles {
            println!(
                "{0: <10} | {1: <20} | {2: <10} | {3: <13} |",
                profile,
                dependencies,
                resolve_dependencies(profile),
                maturity_level(profile.clone())
            );
        }

        println!();
        println!("Subtitle:");
        println!("{} Stable for production usage", Emojis::default().stable());
        println!("{} Unstable but usable", Emojis::default().unstable());
        println!("{} Work in progress", Emojis::default().work_in_progress());
        println!("{} Unknown", Emojis::default().unknown());
    }

    fn runAll(args: &ArgMatches) {
        let proxy_port = args.value_of("proxy-port").unwrap_or(PROXY_PORT);
        let function_port = args.value_of("function-port").unwrap_or(FUNCTION_PORT);
        let proxy_image = args.value_of("proxy-image").unwrap_or(CLOUDSTATE_PROXY_DEV_MODE);

        /*println!("Starting User container...");
        let output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--net=host")
            .arg("--name=proxy")
            .arg("--env")
            .arg(format!("HTTP_PORT={}", proxy_port))
            .arg("--env")
            .arg(format!("USER_FUNCTION_PORT={}", function_port))
            .arg(proxy_image)
            .output()
            .expect("Failed to execute user container");
        
        runProxy(args.clone());
        */
    }

    fn runProxy(args: &ArgMatches) {
        let proxy_port = args.value_of("proxy-port").unwrap_or(PROXY_PORT);
        let function_port = args.value_of("function-port").unwrap_or(FUNCTION_PORT);
        let proxy_image = args.value_of("proxy-image").unwrap_or(CLOUDSTATE_PROXY_DEV_MODE);

        println!("Running only proxy container");
        if args.is_present("show") {
            println!(
                "Command: docker run --rm --net=host --name proxy --env HTTP_PORT:{} --env USER_FUNCTION_PORT:{} {}",
                proxy_port, function_port, proxy_image
            );
        }
        
        println!("For stop press ctrl+c");

        let output = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("--net=host")
            .arg("--name=proxy")
            .arg("--env")
            .arg(format!("HTTP_PORT={}", proxy_port))
            .arg("--env")
            .arg(format!("USER_FUNCTION_PORT={}", function_port))
            .arg(proxy_image)
            .output()
            .expect("Failed to execute proxy container");
        
        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

    fn log_container(
        application: &str,
        namespace: &str,
        tail: bool,
        all_containers: bool,
        have_since: bool,
        since: &str,
    ) {
        let mut log = Command::new("kubectl");
        log.arg("logs");
        log.arg("-n");
        log.arg(namespace);
        log.arg("-l");
        log.arg(format!("user-container={}", application));

        if tail {
            log.arg("-f");
        }

        if all_containers {
            println!(
                "{} Get logs for {} and Sidecar containers",
                Emojis::default().magnifying_glass(),
                application
            );
            log.arg("--all-containers");
        } else {
            println!(
                "{} Get logs for {} container",
                Emojis::default().magnifying_glass(),
                application
            );
            log.arg("-c").arg("user-container");
        }

        if have_since {
            log.arg("--since").arg(since);
        }

        log.status();
    }

    fn maturity_level(profile: &str) -> char {
        match profile {
            "java" => Emojis::default().stable(),
            "kotlin" => Emojis::default().stable(),
            "node" => Emojis::default().stable(),
            "scala" => Emojis::default().work_in_progress(),
            "go" => Emojis::default().unstable(),
            "csharp" => Emojis::default().work_in_progress(),
            "rust" => Emojis::default().work_in_progress(),
            "python" => Emojis::default().work_in_progress(),
            _ => Emojis::default().unknown(),
        }
    }

    fn resolve_dependencies(profile: &str) -> bool {
        match profile {
            "java" => JavaBuilder {}.is_dependencies_ok(),
            "kotlin" => JavaBuilder {}.is_dependencies_ok(),
            "node" => NodeBuilder {}.is_dependencies_ok(),
            "scala" => ScalaBuilder {}.is_dependencies_ok(),
            "go" => GoBuilder {}.is_dependencies_ok(),
            "csharp" => CSharpBuilder {}.is_dependencies_ok(),
            "rust" => RustBuilder {}.is_dependencies_ok(),
            "python" => PythonBuilder {}.is_dependencies_ok(),
            _ => false,
        }
    }

    fn create_namespace(namespace: String) -> Result<(), String> {
        println!(
            "{} Creating CloudState namespace...",
            Emojis::default().winking()
        );
        if let result = Command::new("kubectl")
            .arg("create")
            .arg("namespace")
            .arg(namespace)
            .status()
            .is_ok()
        {
            println!(
                "{} Success on create CloudState namespace",
                Emojis::default().smiling()
            );
            return Ok(());
        };

        println!(
            "{} Failure on create CloudState namespace",
            Emojis::default().screaming()
        );
        return Err(String::from("Failure on create CloudState namespace"));
    }

    fn init_operator(namespace: String) -> Result<(), String> {
        println!(
            "{} Initializing CloudState operator...",
            Emojis::default().rocket()
        );
        if let result = Command::new("kubectl")
            .arg("apply")
            .arg("-n")
            .arg(namespace)
            .arg("-f")
            .arg(CLOUD_STATE_OPERATOR_DEPLOYMENT)
            .status()
            .is_ok()
        {
            println!(
                "{} Success on installing CloudState operator",
                Emojis::default().success()
            );
            return Ok(());
        };

        println!(
            "{} Failure on installing CloudState operator",
            Emojis::default().crying()
        );
        return Err(String::from("Failure on installing CloudState operator"));
    }
}
