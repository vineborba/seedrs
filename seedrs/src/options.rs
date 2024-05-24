use crate::{PackageManager, Tech};

#[derive(Debug)]
pub struct Options {
    pub techs: Vec<Tech>,
    pub package_manager: Option<PackageManager>,
    pub project_prefix: Option<String>,
}
