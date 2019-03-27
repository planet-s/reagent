use reagent::Reagent;
use std::{fs, process};
use std::path::Path;

fn generate<P: AsRef<Path>>(reagent: &Reagent, input: &str, generated: P) {
    let generated = generated.as_ref();

    let original_opt = if generated.is_file() {
        match fs::read_to_string(generated) {
            Ok(ok) => Some(ok),
            Err(err) => {
                eprintln!("reagent: failed to read \"{}\": {}", generated.display(), err);
                process::exit(1);
            }
        }
    } else {
        None
    };

    let updated = match reagent.generate(input, original_opt.as_ref().map(|x| x.as_str())) {
        Ok(ok) => ok,
        Err(err) => {
            eprintln!("reagent: failed to generate \"{}\": {}", generated.display(), err);
            process::exit(1);
        }
    };

    match fs::write(generated, updated) {
        Ok(_) => (),
        Err(err) => {
            eprintln!("reagent: failed to write \"{}\": {}", generated.display(), err);
            process::exit(1);
        }
    }
}

fn main() {
    let config_path = "reagent.toml";
    match Reagent::from_path(config_path) {
        Ok(reagent) => {
            println!("{:#?}", reagent);

            macro_rules! generate {
                ($template:expr, $generated:expr) => ({
                    generate(
                        &reagent,
                        include_str!(concat!("../templates/", $template)),
                        $generated
                    );
                });
            }

            generate!("cargo/Cargo.toml", "Cargo.toml");
            generate!("cargo/rustfmt.toml", "rustfmt.toml");
            generate!("git/ignore", ".gitignore");
            if let Some(license) = reagent.license.as_ref() {
                match license.as_str() {
                    "MIT" => {
                        generate!("license/MIT", "LICENSE");
                    },
                    _ => {
                        eprintln!("reagent: no template for license \"{}\"", license);
                    }
                }
            }
        },
        Err(err) => {
            eprintln!("reagent: failed to parse config \"{}\": {}", config_path, err);
            process::exit(1);
        }
    }
}
