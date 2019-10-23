#[macro_use]
extern crate clap;
extern crate cloudstate;

use clap::App;
use cloudstate::matches::Resolver;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let resolver = Resolver {
        args: matches
    };

    let res = match resolver.matches(){
        Ok(res)  => {
            Ok(res)
        },
        Err(e) => Err(e),
    };

    println!("{:?}", res.map_err(|err| err.to_string()).map(|e| e));

}
