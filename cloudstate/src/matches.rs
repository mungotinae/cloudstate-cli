extern crate clap;

use clap::ArgMatches;

use crate::commands::command;

#[derive(Debug)]
pub struct Resolver<'a> { pub args: ArgMatches<'a>, }

impl<'a> Resolver<'a> {

    pub fn matches(self) -> Result<(), String> {
        let _matches = self.args.clone();

        // handle matches
        if _matches.is_present("init") {
            command::init();
        }

        if _matches.is_present("list-idioms") {
            command::list_idioms();
        }

        if let Some(ref project_name) = _matches.value_of("create") {
            println!("Creating user function project: {:?}", project_name);
            if let Some(ref idiom) = _matches.value_of("idiom") {

                let supported = ["java", "node", "go", "dotnet", "rust", "python", "scala"];
                if !supported.contains(idiom) {
                    return Err(String::from("Invalid Template name!"));
                }

                println!("Using template: {:?}", idiom);
                command::create_project(project_name, idiom);
            }

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
        if let Some(_matches) = _matches.subcommand_matches("idiom") {
            if _matches.is_present("name") {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }

        return Ok(());
    }
}