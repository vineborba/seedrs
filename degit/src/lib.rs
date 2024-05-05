use std::{
    fs,
    path::Path,
    process::{exit, Command, Stdio},
};

pub mod options;

use anyhow::{bail, Context, Result};
use colored::Colorize;
use options::Options;
use regex::Regex;

pub struct Repository {
    pub owner: String,
    pub name: String,
    pub https: String,
    pub ssh: String,
    pub host: String,
}

impl Repository {
    pub fn from_url(url: &str) -> Result<Self> {
        let re = Regex::new(
            r"^(https?://|git@)?(?<host>github|gitlab(\.[\w\-]*)?)\.(?<dns>com|org)(/|:)(?<repo_owner>[\w\-]+)/(?<repo_name>[\w\-.]+)\.git$",
        )?;

        let Some(captures) = re.captures(url) else {
            bail!("Invalid URL provided");
        };

        let owner = String::from(&captures["repo_owner"]);
        let name = String::from(&captures["repo_name"]);
        let host = String::from(&captures["host"]);
        let domain = format!("{}.{}", &host, &captures["dns"]);
        let ssh = format!("git@{}:{}/{}", &domain, &owner, &name);
        let https = format!("https://{}/{}/{}", &domain, &owner, &name);
        Ok(Self {
            name,
            owner,
            https,
            ssh,
            host,
        })
    }

    pub fn check_destination(&self, dest: &Option<String>) -> Result<String> {
        let p = if let Some(d) = dest.as_deref() {
            let mut p = String::from(d);
            if p.ends_with('/') {
                p.push_str(&self.name);
            }
            p
        } else {
            format!("./{}", self.name)
        };

        let parsed_path = Path::new(&p);

        let exists = parsed_path
            .try_exists()
            .with_context(|| "Can't verify destination.")?;

        if exists {
            bail!("Can't write to destination. Path already exists!");
        }

        Ok(String::from(
            parsed_path.to_str().expect("Failed to parse path"),
        ))
    }
}

pub fn run(opt: Options) -> Result<()> {
    let repository = Repository::from_url(&opt.url)?;
    let clone_path = repository.check_destination(&opt.dest)?;

    let url = if opt.ssh {
        &repository.ssh
    } else {
        &repository.https
    };

    let git_process = Command::new("git")
        .arg("clone")
        .arg(url)
        .arg(&clone_path)
        .stdout(Stdio::null())
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .with_context(|| "Failed to spawn git process")?;

    println!(
        "Cloning {} to {} from {}",
        format!("{}/{}", &repository.owner, &repository.name).cyan(),
        &clone_path.green().bold(),
        &repository.host.white().bold()
    );

    let output = git_process.wait_with_output().with_context(|| "")?;
    if !output.status.success() {
        let exit_code = output.status.code().expect("Failed to get exit status");
        let error = format!(
            "git process failed, exited with code {}",
            exit_code.clone().to_string().bold().red()
        );
        eprintln!("{error}");
        exit(exit_code)
    }

    println!(
        "Successfuly cloned {} to {}",
        format!("{}/{}", &repository.owner, &repository.name).cyan(),
        &clone_path.green().bold(),
    );

    fs::remove_dir_all(format!("{clone_path}/.git"))?;

    Ok(())
}
