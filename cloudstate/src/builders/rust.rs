extern crate dirs;
extern crate cargo_toml_builder;

use cargo_toml_builder::prelude::*;
use std::path::Path;
use std::string::ToString;
use crate::builders::{ProjectBuilder, Application};

pub struct RustBuilder;

impl RustBuilder {
    fn get_cargo_toml(name: &str, version: &str) -> String {
        let log_dep = Dependency::version("log", "0.4.8");
        let log_rs_dep = Dependency::version("log4rs","0.8.3");
        // let log_rs_dep = Dependency::repo("cloudstate", "https://github.com/foo/bar");
        let cloud_state_dep =  Dependency::version("cloudstate", "0.0.1");

        let dependencies = vec![log_dep, log_rs_dep, cloud_state_dep];

        let cargo_toml = CargoToml::builder()
            .name(name)
            .version(version)
            .author(whoami::username().as_ref())
            .dependencies(&dependencies)
            .build();

        cargo_toml.unwrap().to_string()

    }

    fn get_main() -> &'static str {
        let main_rs_contents = r###"
extern crate log;
extern crate log4rs;
extern crate cloudstate;

use log::{info};
use cloudstate::serveless::{CloudState, EntityService};

fn main() {

    // CloudState depends of log4rs to print messages
    log4rs::init_file("config/log4rs.yml", Default::default()).unwrap();
    info!("Starting CloudState Server...");

    let service = EntityService::new()
        .persistence_id("shopping-cart".to_string())
        .protos(vec!["shoppingcart/shoppingcart.proto".to_string(), "shoppingcart/persistence/domain.proto".to_string()])
        .snapshot(1)
        .event_sourced();

    CloudState::new()
        .register_entity_service(
            String::from("com.example.shoppingcart.ShoppingCart"),
            service)
        .start();
}
        "###;
        main_rs_contents
    }

    fn get_dockerfile() -> &'static str {
        let dockerfile_contents = r###"
# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
ARG BASE_IMAGE=ekidd/rust-musl-builder:latest

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS cargo-build

RUN sudo apt-get update && sudo apt-get install -y upx-ucl

# Add our source code.
ADD . ./

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

RUN /usr/bin/upx --brute /home/rust/src/target/x86_64-unknown-linux-musl/release/myapp

# Now, we need to build our _real_ Docker container, copying in `using-diesel`.
# ------------------------------------------------------------------------------
# Final Stage
# -------------------- ----------------------------------------------------------
FROM scratch
COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/myapp \
    /usr/local/bin/

CMD ["/usr/local/bin/myapp"]
            "###;
        dockerfile_contents
    }
}

impl ProjectBuilder for RustBuilder {

    fn pre_compile(&self, app: &Application) {
        unimplemented!()
    }

    fn compile(&self, app: &Application) {
        unimplemented!()
    }

    fn pos_compile(&self, app: &Application) {
        unimplemented!()
    }

    /*fn create(&self, app: Application) -> Result<(), throw::Error<&'static str>> {
        println!("Downloading language template...");
        let home_dir = get_user_dir();
        *//*
        let status = Command::new("cargo")
            .arg("new")
            //.arg("--bin")
            .arg(app.name)
            .status();

        if status.is_ok() {

            //TODO: Create Dockerfile
            println!("Creating Dockerfile");

            let dockerfile_contents = RustBuilder::get_dockerfile();

            let dockerfile = dockerfile_contents.replace("myapp", app.name.as_ref());

            let mut docker_file = File::create(app.name.to_owned() + "/" + "Dockerfile").unwrap();
            docker_file.write_all(dockerfile.as_ref());

            //TODO: Create deployment.yml
            println!("Creating deployment.yml");
            let mut file = File::create(app.name.to_owned() + "/" + "deployment.yml");

            //TODO: Add CloudState Crate dependency
            let cargo_contents = RustBuilder::get_cargo_toml(app.name.to_owned().as_str(), "0.0.1");
            let mut cargo_file = File::create(app.name.to_owned() + "/Cargo.toml").unwrap();
            cargo_file.write_all(cargo_contents.as_ref());

            //TODO: Create main.rs
            let main_rs_contents = RustBuilder::get_main();
            let mut main_file = File::create(app.name.to_owned() + "/src/main.rs").unwrap();
            main_file.write_all(main_rs_contents.as_ref());

            println!("Project created!");
            Command::new("ls")
                .arg("-ltr")
                .arg(app.name)
                .spawn()
                .expect("Error during create Rust project");
        } else {
            println!("Error on create Rust project")
        }*//*

        Ok(())
    }*/

    fn build(self, path: &Path, app: Application) {
        unimplemented!()
    }
}
