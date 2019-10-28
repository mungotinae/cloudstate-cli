use crate::builders::{ProjectBuilder, Application};
use std::path::Path;

pub struct GoBuilder;

impl ProjectBuilder for GoBuilder {

    fn create(self, name: &str) {
        unimplemented!()
    }

    fn build(self, path: &Path, app: Application) {
        unimplemented!()
    }
}
