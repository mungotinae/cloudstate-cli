use git2::Repository;
use crate::builders::Application;

pub mod commands;
pub mod matches;
pub mod builders;

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