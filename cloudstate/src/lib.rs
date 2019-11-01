use git2::Repository;
use crate::builders::Application;

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

    pub fn smiling(self) -> char {
        '\u{1F63B}'
    }

    pub fn screaming(self) -> char {
        '\u{1F631}'
    }

    pub fn winking(self) -> char {
        '\u{1F609}'
    }


}

pub fn get_user_dir() -> String  {
    dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.cloudstate"
}

fn get_templates(home_dir: String) -> Repository {
    let repo = match Repository::clone("https://github.com/sleipnir/cloudstate-templates.git", home_dir) {
        Ok(repo) => repo,
        Err(e) => panic!("Failed to clone: {}", e),
    };

    repo
}

pub fn get_project_folder(application: &mut Application, current_dir: &String) -> String {
    current_dir.clone() + "/" + application.name.as_ref()
}