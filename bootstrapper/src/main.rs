use std::{collections::HashSet, fs, process::Command};

use anyhow::Result;
use which::which;

#[derive(Debug)]
enum Techs {
    React,
    ReactNative,
    NodeExpress,
    NodeNest,

    Invalid,
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

    pub fn create_args(&self, project_prefix: &str, package_manager: String) -> Vec<String> {
        let mut args = vec![];

        let project_name = self.name(project_prefix);

        match self {
            Techs::React => {
                args.push("create".to_string());
                args.push("vite@latest".to_string());
                args.push(project_name);
                args.push("--".to_string());
                args.push("--template".to_string());
                args.push("react-ts".to_string());
            }
            Techs::ReactNative => todo!(),
            Techs::NodeExpress => todo!(),
            Techs::NodeNest => {
                args.push("exec".to_string());
                args.push("--yes".to_string());
                args.push("@nestjs/cli".to_string());
                args.push("new".to_string());
                args.push(project_name);
                args.push("--".to_string());
                args.push("-p".to_string());
                args.push(package_manager);
            }
            Techs::Invalid => todo!(),
        };

        args
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum PackageManagers {
    Npm,
    Pnpm,
    Yarn,
}

impl PackageManagers {
    pub fn executable_name(&self) -> String {
        match self {
            PackageManagers::Npm => String::from("npm"),
            PackageManagers::Pnpm => String::from("pnpm"),
            PackageManagers::Yarn => String::from("yarn"),
        }
    }
}

fn main() -> Result<()> {
    let techs = vec![Techs::React, Techs::NodeNest];
    let managers = vec![
        PackageManagers::Npm,
        PackageManagers::Pnpm,
        PackageManagers::Yarn,
    ];

    let mut package_managers: HashSet<String> = HashSet::new();

    managers.into_iter().for_each(|m| {
        let executable = m.executable_name();
        if which(&executable).is_ok() {
            package_managers.insert(executable);
        }
    });

    // println!("Which package manager will you use?");
    // executables
    //     .into_iter()
    //     .enumerate()
    //     .for_each(|e| println!("{}. {}", e.0 + 1, e.1))

    let project_prefix = "my-project";
    let package_manager = "pnpm";

    fs::create_dir(project_prefix)?;

    techs.into_iter().enumerate().for_each(|(_, t)| {
        let name = t.name(project_prefix);
        let output = Command::new("npm")
            .current_dir(project_prefix)
            .args(t.create_args(project_prefix, package_manager.to_string()))
            .output()
            .expect("Failed to run create command for {name}");

        if output.status.success() {
            println!("Successfuly created {name}");
        } else {
            eprintln!("Failed to create {name}");
        }
    });

    Ok(())
}
