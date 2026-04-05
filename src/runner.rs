use std::collections::BTreeMap;
use std::path::PathBuf;
use std::process::Command;

use crate::errors::{HarmonyAppError, Result};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommandSpec {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub cwd: PathBuf,
    pub env: BTreeMap<String, String>,
}

impl CommandSpec {
    pub fn display(&self) -> String {
        let rendered_args = self
            .args
            .iter()
            .map(|value| {
                if value.contains(' ') {
                    format!("\"{value}\"")
                } else {
                    value.clone()
                }
            })
            .collect::<Vec<_>>()
            .join(" ");
        if rendered_args.is_empty() {
            self.program.display().to_string()
        } else {
            format!("{} {}", self.program.display(), rendered_args)
        }
    }
}

pub trait CommandRunner {
    fn run(&mut self, spec: &CommandSpec) -> Result<()>;
}

pub struct RealCommandRunner;

impl CommandRunner for RealCommandRunner {
    fn run(&mut self, spec: &CommandSpec) -> Result<()> {
        let mut command = Command::new(&spec.program);
        command.args(&spec.args).current_dir(&spec.cwd);
        for (key, value) in &spec.env {
            command.env(key, value);
        }

        let status = command
            .status()
            .map_err(|source| HarmonyAppError::CommandSpawn {
                program: spec.program.display().to_string(),
                cwd: spec.cwd.clone(),
                source,
            })?;

        if status.success() {
            Ok(())
        } else {
            Err(HarmonyAppError::CommandFailed {
                program: spec.program.display().to_string(),
                cwd: spec.cwd.clone(),
                code: status.code(),
            })
        }
    }
}
