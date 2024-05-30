use anyhow::Result;
use derive_builder::Builder;
use std::process::{Child, Command};

use crate::{PackageManager, Tech};

mod project_kind;

pub use project_kind::ProjectKind;

#[derive(Debug, Builder)]
pub struct Project {
    pub name: String,
    tech: Tech,
    package_manager: PackageManager,
    pub init_git: bool,
    pub should_install: bool,
    #[builder(default)]
    template: Option<String>,
}

impl Project {
    pub fn init_command_args(&self) -> Vec<String> {
        let mut args = vec![];

        match self.tech {
            Tech::React => {
                args.push("create".into());
                args.push("vite@latest".into());
                args.push(self.name.clone());
                args.push("--".into());
                args.push("--template".into());
                args.push("react-ts".into());
            }
            Tech::NodeNest => {
                args.push("exec".into());
                args.push("--yes".into());
                args.push("@nestjs/cli".into());
                args.push("new".into());
                args.push(self.name.clone());
                args.push("--".into());
                args.push("--skip-install".into());
                args.push("-p".into());
                args.push(self.package_manager.to_string());
            }
            Tech::ReactNative => {
                args.push("create".into());
                args.push("expo-app".into());
                args.push(self.name.clone());
                args.push("--".into());
                args.push("--no-install".into());
                args.push("--template".into());
                args.push("blank-typescript".into());
            }
            Tech::Invalid => unreachable!(),
        };

        args
    }

    pub fn spawn_init_command(&self, project_prefix: &str) -> Result<Child> {
        let project_creation_args = self.init_command_args();

        let mut init = Command::new("npm");

        if self.tech.is_mobile() {
            init.env("npm_config_user_agent", self.package_manager.to_string());
        }

        Ok(init
            .current_dir(project_prefix)
            .args(project_creation_args)
            .spawn()?)
    }

    pub fn spawn_git_init_command(&self, project_prefix: &str) -> Result<Child> {
        Ok(Command::new("git")
            .current_dir(format!("{project_prefix}/{}", &self.name))
            .arg("init")
            .spawn()?)
    }

    pub fn spawn_install_deps_command(&self, project_prefix: &str) -> Result<Child> {
        Ok(Command::new(self.package_manager.to_string())
            .current_dir(format!("{project_prefix}/{}", &self.name))
            .arg("install")
            .spawn()?)
    }
}
