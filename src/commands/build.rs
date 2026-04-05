use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::cli::BuildCommand;
use crate::commands::command_env;
use crate::config::AppContext;
use crate::errors::{HarmonyAppError, Result};
use crate::runner::{CommandRunner, CommandSpec};

pub fn run<R: CommandRunner, W: Write>(
    command: &BuildCommand,
    cwd: &Path,
    runner: &mut R,
    stdout: &mut W,
) -> Result<()> {
    let app = AppContext::load(&command.common, cwd)?;
    let plan = build_plan(&app);

    if command.common.dry_run {
        writeln!(stdout, "[dry-run] {}", plan.command.display())?;
        writeln!(
            stdout,
            "[dry-run] copy {} -> {}",
            plan.source.display(),
            plan.destination.display()
        )?;
        return Ok(());
    }

    runner.run(&plan.command)?;
    if !plan.source.exists() {
        return Err(HarmonyAppError::MissingFile {
            path: plan.source.clone(),
        });
    }
    remove_legacy_shared_artifact(&plan.legacy_shared_destination)?;
    if let Some(parent) = plan.destination.parent() {
        fs::create_dir_all(parent).map_err(|source| HarmonyAppError::io(parent, source))?;
    }
    fs::copy(&plan.source, &plan.destination)
        .map_err(|source| HarmonyAppError::io(&plan.destination, source))?;

    writeln!(
        stdout,
        "Built Rust staticlib and copied it to {}",
        plan.destination.display()
    )?;
    Ok(())
}

pub(crate) struct BuildPlan {
    pub command: CommandSpec,
    pub source: PathBuf,
    pub destination: PathBuf,
    pub legacy_shared_destination: PathBuf,
}

pub(crate) fn build_plan(app: &AppContext) -> BuildPlan {
    let mut args = vec![
        "rustc".to_string(),
        "--lib".to_string(),
        "--manifest-path".to_string(),
        app.project.manifest_path.display().to_string(),
        "--target".to_string(),
        app.config.target.clone(),
    ];
    if app.config.profile_dir == "release" {
        args.push("--release".to_string());
    }
    args.push("--".to_string());
    args.push("--crate-type".to_string());
    args.push("staticlib".to_string());

    let source = app
        .project
        .static_artifact_path(&app.config.target, &app.config.profile_dir);
    let destination = app
        .config
        .output_dir
        .join("entry")
        .join("src")
        .join("main")
        .join("cpp")
        .join("libs")
        .join(&app.config.abi)
        .join(format!("lib{}.a", app.project.lib_name));

    BuildPlan {
        command: CommandSpec {
            program: PathBuf::from("cargo"),
            args,
            cwd: app.project.project_dir.clone(),
            env: command_env(app),
        },
        source,
        destination,
        legacy_shared_destination: app
            .config
            .output_dir
            .join("entry")
            .join("src")
            .join("main")
            .join("cpp")
            .join("libs")
            .join(&app.config.abi)
            .join(format!("lib{}.so", app.project.lib_name)),
    }
}

fn remove_legacy_shared_artifact(path: &Path) -> Result<()> {
    if path.exists() {
        fs::remove_file(path).map_err(|source| HarmonyAppError::io(path, source))?;
    }
    Ok(())
}
