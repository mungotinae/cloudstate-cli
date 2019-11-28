use crate::builders::{ProjectBuilder, Application};
use std::path::Path;
use std::process::{Command, Stdio};
use std::string::ToString;
use std::{env, fs};
use crate::{set_deployment_vars, k8s_deploy, Emojis};
use std::fs::File;
use std::io::Write;

use std::{thread, time};

pub struct CSharpBuilder;

impl ProjectBuilder for CSharpBuilder {

    fn is_dependencies_ok(&self) -> bool {
        Command::new("which")
            .arg("dotnet")
            .stdout(Stdio::null())
            .status().is_ok()
    }

    fn pre_compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        // Find and replace occurrences of {docker-image} and {tag} in deployment.yml
        set_deployment_vars(&app);
    }

    fn compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        println!("{} Downloading and install dependencies...", Emojis::default().floppy_disk());
        println!("{} Compiling project...", Emojis::default().coffee());
        let restore_status = Command::new("dotnet")
            .arg("restore")
            .status();
        
        let three_millis = time::Duration::from_millis(3000);
        thread::sleep(three_millis);

        if restore_status.is_ok() {

            let status = Command::new("dotnet")
            .arg("build")
            .status();

            if status.is_ok() {
                println!("{} Project successfully compiled", Emojis::default().ok())
            }

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
