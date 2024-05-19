use anyhow::{bail, Result};
use colored::Colorize;
use std::io::{self, BufRead, Write};

use crate::{PackageManagers, Techs};

pub fn render_project_naming() -> String {
    let default_project = String::from("my-project");

    print!(
        "How will the project be named? [default: {}] ",
        default_project.bold()
    );
    let mut stdout = io::stdout();
    stdout.flush().unwrap();
    let stdin = io::stdin();
    let mut project = String::new();
    while stdin.lock().read_line(&mut project).is_err() {
        println!("Invalid input, please name your project with a valid UTF-8 string!");
    }
    println!();

    let trimmed = project.trim();
    if trimmed.is_empty() {
        return default_project;
    }
    trimmed.to_string()
}

pub fn render_tech_selection() -> Result<Vec<Techs>> {
    let mut techs = vec![];

    println!("Please, select which technologies will be used in this project:");
    let techs_vec = Techs::values();
    for (index, tech) in techs_vec.iter().enumerate() {
        println!("{}. {}", index + 1, tech);
    }

    let mut answer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("Enter the numbers, using comma as a separator: ");
    stdout.flush().unwrap();
    while stdin.lock().read_line(&mut answer).is_err() {
        println!("\nInvalid input, please select one of the provided techlogies!");
        print!("Enter the numbers, using comma as a separator: ");
        stdout.flush().unwrap();
    }

    let selected = answer
        .split(',')
        .filter_map(|s| s.trim().parse::<usize>().ok())
        .collect::<Vec<usize>>();

    for i in selected.into_iter() {
        match techs_vec.get(i - 1) {
            Some(t) => techs.push(t.clone()),
            None => {
                eprintln!("Invalid tech selected: {}", i.to_string().red())
            }
        }
    }

    if techs.is_empty() {
        bail!("No valid tech was selected!".red())
    }

    println!();
    Ok(techs)
}

pub fn render_package_manager_selection() -> Result<PackageManagers> {
    println!("Please, select which package manager you will use for this project:");

    let package_managers_vec = PackageManagers::values();
    for (index, pkg_manager) in package_managers_vec.iter().enumerate() {
        println!("{}. {}", index + 1, pkg_manager);
    }

    let mut answer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    print!("Enter a number: ");
    stdout.flush().unwrap();
    while stdin.lock().read_line(&mut answer).is_err() {
        println!("\nInvalid input, please select one of the provided package managers!");
        print!("Enter a number: ");
        stdout.flush().unwrap();
    }

    println!();
    if let Ok(selected) = answer.trim().parse::<usize>() {
        match package_managers_vec.get(selected - 1) {
            Some(p) => Ok(p.clone()),
            None => bail!("Invalid package manager selected!".red()),
        }
    } else {
        bail!("Invalid package manager selected!".red())
    }
}
