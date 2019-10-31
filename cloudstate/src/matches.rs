extern crate clap;

use clap::ArgMatches;

use std::{env, fs};
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

use crate::commands::command;
use crate::builders::Application;
use crate::get_project_folder;
use serde_json::ser::CharEscape;

#[derive(Debug)]
pub struct Resolver<'a> { pub args: ArgMatches<'a>, }

impl<'a> Resolver<'a> {

    pub fn matches(self) -> Result<(), String> {
        let _matches = self.args.clone();

        if _matches.is_present("list-profiles") {
            command::list_profiles();
        }

        // handle matches
        if _matches.is_present("init") {
            command::init();
        }

        match _matches.value_of("create") {
            Some(ref project_name) => {

                let mut application = Application::default();

                println!("Creating user function project: {:?}", project_name);
                if let Some(ref profile) = _matches.value_of("profile") {
                    let supported = ["java", "node", "go", "dotnet", "rust", "python", "scala"];
                    if !supported.contains(profile) {
                        return Err(String::from("Invalid Template name!"));
                    }

                    let tag = _matches.value_of("tag");
                    let repo = _matches.value_of("repo");
                    let editor = _matches.value_of("set-editor");
                    let repo_user = _matches.value_of("set-user");
                    let repo_pass = _matches.value_of("set-pass");
                    let datastore = _matches.value_of("datastore");

                    application.name = project_name.to_string();
                    application.profile = profile.to_string();
                    application.tag = tag.or_else(|| Option::from("0.0.1")).unwrap().to_string();
                    application.editor = editor.or_else(|| Option::from("vi")).unwrap().to_string();
                    application.data_store = datastore.or_else(|| Option::from("InMemory"))
                        .unwrap().to_string();

                    application.repo = if repo.or_else(|| Option::from("")).unwrap().is_empty() {
                        project_name.clone().to_string()
                    } else {
                        format!("{}/{}", repo.unwrap(), project_name.clone())
                    };

                    application.repo_user = repo_user.or_else(|| Option::from("")).unwrap().to_string();
                    application.repo_pass = repo_pass.or_else(|| Option::from("")).unwrap().to_string();

                    // Create cloudstate project dir
                    let path = get_project_folder(&mut application, &env::current_dir().unwrap().to_str().unwrap().to_string());
                    application.home_dir = dirs::home_dir().unwrap().to_str().unwrap().to_owned() + "/.cloudstate";
                    application.work_dir = path.clone();

                    let str = path + String::from("/.cloudstate").as_ref();
                    application.user_dir = str.clone();

                    let user_dir = Path::new(&str);
                    create_dir_all(user_dir.clone());

                    let application_json = serde_json::to_string(&application).unwrap();
                    let mut file = File::create(format!("{}{}/{}", application.work_dir, "/.cloudstate", "user.json")).unwrap();
                    file.write_all(application_json.as_ref()).unwrap();

                    println!("Using profile: {:?}", profile);
                    command::create_project(application);

                }
            }
            None => {}
        }

        if _matches.is_present("build") {
            <Resolver<'a>>::_build(&_matches)
        }

        if _matches.is_present("push") {
            println!("{:?}", env::current_dir());
            let path = format!("{}/.cloudstate/user.json", env::current_dir().unwrap().to_str().unwrap());
            println!("Path -> {}", path);
            let app_settings = fs::read_to_string(path);
            if app_settings.is_ok() {
                let mut application: Application = serde_json::from_str(app_settings.unwrap().as_str()).unwrap();
                println!("{:?}", application);

                // verify other options
                let tag = _matches.value_of("tag");
                if tag.is_some() {
                    application.tag = tag.unwrap().to_string();
                }

                <Resolver<'a>>::_push(application);

            } else {
                println!("App settings not found!");
            }

        }

        if _matches.is_present("deploy") {
            <Resolver<'a>>::_deploy(&_matches)
        }

        return Ok(());
    }

    fn _build(_matches: &ArgMatches) -> () {
        let build_path = _matches.value_of("build").unwrap();
        env::set_current_dir(build_path);
        println!("{:?}", env::current_dir());
        let path = format!("{}/.cloudstate/user.json", env::current_dir().unwrap().to_str().unwrap());
        println!("Path -> {}", path);
        let app_settings = fs::read_to_string(path);
        if app_settings.is_ok() {
            let mut application: Application = serde_json::from_str(app_settings.unwrap().as_str()).unwrap();
            println!("{:?}", application);

            // verify other options
            let tag = _matches.value_of("tag");
            if tag.is_some() {
                application.tag = tag.unwrap().to_string();
            }

            command::build(application.clone());

        } else {
            println!("App settings not found!");
        }
    }

    fn _deploy(_matches: &ArgMatches) -> () {
        println!("{:?}", env::current_dir());
        let path = format!("{}/.cloudstate/user.json", env::current_dir().unwrap().to_str().unwrap());
        let app_settings = fs::read_to_string(path);
        if app_settings.is_ok() {
            let mut application: Application = serde_json::from_str(app_settings.unwrap().as_str()).unwrap();
            println!("{:?}", application);

            // verify other options
            let tag = _matches.value_of("tag");
            if tag.is_some() {
                application.tag = tag.unwrap().to_string();
            }

            command::deploy(application);
        } else {
            println!("App settings not found!");
        }
    }

    fn _push(app: Application) -> () {
        command::push(app);
    }
}