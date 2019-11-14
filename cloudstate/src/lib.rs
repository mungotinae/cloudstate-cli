use git2::Repository;
use crate::builders::Application;
use std::path::Path;
use std::{fs, env};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

pub mod commands;
pub mod matches;
pub mod builders;

#[derive(Debug, Default)]
pub struct Emojis;

impl Emojis {

    pub fn rocket(self) -> char {
        '\u{1F680}'
    }

    pub fn success(self) -> char {
        '\u{1F64C}'
    }

    pub fn crying(self) -> char {
        '\u{1F63F}'
    }

    pub fn fire(self) -> char {
        '\u{1F525}'
    }

    pub fn smiling(self) -> char {
        '\u{1F63B}'
    }

    pub fn screaming(self) -> char {
        '\u{1F631}'
    }

    pub fn winking(self) -> char {
        '\u{1F609}'
    }

    pub fn stable(self) -> char { '\u{1F44C}' }

    pub fn unstable(self) -> char { '\u{1F44D}' }

    pub fn work_in_progress(self) -> char {'\u{231B}' }

    pub fn unknown(self) -> char { '\u{1F44E}' }

    pub fn broken_heart(self) -> char { '\u{1F494}' }

    pub fn stuck_out(self) -> char { '\u{1F61D}' }

    pub fn bomb(self) -> char { '\u{1F4A3}' }

    pub fn warning(self) -> char { '\u{26A0}'}

    pub fn sunglasses(self) -> char { '\u{1F60E}' }

    pub fn ok(self) -> char { '\u{2714}' }

    pub fn nok(self) -> char { '\u{2716}' }

    pub fn coffee(self) -> char { '\u{2615}' }

    pub fn magnifying_glass(self) -> char { '\u{1F50D}' }

    pub fn building(self) -> char { '\u{1F3E2}' }

    pub fn worker(self) -> char { '\u{1F477}' }

    pub fn floppy_disk(self) -> char { '\u{1F4BE}' }

    pub fn moon(self) -> char { '\u{1F31B}' }

    pub fn frame_picture(self) -> char { '\u{1F5BC}' }


}

pub fn get_user_dir() -> String  {
    dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.cloudstate"
}

pub fn get_templates(home_dir: String) -> Repository {
    let repo = match Repository::clone("https://github.com/sleipnir/cloudstate-templates.git", home_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    repo
}

pub fn get_project_folder(application: &mut Application, current_dir: &String) -> String {
    current_dir.clone() + "/" + application.name.as_ref()
}

pub fn set_deployment_vars(app: &&Application) {
    let deployment_path = Path::new(&app.work_dir).join("deployment.yml");
    let deployment_template_content = fs::read_to_string(deployment_path.clone()).unwrap();

    let image_name = &app.registry;

    let deployment_name = deployment_template_content.replace("{application-name}", app.name.as_ref());
    let deployment_image = deployment_name.replace("{image-name}", image_name.as_str());
    let deployment_content = deployment_image.replace("{tag}", app.tag.as_ref());
    let mut deployment_file = File::create(deployment_path).unwrap();
    deployment_file.write_all(deployment_content.as_ref());
}

pub fn check_command(name: &str) -> Option<i32> {
    Command::new("which")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status().unwrap()
        .code()
}

pub fn k8s_deploy(app: &Application) -> () {
    env::set_current_dir(&app.work_dir);
    let result = Command::new("kubectl")
        .arg("apply")
        .arg("-n")
        .arg(&app.namespace)
        .arg("-f")
        .arg("deployment.yml")
        .status();

    if result.is_ok() {
        println!("Success on installing 'User Function' {} in namespace: {}", &app.name, &app.namespace);
    } else {
        panic!("Error on installing 'User Function' {} in namespace: {}", &app.name, &app.namespace);
    }
}