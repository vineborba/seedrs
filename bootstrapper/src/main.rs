use std::{collections::HashSet, fs, process::Command};

mod package_managers;
mod techs;

use anyhow::Result;
use package_managers::PackageManagers;
use rayon::prelude::*;
use techs::Techs;
use which::which;

fn main() -> Result<()> {
    let techs = vec![Techs::React, Techs::NodeNest, Techs::ReactNative];
    let managers = vec![
        PackageManagers::Npm,
        PackageManagers::Pnpm,
        PackageManagers::Yarn,
    ];

    let mut package_managers_executables: HashSet<String> = HashSet::new();

    managers.into_iter().for_each(|m| {
        let executable = m.executable_name();
        if which(&executable).is_ok() {
            package_managers_executables.insert(executable);
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

    techs.into_par_iter().enumerate().for_each(|(_, t)| {
        let name = t.name(project_prefix);
        let mut command = Command::new("npm");

        if t.is_mobile() {
            command.env("npm_config_user_agent", package_manager);
        }

        let output = command
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
