use ramhorns::Content;
use serde::{Deserialize, Serialize};

pub use self::cargo::Cargo;
mod cargo;

pub use self::debian::Debian;
mod debian;

pub use self::git::Git;
mod git;

pub use self::license::License;
mod license;

#[derive(Debug, Default)]
pub struct Reagent {
    cargo: Cargo,
    debian: Debian,
}

/// Configuration for `reagent`, stored in `reagent.toml`
#[derive(Content, Debug, Deserialize, Serialize)]
pub struct Config {
    /// The name of the package: `"reagent"`
    pub package: Option<String>,
    /// The description of the package: `"Redox OS Standard Tests"`
    pub description: Option<String>,
    /// The repository or homepage of the package: `"https://gitlab.redox-os.org/redox-os/reagent"`
    pub url: Option<String>,
    /// The current version of the package: `"0.1.0"`
    pub version: Option<String>,
    /// The license used by the package: `"MIT"`
    pub license: Option<String>,
    /// The name of the package maintainer: `"Jeremy Soller"`
    pub name: Option<String>,
    /// The email of the package maintainer: `"jackpot51@gmail.com"`
    pub email: Option<String>,
}
