use clap::Parser;

use seedrs::{Options, PackageManagers, Techs};

#[derive(Parser, Debug)]
pub struct Args {
    /// Technologies that will be used in the project
    #[arg(short, long, value_delimiter = ',')]
    techs: Option<Vec<Techs>>,

    /// PackageManager to be used
    #[arg(short, long)]
    package_manager: Option<PackageManagers>,

    /// Project name
    name: Option<String>,
}

impl From<Args> for Options {
    fn from(value: Args) -> Self {
        let techs: Vec<Techs> = if let Some(techs) = value.techs {
            techs
        } else {
            vec![]
        };
        Self {
            techs,
            project_prefix: value.name,
            package_manager: value.package_manager,
        }
    }
}
