use ramhorns::Template;
use reagent::Config;
use std::{fs, process};
use toml;

fn main() {
    match fs::read_to_string("reagent.toml") {
        Ok(toml) => match toml::from_str::<Config>(&toml) {
            Ok(config) => {
                println!("{:#?}", config);

                let template = Template::new(include_str!("../templates/license/MIT")).unwrap();
                let rendered = template.render(&config);
                fs::write("LICENSE", rendered).unwrap();
            },
            Err(err) => {
                eprintln!("reagent: failed to parse reagent.toml: {}", err);
                process::exit(1);
            }
        },
        Err(err) => {
            eprintln!("reagent: failed to read reagent.toml: {}", err);
            process::exit(1);
        }
    }
}
