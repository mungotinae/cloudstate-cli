pub mod java;
pub mod node;
pub mod go;
pub mod rust;
pub mod dotnet;
pub mod python;
pub mod scala;

use std::path::Path;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use tar::Archive;
use flate2::read::GzDecoder;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Application {
    pub name: String,
    pub tag: String,
    pub home_dir: String,
    pub work_dir: String,
    pub user_dir: String,
    pub profile: String,
    pub namespace: String,
    pub registry: String,
    pub registry_user: String,
    pub registry_pass: String,
    pub editor: String,
    pub data_store: String,
    pub port: u16,
}

impl Default for Application {

    fn default() -> Self {
        Application {
            name: "shopping-cart".to_string(),
            tag: "0.0.1".to_string(),
            home_dir: "".to_string(),
            work_dir: String::from(""),
            user_dir: "".to_string(),
            profile: String::from(""),
            namespace: String::from("cloudstate"),
            registry: "".to_string(),
            registry_user: "".to_string(),
            registry_pass: "".to_string(),
            editor: "vi".to_string(),
            data_store: String::from("InMemory"),
            port: 8088
        }
    }
}

pub trait ProjectBuilder {

    fn is_dependencies_ok(&self) -> bool;

    /// Build template file structure for profiles
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn make_template(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        let root = &app.home_dir;
        let template_package =  format!("{}/templates/{}/{}.tar.gz", root, app.profile, app.profile);

        println!("Extracting profile template.... {}", template_package.clone());
        let tar_gz = File::open(template_package).unwrap();
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack("../").unwrap();

    }

    /// Perform certain operations before starting project compilation. Like replacing application name with a specified in --create="name"
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn pre_compile(&self, app: &Application);

    /// Compile project for specified template. Example: mvn install for java profile
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn compile(&self, app: &Application);

    /// Perform any operation after the compilation step. Such as opening the user's default editor in the project folder.
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn pos_compile(&self, app: &Application) {
        env::set_current_dir(&app.work_dir);

        println!("Project created!");
        Command::new("ls")
            .arg("-ltr")
            .spawn()
            .expect("Error during create Java project");

        println!("Open editor!");
        let cached = dirs::home_dir().unwrap();
        Command::new(&app.editor)
            .arg(".")
            .status()
            .expect("Error on open code editor");

    }

    /// Create project from specified profile template.
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn create(&self, app: Application) {
        self.make_template(&app);
        self.pre_compile(&app);
        self.compile(&app);
        self.pos_compile(&app)
    }

    /// Build CloudState project. Run docker build
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn build(self, app: Application);

    /// Push image to registry
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn push(self, app: Application);

    /// Push image to registry
    ///
    /// # Arguments
    ///
    /// * `app` - A application metadata
    ///
    fn deploy(self, app: Application);
}