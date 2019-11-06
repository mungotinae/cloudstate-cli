use crate::builders::{ProjectBuilder, Application};
use std::path::Path;
use std::env;
use std::process::Command;
use crate::k8s_deploy;

pub struct GoBuilder;

impl ProjectBuilder for GoBuilder {

    fn pre_compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);
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
