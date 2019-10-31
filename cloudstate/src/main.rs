#[macro_use]
extern crate clap;
extern crate throw;
extern crate cloudstate;

use clap::App;
use cloudstate::matches::Resolver;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let version = format!("{}.{}.{}{}",
                      env!("CARGO_PKG_VERSION_MAJOR"),
                      env!("CARGO_PKG_VERSION_MINOR"),
                      env!("CARGO_PKG_VERSION_PATCH"),
                      option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml)
        .version(version.as_str())
        .get_matches();

    let resolver = Resolver {
        args: matches
    };

    let res = match resolver.matches(){
        Ok(res)  => Ok(res),
        Err(e) => Err(e),
    };

    println!("{:?}", res.map_err(|err| err.to_string()).map(|e| e));

}
