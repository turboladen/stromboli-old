pub mod homebrew;

pub use homebrew::Homebrew;

use crate::{logging::HasLogger, Error, Success};
use std::ffi::OsStr;

// nf-oct-package/f487 from https://www.nerdfonts.com/cheat-sheet.
const ICON: char = '';

pub trait OsPackageManager: HasLogger {
    const NAME: &'static str;

    /// Use the package manager to install a package.
    ///
    fn install_package<S>(&self, package_name: S) -> Result<Success, Error>
    where
        S: AsRef<OsStr>;

    /// Use the package manager to install a list of packages.
    ///
    fn install_package_list<I, S>(&self, package_names: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>;

    /// Install all of the packages you want.
    ///
    fn install_all_packages(&self) -> Result<Success, Error>;

    fn install_package_with_logging<S>(&self, package_name: S) -> Result<Success, Error>
    where
        S: AsRef<OsStr>,
    {
        self.logger()
            .log_sub_heading_group("install-package", || self.install_package(package_name))
    }

    fn install_package_list_with_logging<I, S>(&self, package_names: I) -> Result<Success, Error>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        self.logger()
            .log_sub_heading_group("install-package", || self.install_package_list(package_names))
    }

    /// Wrapper around `install_all_packages()`, but adds log messages to the start & end of
    /// that call.
    ///
    fn install_all_packages_with_logging(&self) -> Result<Success, Error> {
        self.logger()
            .log_sub_heading_group("install-all-packages", || self.install_all_packages())
    }
}
