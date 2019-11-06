use crate::builders::{ProjectBuilder, Application};
use crate::k8s_deploy;

use std::path::Path;
use std::{env, fs};
use std::process::Command;
use std::fs::File;
use std::io::Write;

pub struct GoBuilder;

impl ProjectBuilder for GoBuilder {

    fn pre_compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        // set dockerfile
        let docker_path = Path::new(&app.work_dir).join("Dockerfile");
        let docker_template_content = fs::read_to_string(docker_path.clone()).unwrap();

        let dockerfile = docker_template_content.replace("{application-name}", app.name.as_ref());

        let mut docker_file = File::create(docker_path).unwrap();
        docker_file.write_all(dockerfile.as_ref());
    }

    fn compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        println!("Downloading and install dependencies...");
        let status = Command::new("go")
            .arg("build")
            .arg(".")
            .status();

        if status.is_ok() {
            println!("Project successfully compiled");

        }
    }

    fn build(self, app: Application) {
        env::set_current_dir(&app.work_dir);

        let status = Command::new("docker")
            .arg("build")
            .arg("-t")
            .arg(format!("{}:{}", app.registry, app.tag))
            .arg(&app.work_dir)
            .status();

        if status.is_ok() {
            println!("Image created successfully!")
        }

    }

    fn push(self, app: Application) {
        env::set_current_dir(&app.work_dir);

        println!("Push container image...");
        let status = Command::new("docker")
            .arg("push")
            .arg(format!("{}:{}", app.registry, app.tag))
            .status();

        if status.is_ok() {
            println!("Pushed!");
        }
    }

    fn deploy(self, app: Application) {
        k8s_deploy(&app)
    }
}
