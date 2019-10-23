extern crate clap;

use clap::ArgMatches;
use std::fmt::Error;

use crate::commands::command;

#[derive(Debug)]
pub struct Resolver<'a> { pub args: ArgMatches<'a>, }

impl<'a> Resolver<'a> {

    pub fn matches(self) -> Result<(), Error> {
        let _matches = self.args.clone();

        // handle matches
        if _matches.is_present("init") {
            command::init();
        }

        if _matches.is_present("list-templates") {
            command::list_templates();
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
        if let Some(_matches) = _matches.subcommand_matches("test") {
            if _matches.is_present("debug") {
                println!("Printing debug info...");
            } else {
                println!("Printing normally...");
            }
        }

        Ok(())
    }
}