use anyhow::Result;
use colored::Colorize;
use std::io::{self, BufRead, Write};

use crate::{PackageManager, ProjectKind, Tech};

pub fn render_project_naming_prompt() -> Result<String> {
    let default_project = String::from("my-project");

    let mut stdout = io::stdout();
    stdout.flush()?;
    let stdin = io::stdin();
    let mut project = String::new();
    loop {
        print!(
            "How will the project be named? [default: {}] ",
            default_project.bold()
        );
        stdout.flush()?;
        if stdin.lock().read_line(&mut project).is_err() {
            println!("Invalid input, please name your project with a valid UTF-8 string!");
            continue;
        }
        break;
    }
    println!();

    let trimmed = project.trim();
    if trimmed.is_empty() {
        return Ok(default_project);
    }
    Ok(trimmed.to_string())
}

pub fn render_project_definition_prompt() -> Result<(i32, i32, i32)> {
    let mut apis = None;
    let mut webs = None;
    let mut apps = None;

    let kinds = ProjectKind::values();

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    for kind in kinds {
        let ref_check = match kind {
            ProjectKind::Web => &mut webs,
            ProjectKind::Mobile => &mut apps,
            ProjectKind::Api => &mut apis,
        };

        while ref_check.is_none() {
            let mut answer = String::new();
            print!("How many {kind}s will the project have? (value >= 0, defaults to 0): ");
            stdout.flush()?;

            if stdin.lock().read_line(&mut answer).is_err() {
                println!("\nInvalid input, please insert a valid number!");
                continue;
            }

            let parsed = if answer.is_empty() {
                0
            } else {
                answer.trim().parse::<i32>().unwrap_or(-1)
            };

            if parsed >= 0 {
                *ref_check = Some(parsed);
            } else {
                println!("\nInvalid input, please insert a valid number!");
            }
        }
    }
    println!();

    Ok((apis.unwrap(), webs.unwrap(), apps.unwrap()))
}

pub fn render_naming_prompt(prefix: &str, kind: ProjectKind, order: i32) -> Result<String> {
    let default_name = if order == 1 {
        format!("{}-{}", prefix, kind.get_suffix())
    } else {
        format!("{}-{}-{}", prefix, kind.get_suffix(), order)
    };
    let order_text = if order == 1 {
        String::new()
    } else {
        format!(" {order}")
    };

    let mut answer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        print!(
            "How will the {kind}{order_text} be called? [default: {}] ",
            default_name.bold()
        );
        stdout.flush()?;

        if stdin.lock().read_line(&mut answer).is_err() {
            println!("\nPlease, insert a valid UTF-8 string!");
            continue;
        }
        println!();
        break;
    }

    let trimmed = answer.trim();
    if trimmed.is_empty() {
        return Ok(default_name);
    }
    Ok(trimmed.to_string())
}

pub fn render_git_init_prompt(project_name: &str) -> Result<bool> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut answer = String::new();

    loop {
        print!(
            "Should a git repo be initialized in project {project_name}? [no/anything else for {}] ",
            "yes".bold()
        );
        stdout.flush()?;

        if stdin.lock().read_line(&mut answer).is_err() {
            println!("\nPlease, insert a valid UTF-8 string!");
            continue;
        }
        break;
    }
    println!();

    Ok(answer.trim() != "no")
}

pub fn render_install_dependencies_prompt(project_name: &str) -> Result<bool> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut answer = String::new();

    loop {
        print!(
            "Would you like to install dependencies for {project_name}? [no/anything else for {}] ",
            "yes".bold()
        );
        stdout.flush()?;

        if stdin.lock().read_line(&mut answer).is_err() {
            println!("\nPlease, insert a valid UTF-8 string!");
            continue;
        }
        break;
    }
    println!();

    Ok(answer.trim() != "no")
}

pub fn render_tech_selection_prompt(project_name: &str, kind: ProjectKind) -> Result<Tech> {
    println!("Please, select which technology will be used in {project_name}:");
    let techs_vec = kind.get_techs();
    for (index, tech) in techs_vec.iter().enumerate() {
        println!("{}. {}", index + 1, tech);
    }

    let mut answer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut tech = None;

    while tech.is_none() {
        print!("Enter the number: ");
        stdout.flush()?;
        if stdin.lock().read_line(&mut answer).is_err() {
            println!("\nInvalid input, please select one of the provided techlogies!");
            continue;
        }

        let parsed = answer.trim().parse::<usize>();
        if parsed.is_err() {
            println!("\nInvalid input, please select one of the provided techlogies!");
            continue;
        }
        let parsed = parsed.unwrap();
        if parsed < 1 || parsed > techs_vec.len() {
            println!("\nInvalid input, please select one of the provided techlogies!");
            continue;
        }

        tech = techs_vec.get(parsed - 1);
    }
    println!();
    Ok(tech.unwrap().clone())
}

pub fn render_package_manager_selection_prompt(
    project_name: &str,
    tech: &Tech,
) -> Result<PackageManager> {
    println!("Please, select which package manager you will use for {project_name}:");

    let package_managers_vec = tech.get_package_managers();
    for (index, pkg_manager) in package_managers_vec.iter().enumerate() {
        println!("{}. {}", index + 1, pkg_manager);
    }

    let mut answer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut package_manager = None;
    while package_manager.is_none() {
        print!("Enter a number: ");
        stdout.flush()?;
        if stdin.lock().read_line(&mut answer).is_err() {
            println!("\nInvalid input, please select one of the provided package managers!");
            continue;
        }

        let parsed = answer.trim().parse::<usize>();
        if parsed.is_err() {
            println!("\nInvalid input, please select one of the provided package managers!");
            continue;
        }
        let parsed = parsed.unwrap();
        if parsed < 1 || parsed > package_managers_vec.len() {
            println!("\nInvalid input, please select one of the provided package_manager!");
            continue;
        }

        package_manager = package_managers_vec.get(parsed - 1);
    }

    println!();
    Ok(package_manager.unwrap().clone())
}
