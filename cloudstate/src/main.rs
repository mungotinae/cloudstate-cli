#[macro_use]
extern crate clap;
extern crate cloudstate;
extern crate self_update;

use cloudstate::matches::Resolver;
use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml);
    let clone_app = app.clone();
    let matches = app.get_matches();

    let mut resolver = Resolver {
        app: clone_app,
        args: matches
    };

    let res = match resolver.matches(){
        Ok(res)  => Ok(res),
        Err(e) => Err(e),
    };

    println!("{:?}", res.map_err(|err| err.to_string()).map(|e| e));

}
