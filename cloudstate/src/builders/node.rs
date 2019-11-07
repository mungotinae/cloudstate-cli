use crate::builders::{ProjectBuilder, Application};
use std::path::Path;
use std::process::{Command, Stdio};

pub struct NodeBuilder;

impl ProjectBuilder for NodeBuilder {

    fn is_dependencies_ok(&self) -> bool {
        Command::new("which")
            .arg("npm")
            .stdout(Stdio::null())
            .status().is_ok()
    }

    fn pre_compile(&self, app: &Application) {
        unimplemented!()
    }

    fn compile(&self, app: &Application) {
        unimplemented!()
    }

    fn pos_compile(&self, app: &Application) {
        unimplemented!()
    }

    fn build(self, app: Application) {
        unimplemented!()
    }

    fn push(self, app: Application) {
        unimplemented!()
    }

    fn deploy(self, app: Application) {
        unimplemented!()
    }
}