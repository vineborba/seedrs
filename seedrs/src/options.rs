use crate::{PackageManagers, Techs};

#[derive(Debug)]
pub struct Options {
    pub techs: Vec<Techs>,
    pub package_manager: Option<PackageManagers>,
    pub project_prefix: Option<String>,
}
