use anyhow::{bail, Result};
use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use core::fmt;
use std::process::Command;

use crate::PackageManager;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Tech {
    React,
    ReactNative,
    NodeNest,

    #[default]
    Invalid,
}

impl fmt::Display for Tech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Tech::React => "React",
            Tech::ReactNative => "React-Native",
            Tech::NodeNest => "NestJS",
            Tech::Invalid => "Invalid",
        };

        write!(f, "{str}")
    }
}

impl From<&str> for Tech {
    fn from(value: &str) -> Self {
        match value {
            "react" => Self::React,
            "react-native" | "rn" => Self::ReactNative,
            "nestjs" | "node-nest" | "node-nestjs" => Self::NodeNest,
            _ => Self::Invalid,
        }
    }
}

impl Tech {
    pub fn suffix(&self) -> String {
        match self {
            Tech::React => String::from("web"),
            Tech::ReactNative => String::from("app"),
            Tech::NodeNest => String::from("api"),
            Tech::Invalid => unreachable!(),
        }
    }

    pub fn values() -> Vec<Self> {
        vec![Self::React, Self::NodeNest, Self::ReactNative]
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::ReactNative)
    }

    pub fn get_package_managers(&self) -> Vec<PackageManager> {
        match self {
            Self::ReactNative | Self::React | Self::NodeNest => {
                vec![
                    PackageManager::Npm,
                    PackageManager::Yarn,
                    PackageManager::Pnpm,
                ]
            }
            Self::Invalid => unreachable!(),
        }
    }

    pub fn colorize(&self) -> ColoredString {
        match self {
            Tech::React => format!("{self}").bright_blue().bold(),
            Tech::ReactNative => format!("{self}").cyan().bold(),
            Tech::NodeNest => format!("{self}").bright_red().bold(),
            Tech::Invalid => format!("{self}").black().bold(),
        }
    }

    pub fn init_command_args<'a>(
        &'a self,
        project_name: &'a str,
        package_manager: &'a str,
    ) -> Vec<&'a str> {
        let mut args = vec![];

        match self {
            Tech::React => {
                args.push("create");
                args.push("vite@latest");
                args.push(project_name);
                args.push("--");
                args.push("--template");
                args.push("react-ts");
            }
            Tech::NodeNest => {
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
            Tech::ReactNative => {
                args.push("create");
                args.push("expo-app");
                args.push(project_name);
                args.push("--");
                args.push("--no-install");
                args.push("--template");
                args.push("blank-typescript");
            }
            Tech::Invalid => unreachable!(),
        };

        args
    }

    pub fn bootstrap_project(&self, project_prefix: &str, package_manager: &str) -> Result<()> {
        let project_name = self.suffix();

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
