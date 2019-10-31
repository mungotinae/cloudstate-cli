extern crate dirs;
extern crate tar;
extern crate flate2;
extern crate cargo_toml_builder;
extern crate throw;

use std::path::Path;
use std::process::Command;
use std::string::ToString;

use std::{env, fs};
use std::fs::File;
use std::io::Write;

use crate::builders::{ProjectBuilder, Application};

pub struct JavaBuilder;

impl ProjectBuilder for JavaBuilder {

    fn pre_compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        // Find and replace occurrences of {docker-image} and {tag} in deployment.yml
        JavaBuilder::set_deployment_vars(&app);

        // Find and replace occurrences of {application-name}, {application-version} in pom.xml
        JavaBuilder::set_pom_vars(&app);

    }

    fn compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        println!("Downloading and install dependencies...");
        let install_status = Command::new("mvn")
            .arg("install")
            .status();

        println!("Compiling project...");
        if install_status.is_ok() {
            println!("Project successfully compiled")
        };

    }

    fn pos_compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        println!("Project created!");
        Command::new("ls")
            .arg("-ltr")
            .spawn()
            .expect("Error during create Java project");

        println!("Open editor!");
        let cached = dirs::home_dir().unwrap();
        Command::new("code")
            .arg(".")
            .arg(format!("--user-data-dir={}", cached.as_path().display().to_string()))
            .status()
            .expect("Error on open code editor");

        Command::new("idea")
            .arg(".")
            .status()
            .expect("Error on open code editor");

    }

    fn build(self, path: &Path, app: Application) {
        unimplemented!()
    }
}

impl JavaBuilder {
    fn set_deployment_vars(app: &&Application) {
        let deployment_path = Path::new(&app.work_dir).join("deployment.yml");
        let deployment_template_content = fs::read_to_string(deployment_path.clone()).unwrap();

        let image_name = &app.repo;

        let deployment_name = deployment_template_content.replace("{application-name}", app.name.as_ref());
        let deployment_image = deployment_name.replace("{image-name}", image_name.as_str());
        let deployment_content = deployment_image.replace("{tag}", app.tag.as_ref());
        let mut deployment_file = File::create(deployment_path).unwrap();
        deployment_file.write_all(deployment_content.as_ref());
    }

    fn set_pom_vars(app: &&Application) {
        let pom_path = Path::new(&app.work_dir).join("pom.xml");
        let pom_template_content = fs::read_to_string(pom_path.clone()).unwrap();

        let name = format!("<artifactId>{}</artifactId>", app.name);
        let pom_name = pom_template_content.replace("<artifactId>{application-name}</artifactId>", name.as_ref());

        let version = format!("<version>{}</version>", app.tag);
        let pom_content = pom_name.replace("<version>{tag}</version>", version.as_ref());
        let mut pom_file = File::create(pom_path).unwrap();
        pom_file.write_all(pom_content.as_ref());
    }
}
