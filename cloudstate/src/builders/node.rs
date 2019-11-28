use crate::builders::{ProjectBuilder, Application};
use std::path::Path;
use std::process::{Command, Stdio};

use std::{env, fs};
use crate::{set_deployment_vars, k8s_deploy, Emojis};
use std::fs::File;
use std::io::Write;

pub struct NodeBuilder;

impl ProjectBuilder for NodeBuilder {

    fn is_dependencies_ok(&self) -> bool {
        Command::new("which")
            .arg("npm")
            .stdout(Stdio::null())
            .status().is_ok()
    }

    fn pre_compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        // Find and replace occurrences of {docker-image} and {tag} in deployment.yml
        set_deployment_vars(&app);

        // set dockerfile
        let docker_path = Path::new(&app.work_dir).join("Dockerfile");
        let docker_template_content = fs::read_to_string(docker_path.clone()).unwrap();

        let dockerfile = docker_template_content.replace("{application-name}", app.name.as_ref());

        let mut docker_file = File::create(docker_path).unwrap();
        docker_file.write_all(dockerfile.as_ref());
    }

    fn compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        println!("{} Downloading and install dependencies...", Emojis::default().floppy_disk());
        println!("{} Compiling project...", Emojis::default().coffee());
        let status = Command::new("npm")
            .arg("install")
            .status();

        if status.is_ok() {
            println!("{} Project successfully compiled", Emojis::default().ok())
        }
    }

    fn build(self, app: Application) {
        println!("{} Building Project...", Emojis::default().worker());
        env::set_current_dir(&app.work_dir);
        self.compile(&app);

        let status = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(format!("{}:{}", app.registry, app.tag))
            .arg(&app.work_dir)
            .status();

        if status.is_ok() {
            println!("{} Image created successfully!", Emojis::default().frame_picture())
        }
    }

    fn push(self, app: Application) {
        env::set_current_dir(&app.work_dir);

        println!("{} Push container image...", Emojis::default().rocket());
        let status = Command::new("docker")
            .arg("push")
            .arg(format!("{}:{}", app.registry, app.tag))
            .status();

        if status.is_ok() {
            println!("{} Pushed!", Emojis::default().moon());
        }
    }

    fn deploy(self, app: Application) {
        k8s_deploy(&app)
    }
}