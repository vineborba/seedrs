use std::{
    fs,
    path::Path,
    process::{exit, Command},
};

use anyhow::{Context, Result};
use clap::Parser;
use regex::Regex;

#[derive(Parser, Debug)]
struct Args {
    /// URL of the repo to be cloned
    url: String,

    /// Optional destination to clone, defaults to "./"
    dest: Option<String>,

    /// Clone with ssh, the default is https
    #[arg(short, long)]
    ssh: bool,
}

struct Metadata {
    owner: String,
    name: String,
    https: String,
    ssh: String,
    host: String,
}

fn main() -> Result<()> {
    let cli = Args::parse();

    let metadata = parse_path(&cli.url)?;
    let clone_path = check_destination(&metadata.name, &cli.dest)?;

    let url = if cli.ssh {
        &metadata.ssh
    } else {
        &metadata.https
    };

    let mut git = Command::new("git");

    match git.arg("clone").arg(url).arg(&clone_path).status() {
        Ok(_) => (),
        Err(err) => {
            eprintln!("Couldn't clone the repository, exited with code {err}")
        }
    };

    fs::remove_dir_all(format!("{clone_path}/.git"))?;

    Ok(())
}

fn parse_path(url: &str) -> Result<Metadata> {
    let re = Regex::new(
        r"^(https?://|git@)?(?<host>github|gitlab(\.[\w\-]*)?)\.(?<dns>com|org)(/|:)(?<repo_owner>[\w\-]+)/(?<repo_name>\w+)\.git$",
    )?;

    let Some(captures) = re.captures(url) else {
        panic!("Invalid url!")
    };

    let owner = String::from(&captures["repo_owner"]);
    let name = String::from(&captures["repo_name"]);
    let host = String::from(&captures["host"]);
    let domain = format!("{}.{}", &host, &captures["dns"]);
    let ssh = format!("git@{}:{}/{}", &domain, &owner, &name);
    let https = format!("https://{}/{}/{}", &domain, &owner, &name);

    Ok(Metadata {
        name,
        owner,
        https,
        ssh,
        host,
    })
}

fn check_destination(name: &str, dest: &Option<String>) -> Result<String> {
    let p = if let Some(d) = dest.as_deref() {
        let mut p = String::from(d);
        if p.ends_with('/') {
            p.push_str(name);
        }
        p
    } else {
        format!("./{}", name)
    };

    let parsed_path = Path::new(&p);

    let exists = parsed_path
        .try_exists()
        .with_context(|| "Can't verify destination.")?;

    if exists {
        eprintln!("Can't write to destination. Path already exists!");
        exit(1);
    }

    Ok(String::from(
        parsed_path.to_str().expect("Failed to parse path"),
    ))
}
