#[derive(Debug, Default)]
pub struct Debian {
    source_format: String,
    changelog: String,
    compat: String,
    control: String,
    copyright: String,
    rules: String,
}
