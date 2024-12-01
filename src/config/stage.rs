use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, Default)]
pub enum Stage {
    Local,
    #[default]
    Development,
    Production,
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => write!(f, "Local"),
            Stage::Development => write!(f, "Development"),
            Stage::Production => write!(f, "Production"),
        };

        write!(f, "{}", stage)
    }
}

impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "local" => Ok(Stage::Local),
            "development" => Ok(Stage::Development),
            "production" => Ok(Stage::Production),
            _ => Err(anyhow::anyhow!("Invalid stage")),
        }
    }
}
