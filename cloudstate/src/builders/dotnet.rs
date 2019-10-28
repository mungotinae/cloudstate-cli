use crate::builders::{ProjectBuilder, Application};
use std::path::Path;

pub struct DotNetBuilder;

impl ProjectBuilder for DotNetBuilder {

    fn create(self, name: &str) {
        unimplemented!()
    }

    fn build(self, path: &Path, app: Application) {
        unimplemented!()
    }
}
