use clap::ValueEnum;
use colored::{ColoredString, Colorize};
use core::fmt;

use crate::PackageManager;

#[derive(Debug, Clone, ValueEnum, Default)]
pub enum Tech {
    React,
    ReactNative,
    NodeNest,

    #[default]
    Invalid,
}

impl fmt::Display for Tech {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Tech::React => "React",
            Tech::ReactNative => "React-Native",
            Tech::NodeNest => "NestJS",
            Tech::Invalid => "Invalid",
        };

        write!(f, "{str}")
    }
}

impl From<&str> for Tech {
    fn from(value: &str) -> Self {
        match value {
            "react" => Self::React,
            "react-native" | "rn" => Self::ReactNative,
            "nestjs" | "node-nest" | "node-nestjs" => Self::NodeNest,
            _ => Self::Invalid,
        }
    }
}

impl Tech {
    pub fn suffix(&self) -> String {
        match self {
            Tech::React => String::from("web"),
            Tech::ReactNative => String::from("app"),
            Tech::NodeNest => String::from("api"),
            Tech::Invalid => unreachable!(),
        }
    }

    pub fn values() -> Vec<Self> {
        vec![Self::React, Self::NodeNest, Self::ReactNative]
    }

    pub fn is_mobile(&self) -> bool {
        matches!(self, Self::ReactNative)
    }

    pub fn get_package_managers(&self) -> Vec<PackageManager> {
        match self {
            Self::ReactNative | Self::React | Self::NodeNest => {
                vec![
                    PackageManager::Npm,
                    PackageManager::Yarn,
                    PackageManager::Pnpm,
                ]
            }
            Self::Invalid => unreachable!(),
        }
    }

    pub fn colorize(&self) -> ColoredString {
        match self {
            Tech::React => format!("{self}").bright_blue().bold(),
            Tech::ReactNative => format!("{self}").cyan().bold(),
            Tech::NodeNest => format!("{self}").bright_red().bold(),
            Tech::Invalid => format!("{self}").black().bold(),
        }
    }
}
