use crate::builders::{ProjectBuilder, Application};
use std::path::Path;

pub struct PythonBuilder;

impl ProjectBuilder for PythonBuilder {

    fn create(self, name: &str) {
        unimplemented!()
    }

    fn build(self, path: &Path, app: Application) {
        unimplemented!()
    }
}