use std::process::Command;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub trait ProjectBuilder {
    fn build(self, name: &str);
}

pub struct RustBuilder;
pub struct JavaBuilder;
pub struct NodeBuilder;
pub struct GoBuilder;
pub struct DotNetBuilder;
pub struct PythonBuilder;
pub struct ScalaBuilder;

impl RustBuilder {
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

    fn build(self, name: &str) {
        let status = Command::new("cargo")
            .arg("new")
            //.arg("--bin")
            .arg(name)
            .status();

        if status.is_ok() {

            //TODO: Create Dockerfile
            println!("Creating Dockerfile");

            let dockerfile_contents = RustBuilder::get_dockerfile();

            let dockerfile = dockerfile_contents.replace("myapp", name);

            let mut file = File::create(name.to_owned() + "/" + "Dockerfile").unwrap();
            file.write_all(dockerfile.as_ref());

            //TODO: Create deployment.yml
            println!("Creating deployment.yml");
            let mut file = File::create(name.to_owned() + "/" + "deployment.yml");

            //TODO: Add CloudState Crate dependency

            println!("Project created!");
            Command::new("ls")
                .arg("-ltr")
                .arg(name)
                .spawn()
                .expect("Error during create Rust project");
        } else {
            println!("Error on create Rust project")
        }

    }
}


impl ProjectBuilder for JavaBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for NodeBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for GoBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for DotNetBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for PythonBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}

impl ProjectBuilder for ScalaBuilder {

    fn build(self, name: &str) {
        unimplemented!()
    }
}


