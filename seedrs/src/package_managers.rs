use std::fmt;

use anyhow::{bail, Result};
use clap::ValueEnum;
use which::which;

#[derive(Debug, PartialEq, Eq, Hash, Clone, ValueEnum)]
pub enum PackageManagers {
    Npm,
    Pnpm,
    Yarn,
}

impl fmt::Display for PackageManagers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            PackageManagers::Pnpm => "pnpm",
            PackageManagers::Npm => "npm",
            PackageManagers::Yarn => "yarn",
        };

        write!(f, "{str}")
    }
}

impl PackageManagers {
    pub fn executable_name(&self) -> String {
        match self {
            PackageManagers::Npm => String::from("npm"),
            PackageManagers::Pnpm => String::from("pnpm"),
            PackageManagers::Yarn => String::from("yarn"),
        }
    }

    pub fn check_if_availabe(&self) -> Result<&Self> {
        if which(self.executable_name()).is_err() {
            bail!("Package manager isn't available for use!");
        }

        Ok(self)
    }

    pub fn values() -> Vec<Self> {
        vec![Self::Npm, Self::Pnpm, Self::Yarn]
    }
}
