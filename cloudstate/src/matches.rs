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

        // handle matches
        if _matches.is_present("list-profiles") {
            command::list_profiles();
        }

        if _matches.is_present("check") {
            command::check();
        }

        if let Some(logs_matches) = _matches.subcommand_matches("logs") {
            command::logs(logs_matches);
        }

        // handle sub commands matches
        if let Some(init_matches) =_matches.subcommand_matches("init") {
            command::init();
        }

        if let Some(destroy_matches) =_matches.subcommand_matches("destroy") {
            command::destroy();
        }

        // Matches create
        if let Some(create_matches) =_matches.subcommand_matches("create") {

            let mut application = Application::default();

            if create_matches.is_present("name") {

                let project_name = create_matches.value_of("name").unwrap();

                println!("Creating user function project: {:?}", project_name);
                if let Some(ref profile) = create_matches.value_of("profile") {
                    let supported = ["java", "node", "go", "dotnet", "rust", "python", "scala"];
                    if !supported.contains(profile) {
                        return Err(String::from("Invalid Template name!"));
                    }

                    let tag = create_matches.value_of("tag");
                    let registry = create_matches.value_of("registry");
                    let editor = create_matches.value_of("set-editor");
                    let registry_user = create_matches.value_of("set-user");
                    let registry_pass = create_matches.value_of("set-pass");
                    let datastore = create_matches.value_of("datastore");

                    application.name = project_name.to_string();
                    application.profile = profile.to_string();
                    application.tag = tag.or_else(|| Option::from("0.0.1")).unwrap().to_string();
                    application.editor = editor.or_else(|| Option::from("vi")).unwrap().to_string();
                    application.data_store = datastore.or_else(|| Option::from("InMemory"))
                        .unwrap().to_string();

                    application.registry = if registry.or_else(|| Option::from("")).unwrap().is_empty() {
                        project_name.clone().to_string()
                    } else {
                        format!("{}/{}", registry.unwrap(), project_name.clone())
                    };

                    application.registry_user = registry_user.or_else(|| Option::from("")).unwrap().to_string();
                    application.registry_pass = registry_pass.or_else(|| Option::from("")).unwrap().to_string();

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
                } else {
                    println!("Name option is mandatory");
                }
            }
        }

        // Matches build
        if let Some(build_matches) =_matches.subcommand_matches("build") {
            println!("Building...");

            if build_matches.is_present("path") {

                let build_path = build_matches.value_of("path").unwrap_or(".");
                println!("Build path: {:?}", build_path);

                env::set_current_dir(build_path);
                println!("{:?}", env::current_dir());

                let path = format!("{}/.cloudstate/user.json", env::current_dir().unwrap().to_str().unwrap());
                let app_settings = fs::read_to_string(path);
                if app_settings.is_ok() {
                    let mut application: Application = serde_json::from_str(app_settings.unwrap().as_str()).unwrap();

                    // verify other options
                    let tag = build_matches.value_of("tag");
                    if tag.is_some() {
                        application.tag = tag.unwrap().to_string();
                    }

                    command::build(application.clone());

                    // Matches push
                    if build_matches.is_present("push") {
                        println!("{:?}", env::current_dir());
                        let path = format!("{}/.cloudstate/user.json", env::current_dir().unwrap().to_str().unwrap());
                        let app_settings = fs::read_to_string(path);
                        if app_settings.is_ok() {
                            let mut application: Application = serde_json::from_str(app_settings.unwrap().as_str()).unwrap();

                            // verify other options
                            let tag = build_matches.value_of("tag");
                            if tag.is_some() {
                                application.tag = tag.unwrap().to_string();
                            }

                            <Resolver<'a>>::_push(application);

                        } else {
                            println!("App settings not found!");
                        }

                    }

                } else {
                    println!("App settings not found!");
                }
            } else {
                println!("Path not captured!")
            }

        }

        if let Some(build_matches) =_matches.subcommand_matches("deploy") {
            <Resolver<'a>>::_deploy(&_matches)
        }

        return Ok(());
    }


    fn _deploy(_matches: &ArgMatches) -> () {
        println!("{:?}", env::current_dir());
        let path = format!("{}/.cloudstate/user.json", env::current_dir().unwrap().to_str().unwrap());
        let app_settings = fs::read_to_string(path);
        if app_settings.is_ok() {
            let mut application: Application = serde_json::from_str(app_settings.unwrap().as_str()).unwrap();

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