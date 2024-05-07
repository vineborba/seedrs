use clap::Parser;

use bootstrapper::{Options, PackageManagers, Techs};

#[derive(Parser, Debug)]
pub struct Args {
    /// Technologies that will be used in the project
    #[arg(short, long, value_delimiter = ',')]
    techs: Vec<Techs>,

    /// PackageManager to be used
    #[arg(short, long)]
    package_manager: Option<PackageManagers>,

    /// Project name
    name: String,
}

impl From<Args> for Options {
    fn from(value: Args) -> Self {
        Self {
            project_prefix: value.name,
            techs: value.techs,
            package_manager: value.package_manager,
        }
    }
}
