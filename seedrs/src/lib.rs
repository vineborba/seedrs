mod options;
mod package_managers;
mod projects;
mod techs;
mod ui;

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

    let project_prefix = ui::render_project_naming_prompt()?;

    let (apis, webs, apps) = ui::render_project_definition_prompt()?;

    let sum = (apis + webs + apps) as usize;

    if sum == 0 {
        println!("You didn't specify any number of projects, exiting.");
        return Ok(());
    }

    let mut projects: Vec<Project> = Vec::with_capacity(sum);

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
    }

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
    }

    dbg!(&projects);

    Ok(())
}
