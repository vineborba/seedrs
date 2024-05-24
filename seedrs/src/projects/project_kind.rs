use std::fmt::Display;

use crate::Tech;

#[derive(Debug)]
pub enum ProjectKind {
    Web,
    Mobile,
    Api,
}

impl ProjectKind {
    pub fn values() -> Vec<Self> {
        vec![Self::Web, Self::Mobile, Self::Api]
    }

    pub fn get_suffix(&self) -> String {
        match self {
            ProjectKind::Web => String::from("web"),
            ProjectKind::Mobile => String::from("app"),
            ProjectKind::Api => String::from("server"),
        }
    }

    pub fn get_techs(&self) -> Vec<Tech> {
        match self {
            ProjectKind::Web => vec![Tech::React],
            ProjectKind::Mobile => vec![Tech::ReactNative],
            ProjectKind::Api => vec![Tech::NodeNest],
        }
    }
}

impl Display for ProjectKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ProjectKind::Web => "web",
            ProjectKind::Mobile => "app",
            ProjectKind::Api => "api",
        };
        write!(f, "{str}")
    }
}
