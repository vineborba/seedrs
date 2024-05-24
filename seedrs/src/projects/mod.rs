use derive_builder::Builder;

use crate::{PackageManager, Tech};

mod project_kind;

pub use project_kind::ProjectKind;

#[derive(Debug, Builder)]
pub struct Project {
    name: String,
    tech: Tech,
    package_manager: PackageManager,
    init_git: bool,
    should_install: bool,
    #[builder(default)]
    template: Option<String>,
}
