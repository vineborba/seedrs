use std::io::{self, BufRead, Write};

use crate::{PackageManagers, Techs};

pub fn render_project_naming() -> String {
    println!("How will the project be named?");
    let mut project = String::new();
    let stdin = io::stdin();
    while stdin.lock().read_line(&mut project).is_err() {
        println!("Invalid input, please name your with a valid UTF-8 string!");
    }
    project.trim().to_owned()
}

pub fn render_tech_selection() -> Vec<Techs> {
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
        techs.push(techs_vec.get(i - 1).unwrap().clone());
    }

    techs
}

pub fn render_package_manager_selection() -> PackageManagers {
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

    let selected = answer.trim().parse::<usize>().unwrap();

    package_managers_vec.get(selected - 1).unwrap().clone()
}
