use anyhow::{bail, Result};
use clap::ValueEnum;
use which::which;

#[derive(Debug, PartialEq, Eq, Hash, Clone, ValueEnum)]
pub enum PackageManagers {
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

    pub fn check_if_availabe(&self) -> Result<&Self> {
        if which(self.executable_name()).is_err() {
            bail!("Package manager isn't available for use!");
        }

        Ok(self)
    }
}
