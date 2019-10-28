use crate::builders::{ProjectBuilder, Application};
use std::path::Path;

pub struct NodeBuilder;

impl ProjectBuilder for NodeBuilder {

    fn create(self, name: &str) {
        unimplemented!()
    }

    fn build(self, path: &Path, app: Application) {
        unimplemented!()
    }
}