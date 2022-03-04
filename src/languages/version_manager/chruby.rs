use crate::{
    logging::{HasLogger, Logger},
    Bootstrap, IdempotentBootstrap, Success, IsInstalled,
};
use std::process::Command;

// https://www.nerdfonts.com/cheat-sheet: nf-oct-ruby
const ICON: char = '';

pub struct Chruby {
    logger: Logger,
}

impl Default for Chruby {
    fn default() -> Self {
        Self {
            logger: Logger::new(ICON, "chruby"),
        }
    }
}

impl HasLogger for Chruby {
    fn logger(&self) -> &Logger {
        &self.logger
    }
}

impl IsInstalled for Chruby {
    fn is_installed(&self) -> bool {
        crate::command_exists("chruby")
    }
}

impl Bootstrap for Chruby {
    fn bootstrap(&self) -> Result<crate::Success, crate::Error> {
        let mut child = Command::new("wget")
            .arg("-0")
            .arg("chruby-0.3.9.tar.gz")
            .arg("https://github.com/postmodern/chruby/archive/v0.3.9.tar.gz")
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("tar")
            .arg("-xzvf")
            .arg("chruby-0.3.9.tar.gz")
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("tar")
            .arg("-xzvf")
            .arg("chruby-0.3.9.tar.gz")
            .spawn()?;
        child.wait()?;

        let mut child = Command::new("sudo")
            .current_dir("chruby-0.3.9")
            .arg("make")
            .arg("install")
            .spawn()?;
        child.wait()?;

        Ok(Success::DidIt)
    }
}

impl IdempotentBootstrap for Chruby {}