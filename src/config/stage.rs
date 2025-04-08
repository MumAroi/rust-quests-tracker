use std::fmt::{self, Formatter};

use anyhow::Result;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => "Local",
            Stage::Development => "Development",
            Stage::Production => "Production",
        };
        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "Local" => Ok(Stage::Local),
            "Development" => Ok(Stage::Development),
            "Production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("Invalid stage: {}", stage)),
        }
    }
}
