use clap::ValueEnum;

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
}
