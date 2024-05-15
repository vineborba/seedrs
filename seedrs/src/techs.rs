use anyhow::{bail, Result};
use clap::ValueEnum;
use core::fmt;
use std::process::Command;

use crate::PackageManagers;

#[derive(Debug, Clone, ValueEnum)]
pub enum Techs {
    React,
    ReactNative,
    NodeExpress,
    NodeNest,

    Invalid,
}

impl fmt::Display for Techs {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Techs::React => "React",
            Techs::ReactNative => "React-Native",
            Techs::NodeExpress => "Express",
            Techs::NodeNest => "NestJS",
            Techs::Invalid => unreachable!(),
        };

        write!(f, "{str}")
    }
}

impl From<&str> for Techs {
    fn from(value: &str) -> Self {
        match value {
            "node" => Self::NodeExpress,
            "react" => Self::React,
            "react-native" => Self::ReactNative,
            "node-nestjs" => Self::NodeNest,
            _ => Self::Invalid,
        }
    }
}

impl Techs {
    pub fn name(&self, project: &str) -> String {
        let suffix = match self {
            Techs::React => "web",
            Techs::ReactNative => "app",
            Techs::NodeExpress | Techs::NodeNest => "api",
            Techs::Invalid => unreachable!(),
        };

        format!("{project}-{suffix}")
    }

    pub fn values() -> Vec<Self> {
        vec![Self::React, Self::ReactNative, Self::NodeNest]
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::ReactNative)
    }

    pub fn get_package_managers(&self) -> Vec<PackageManagers> {
        match self {
            Self::ReactNative | Self::React | Self::NodeNest | Self::NodeExpress => {
                vec![
                    PackageManagers::Npm,
                    PackageManagers::Yarn,
                    PackageManagers::Pnpm,
                ]
            }
            Self::Invalid => unreachable!(),
        }
    }

    pub fn init_command_args<'a>(
        &'a self,
        project_name: &'a str,
        package_manager: &'a str,
    ) -> Vec<&'a str> {
        let mut args = vec![];

        match self {
            Techs::React => {
                args.push("create");
                args.push("vite@latest");
                args.push(project_name);
                args.push("--");
                args.push("--template");
                args.push("react-ts");
            }
            Techs::NodeNest => {
                args.push("exec");
                args.push("--yes");
                args.push("@nestjs/cli");
                args.push("new");
                args.push(project_name);
                args.push("--");
                args.push("--skip-install");
                args.push("-p");
                args.push(package_manager);
            }
            Techs::ReactNative => {
                args.push("create");
                args.push("expo-app");
                args.push(project_name);
                args.push("--");
                args.push("--no-install");
                args.push("--template");
                args.push("blank-typescript");
            }
            Techs::NodeExpress => {
                args.push("init");
                args.push("-y");
            }
            Techs::Invalid => unreachable!(),
        };

        args
    }

    pub fn bootstrap_project(&self, project_prefix: &str, package_manager: &str) -> Result<()> {
        let project_name = self.name(project_prefix);

        let project_creation_args = self.init_command_args(&project_name, package_manager);

        let mut init = Command::new("npm");

        if self.is_mobile() {
            init.env("npm_config_user_agent", package_manager);
        }

        let init_output = init
            .current_dir(project_prefix)
            .args(project_creation_args)
            .output()?;

        if !init_output.status.success() {
            bail!("Failed to run init process.");
        }

        let git_init_output = Command::new("git")
            .current_dir(format!("{project_prefix}/{project_name}"))
            .arg("init")
            .output()?;

        if !git_init_output.status.success() {
            bail!("Failed to init git repository.");
        }

        let install_output = Command::new(package_manager)
            .current_dir(format!("{project_prefix}/{project_name}"))
            .arg("install")
            .output()?;

        if !install_output.status.success() {
            bail!("Failed to install dependencies.");
        }

        Ok(())
    }
}
