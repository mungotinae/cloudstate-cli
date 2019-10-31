extern crate clap;

use clap::ArgMatches;

use std::env;
use std::fs::{File, create_dir_all};
use std::io::Write;
use std::path::Path;

use crate::commands::command;
use crate::builders::Application;
use crate::get_project_folder;

#[derive(Debug)]
pub struct Resolver<'a> { pub args: ArgMatches<'a>, }

impl<'a> Resolver<'a> {

    pub fn matches(self) -> Result<(), String> {
        let _matches = self.args.clone();

        // handle matches
        if _matches.is_present("init") {
            command::init();
        }

        if _matches.is_present("list-profiles") {
            command::list_profiles();
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
                    let datastore = _matches.value_of("datastore");
                    let editor = _matches.value_of("set-editor");

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

        //docker build -t shopping-cart .
        match _matches.value_of("build") {
            Some(ref path) => {
                // TODO: Validating other params
                println!("Building user function project... ");
                command::build(path);
            }
            None => {}
        }

        // Vary the output based on how many times the user used the "verbose" flag
        // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
        match _matches.occurrences_of("v") {
            0 => println!("No verbose info"),
            1 => println!("Some verbose info"),
            2 => println!("Tons of verbose info"),
            3 | _ => println!("Don't be crazy"),
        }

        // You can handle information about subcommands by requesting their matches by name
        // (as below), requesting just the name used, or both at the same time
        if let Some(_matches) = _matches.subcommand_matches("profile") {
            if _matches.is_present("name") {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }

        return Ok(());
    }


}