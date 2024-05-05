use clap::Parser;
use degit::options::Options;

#[derive(Parser, Debug)]
pub struct Args {
    /// URL of the repo to be cloned
    pub url: String,

    /// Optional destination to clone, defaults to "./"
    pub dest: Option<String>,

    /// Clone with ssh, the default is https
    #[arg(short, long)]
    pub ssh: bool,
}

impl From<Args> for Options {
    fn from(value: Args) -> Self {
        Self {
            url: value.url,
            dest: value.dest,
            ssh: value.ssh,
        }
    }
}

fn main() {
    let cli = Args::parse();

    if let Err(err) = degit::run(Options::from(cli)) {
        eprintln!("Exited with error! Err: {err}")
    };
}
