#[derive(Debug)]
pub enum Techs {
    React,
    ReactNative,
    NodeExpress,
    NodeNest,

    Invalid,
}

impl From<&str> for Techs {
    fn from(value: &str) -> Self {
        match value {
            "node" => Self::NodeExpress,
            "react" => Self::React,
            "react-native" => Self::ReactNative,
            "node-nestjs" => Self::NodeNest,
            _ => Self::Invalid,
        }
    }
}

impl Techs {
    pub fn name(&self, project: &str) -> String {
        let suffix = match self {
            Techs::React => "web",
            Techs::ReactNative => "app",
            Techs::NodeExpress | Techs::NodeNest => "api",
            Techs::Invalid => unreachable!(),
        };

        format!("{project}-{suffix}")
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::ReactNative)
    }

    pub fn create_args(&self, project_prefix: &str, package_manager: String) -> Vec<String> {
        let mut args = vec![];

        let project_name = self.name(project_prefix);

        match self {
            Techs::React => {
                args.push("create".to_string());
                args.push("vite@latest".to_string());
                args.push(project_name);
                args.push("--".to_string());
                args.push("--template".to_string());
                args.push("react-ts".to_string());
            }
            Techs::NodeNest => {
                args.push("exec".to_string());
                args.push("--yes".to_string());
                args.push("@nestjs/cli".to_string());
                args.push("new".to_string());
                args.push(project_name);
                args.push("--".to_string());
                args.push("-p".to_string());
                args.push(package_manager);
            }
            Techs::ReactNative => {
                args.push("create".to_string());
                args.push("expo-app".to_string());
                args.push(project_name);
                args.push("--".to_string());
                args.push("--template".to_string());
                args.push("blank-typescript".to_string());
            }
            Techs::NodeExpress => todo!(),
            Techs::Invalid => unreachable!(),
        };

        args
    }
}
