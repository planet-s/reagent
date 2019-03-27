use ramhorns::Template;
use reagent::Config;
use std::{fs, process};

fn main() {
    let config_path = "reagent.toml";
    match Config::from_path(config_path) {
        Ok(config) => {
            println!("{:#?}", config);

            let template = Template::new(include_str!("../templates/license/MIT")).unwrap();
            let rendered = template.render(&config);
            fs::write("LICENSE", rendered).unwrap();
        },
        Err(err) => {
            eprintln!("reagent: failed to parse \"{}\": {}", config_path, err);
            process::exit(1);
        }
    }
}
