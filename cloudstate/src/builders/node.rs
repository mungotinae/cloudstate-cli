use crate::builders::{ProjectBuilder, Application};
use std::path::Path;

pub struct NodeBuilder;

impl ProjectBuilder for NodeBuilder {

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