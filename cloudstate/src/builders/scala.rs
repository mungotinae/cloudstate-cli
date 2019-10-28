use crate::builders::{ProjectBuilder, Application};
use std::path::Path;

pub struct ScalaBuilder;

impl ProjectBuilder for ScalaBuilder {

    fn create(self, name: &str) {
        unimplemented!()
    }

    fn build(self, path: &Path, app: Application) {
        unimplemented!()
    }
}