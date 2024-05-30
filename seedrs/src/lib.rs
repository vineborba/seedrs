mod options;
mod package_managers;
mod projects;
mod techs;
mod ui;

use std::fs;

use anyhow::Result;
use projects::{Project, ProjectBuilder};

pub use options::Options;
pub use package_managers::PackageManager;
pub use projects::ProjectKind;
pub use techs::Tech;

pub fn run(/* opts: Options */) -> Result<()> {
    // let Options {
    //     mut project_prefix,
    //     ..
    //     package_manager,
    //     mut techs,
    // } = opts;

    // if project_prefix.is_none() {
    //     project_prefix = Some(ui::render_project_naming()?);
    // }
    ui::render_welcome();

    let project_prefix = ui::render_project_naming_prompt()?;

    let (apis, webs, apps) = ui::render_project_definition_prompt()?;

    let sum = (apis + webs + apps) as usize;

    if sum == 0 {
        println!("You didn't specify any number of projects, exiting.");
        return Ok(());
    }

    let mut projects: Vec<Project> = Vec::with_capacity(sum);

    // TODO: make a single loop for all types
    for i in 0..webs {
        let name = ui::render_naming_prompt(&project_prefix, ProjectKind::Web, i + 1)?;
        let init_git = ui::render_git_init_prompt(&name)?;
        let install_deps = ui::render_install_dependencies_prompt(&name)?;
        let tech = ui::render_tech_selection_prompt(&name, ProjectKind::Web)?;
        let package_manager = ui::render_package_manager_selection_prompt(&name, &tech)?;

        projects.push(
            ProjectBuilder::default()
                .name(name)
                .tech(tech)
                .init_git(init_git)
                .should_install(install_deps)
                .package_manager(package_manager)
                .build()?,
        );

        println!("\n");
    }

    for i in 0..apis {
        let name = ui::render_naming_prompt(&project_prefix, ProjectKind::Api, i + 1)?;
        let init_git = ui::render_git_init_prompt(&name)?;
        let install_deps = ui::render_install_dependencies_prompt(&name)?;
        let tech = ui::render_tech_selection_prompt(&name, ProjectKind::Api)?;
        let package_manager = ui::render_package_manager_selection_prompt(&name, &tech)?;

        projects.push(
            ProjectBuilder::default()
                .name(name)
                .tech(tech)
                .init_git(init_git)
                .should_install(install_deps)
                .package_manager(package_manager)
                .build()?,
        );

        println!("\n");
    }

    for i in 0..apps {
        let name = ui::render_naming_prompt(&project_prefix, ProjectKind::Mobile, i + 1)?;
        let init_git = ui::render_git_init_prompt(&name)?;
        let install_deps = ui::render_install_dependencies_prompt(&name)?;
        let tech = ui::render_tech_selection_prompt(&name, ProjectKind::Mobile)?;
        let package_manager = ui::render_package_manager_selection_prompt(&name, &tech)?;

        projects.push(
            ProjectBuilder::default()
                .name(name)
                .tech(tech)
                .init_git(init_git)
                .should_install(install_deps)
                .package_manager(package_manager)
                .build()?,
        );

        println!("\n");
    }

    fs::create_dir(&project_prefix)?;

    let mut processes = Vec::new();

    for project in projects.iter() {
        match project.spawn_init_command(&project_prefix) {
            Ok(child) => processes.push((project, child)),
            Err(err) => {
                eprintln!(
                    "Failed to spawn init command for {}. Error: {}",
                    project.name, err
                );
            }
        }
    }

    let mut install_deps = Vec::new();
    let mut init_git = Vec::new();

    for (project, proc) in processes.into_iter() {
        match proc.wait_with_output() {
            Ok(output) => {
                if output.status.success() {
                    if project.init_git {
                        init_git.push(project);
                    }
                    if project.should_install {
                        install_deps.push(project);
                    }
                } else if let Some(code) = output.status.code() {
                    eprintln!(
                        "Error: init command for {} exited with {}",
                        project.name, code,
                    );
                } else {
                    eprintln!(
                        "Error: init command for {} exited with no exit code",
                        project.name,
                    );
                }
            }
            Err(err) => {
                eprintln!(
                    "Failed to run init command for {}. Error: {}",
                    project.name, err
                );
            }
        }
    }

    let mut processes = Vec::new();

    for project in install_deps.into_iter() {
        match project.spawn_git_init_command(&project_prefix) {
            Ok(child) => processes.push((project, child, "git init")),
            Err(err) => {
                eprintln!(
                    "Failed to spawn git initialization process for project {}. Error: {}",
                    project.name, err
                )
            }
        }
    }

    for project in init_git.into_iter() {
        match project.spawn_install_deps_command(&project_prefix) {
            Ok(child) => processes.push((project, child, "install dependencies")),
            Err(err) => {
                eprintln!(
                    "Failed to spawn dependencies install process for project {}. Error: {}",
                    project.name, err
                )
            }
        }
    }

    for (project, proc, proc_type) in processes.into_iter() {
        match proc.wait_with_output() {
            Ok(output) => {
                if output.status.success() {
                    println!("Successfully {} for {}", proc_type, project.name);
                } else if let Some(code) = output.status.code() {
                    eprintln!(
                        "Error: {} command for {} exited with {}",
                        proc_type, project.name, code,
                    );
                } else {
                    eprintln!(
                        "Error: {} for {} exited with no exit code",
                        proc_type, project.name,
                    );
                }
            }
            Err(err) => {
                eprintln!(
                    "Failed to run {} for {}. Error: {}",
                    proc_type, project.name, err
                );
            }
        }
    }

    Ok(())
}
