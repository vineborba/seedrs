mod options;
mod package_managers;
mod techs;

use anyhow::Result;
use rayon::prelude::*;
use std::{fs, process::Command};

pub use options::Options;
pub use package_managers::PackageManagers;
pub use techs::Techs;

pub fn run(opts: Options) -> Result<()> {
    let Options {
        project_prefix,
        package_manager,
        techs,
    } = opts;

    let package_manager = package_manager
        .unwrap_or(PackageManagers::Npm)
        .executable_name();

    fs::create_dir(&project_prefix)?;

    techs.into_par_iter().enumerate().for_each(|(_, t)| {
        let name = t.name(&project_prefix);
        let mut command = Command::new("npm");

        if t.is_mobile() {
            command.env("npm_config_user_agent", &package_manager);
        }

        let output = command
            .current_dir(&project_prefix)
            .args(t.create_args(&project_prefix, package_manager.to_string()))
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
