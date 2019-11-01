extern crate dirs;
extern crate tar;
extern crate flate2;
extern crate cargo_toml_builder;
extern crate throw;

use std::path::Path;
use std::process::{Command, ExitStatus};
use std::string::ToString;

use std::{env, fs, io};
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
        let install_status = JavaBuilder::install();

        println!("Compiling project...");
        if install_status.is_ok() {
            println!("Project successfully compiled")
        };

    }

    fn build(self, app: Application) {
        println!("Building Project...");
        env::set_current_dir(&app.work_dir);

        let install_status = JavaBuilder::install();

        println!("Compiling project...");
        if install_status.is_ok() {
            println!("Project successfully compiled");
        };

    }

    fn push(self, app: Application) {
        env::set_current_dir(&app.work_dir);

        let push_status = JavaBuilder::push(&app);

        println!("Push container image...");
        if push_status.is_ok() {
            println!("Pushed!");
        };
    }

    fn deploy(self, app: Application) {
        env::set_current_dir(&app.work_dir);

        let result = Command::new("kubectl")
            .arg("apply")
            .arg("-n")
            .arg(&app.namespace)
            .arg("-f")
            .arg("deployment.yml")
            .spawn();

        if result.is_ok() {
            println!("Success on installing 'User Function' {} in namespace: {}", &app.name, &app.namespace);
        } else {
            panic!("Error on installing 'User Function' {} in namespace: {}", &app.name, &app.namespace);
        }

    }
}


impl JavaBuilder {
    fn set_deployment_vars(app: &&Application) {
        let deployment_path = Path::new(&app.work_dir).join("deployment.yml");
        let deployment_template_content = fs::read_to_string(deployment_path.clone()).unwrap();

        let image_name = &app.registry;

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
        let repo_name = format!("<repo.name>{}</repo.name>", app.registry);
        let tag_version = format!("<tag.version>{}</tag.version>", app.tag);

        let repo = pom_name.replace("<repo.name>{repo}</repo.name>", repo_name.as_ref());
        let tag = repo.replace("<tag.version>{tag}</tag.version>", tag_version.as_ref());
        let pom_content = tag.replace("<version>{tag}</version>", version.as_ref());
        let mut pom_file = File::create(pom_path).unwrap();
        pom_file.write_all(pom_content.as_ref());
    }

    fn install() -> io::Result<ExitStatus> {
        //TODO: Rewrite pom.xml with tag specified
        let install_status = Command::new("mvn")
            .arg("install")
            .status();
        install_status
    }

    fn push(app: &Application) -> io::Result<ExitStatus> {
        //TODO: Rewrite deployment.yml with tag specified

        let push_status = Command::new("mvn")
            .arg("jib:build")
            .arg(format!("-Djib.to.auth.username={}", &app.registry_user))
            .arg(format!("-Djib.to.auth.password={}", &app.registry_pass))
            .status();
        push_status
    }
}
