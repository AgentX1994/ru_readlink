use ru_readlink::readlink::read_link;
use ru_readlink::settings::Settings;

fn main() {
    let settings = Settings::from_args();
    for file in &settings.files {
        match read_link(file, &settings.canonicalize) {
            Ok(path) => {
                print!("{}", path.display());
                if settings.output_delimiter {
                    if settings.zero {
                        print!("\0");
                    } else {
                        println!();
                    }
                }
            }
            Err(e) => {
                if !settings.quiet {
                    eprintln!("Error with file \"{}\": {}", file.display(), e);
                }
                std::process::exit(1);
            }
        }
    }
}
