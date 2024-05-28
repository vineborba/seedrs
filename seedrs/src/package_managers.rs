use anyhow::{bail, Result};
use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use std::fmt;
use which::which;

#[derive(Debug, PartialEq, Eq, Hash, Clone, ValueEnum, Default)]
pub enum PackageManager {
    #[default]
    Npm,
    Pnpm,
    Yarn,
}

impl fmt::Display for PackageManager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            PackageManager::Pnpm => "pnpm",
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
        };

        write!(f, "{str}")
    }
}

impl PackageManager {
    pub fn executable_name(&self) -> String {
        match self {
            PackageManager::Npm => String::from("npm"),
            PackageManager::Pnpm => String::from("pnpm"),
            PackageManager::Yarn => String::from("yarn"),
        }
    }

    pub fn check_if_availabe(&self) -> Result<&Self> {
        if which(self.executable_name()).is_err() {
            bail!("Package manager isn't available for use!");
        }

        Ok(self)
    }

    pub fn colorize(&self) -> ColoredString {
        match self {
            PackageManager::Npm => format!("{self}").red().bold(),
            PackageManager::Pnpm => format!("{self}").yellow().bold(),
            PackageManager::Yarn => format!("{self}").blue().bold(),
        }
    }
}
