use reagent::Reagent;
use std::{fs, path::Path, process};

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

    if let Some(parent) = generated.parent() {
        match fs::create_dir_all(parent) {
            Ok(_) => (),
            Err(err) => {
                eprintln!("reagent: failed to create directory \"{}\": {}", parent.display(), err);
                process::exit(1);
            }
        }
    }

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
                ($template:expr, $generated:expr) => {{
                    generate(
                        &reagent,
                        include_str!(concat!("../templates/", $template)),
                        $generated,
                    );
                    println!("{}", $generated);
                }};
            }

            generate!("debian/source/format", "debian/source/format");
            generate!("debian/changelog", "debian/changelog");
            generate!("debian/compat", "debian/compat");
            generate!("debian/control", "debian/control");
            generate!("debian/copyright", "debian/copyright");
            generate!("debian/rules", "debian/rules");

            generate!("git/gitignore", ".gitignore");

            generate!("gitlab/gitlab-ci.yml", ".gitlab-ci.yml");

            if let Some(license) = reagent.license.as_ref() {
                match license.as_str() {
                    "MIT" => {
                        generate!("license/LICENSE.MIT", "LICENSE");
                    }
                    _ => {
                        eprintln!("reagent: no template for license \"{}\"", license);
                    }
                }
            }

            generate!("make/Makefile", "Makefile");

            generate!("rust/cargo", "Cargo.toml");
            generate!("rust/rustfmt", "rustfmt.toml");
        }
        Err(err) => {
            eprintln!("reagent: failed to parse config \"{}\": {}", config_path, err);
            process::exit(1);
        }
    }
}
