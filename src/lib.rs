use ramhorns::{Content, Template};
use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};
use toml;

/// Configuration for `reagent`, stored in `reagent.toml`
#[derive(Content, Debug, Deserialize, Serialize)]
pub struct Reagent {
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
    /// Build for Redox OS
    #[serde(default)]
    pub redox: bool,
}

impl Reagent {
    /// Create a reagent instance from a toml file
    pub fn from_path<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let toml = fs::read_to_string(path)?;
        toml::from_str(&toml).map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))
    }

    /// Split a string based on reagent tags, returning the following:
    /// - Lines up to the <reagent> tag
    /// - Lines in between and including the tags
    /// - Lines after the </reagent> tag
    /// If there are no reagent tags, the entire file is assumed to be in between tags
    pub fn split(string: &str) -> (&str, &str, &str) {
        let mut start_opt = None;
        let mut end_opt = None;

        let mut last_index = 0;
        for (index, newline) in string.match_indices("\n") {
            let line = &string[last_index..index];

            if line.contains("<reagent>") {
                if start_opt.is_some() {
                    eprintln!("reagent: duplicate <reagent> tag");
                }
                start_opt = Some(last_index);
            }

            if line.contains("</reagent>") {
                if end_opt.is_some() {
                    eprintln!("reagent: duplicate </reagent> tag");
                }
                end_opt = Some(index + newline.len());
            }

            last_index = index + newline.len();
        }

        let start = start_opt.unwrap_or(0);
        let end = end_opt.unwrap_or(string.len());

        (&string[..start], &string[start..end], &string[end..])
    }

    /// Generate updated file based on input template
    /// If the file is empty, use the entire input file to generate it
    /// If the file is not empty, only generate the sections inside of reagent tags
    /// This starts from the line after a <reagent> tag, and ends on the line before a </reagent>
    /// tag
    pub fn generate(
        &self,
        input: &str,
        original_opt: Option<&str>,
    ) -> Result<String, ramhorns::Error> {
        let template = Template::new(input)?;
        let rendered = template.render(self);

        let rendered_split = Self::split(&rendered);
        let original_split = match original_opt {
            Some(original) => Self::split(original),
            None => rendered_split,
        };

        let mut updated = String::with_capacity(
            original_split.0.len() + rendered_split.1.len() + original_split.2.len(),
        );

        updated.push_str(original_split.0);
        updated.push_str(rendered_split.1);
        updated.push_str(original_split.2);

        Ok(updated)
    }
}
