#[derive(Debug)]
pub struct Options {
    pub url: String,
    pub dest: Option<String>,
    pub ssh: bool,
}
