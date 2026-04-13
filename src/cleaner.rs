use std::{
    env::consts,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use anyhow::{Context, Result, bail};
use log::info;
use which::which;

pub struct CargoCleaner {
    cargo: PathBuf,
    dry_run: bool,
}

impl CargoCleaner {
    pub fn new(dry_run: bool) -> Result<Self> {
        let cargo_program = format!("cargo{}", consts::EXE_SUFFIX);

        let cargo = which(&cargo_program)
            .with_context(|| format!("Unable to find {cargo_program} in PATH"))?;

        info!("cargo is @ {}", cargo.display());

        Ok(Self { cargo, dry_run })
    }

    pub fn clean<D>(&self, directory: D) -> Result<()>
    where
        D: AsRef<Path>,
    {
        info!("cleaning {}", directory.as_ref().display());

        if self.dry_run {
            return Ok(());
        }

        let mut cmd = Command::new(&self.cargo)
            .arg("clean")
            .stderr(Stdio::null())
            .stdout(Stdio::null())
            .current_dir(&directory)
            .spawn()?;

        let exit = cmd.wait()?;

        if !exit.success() {
            let code = exit.code().unwrap_or(-1);

            bail!(
                "cargo clean {} returned {code}",
                directory.as_ref().display(),
            )
        }

        Ok(())
    }
}
