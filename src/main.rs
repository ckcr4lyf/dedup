use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
   /// Directory to look through
   target_directory: String,
}

fn main() {

    let args = Args::parse();
    let paths = std::fs::read_dir(args.target_directory).unwrap();

    for path in paths {
        if let Ok(p) = path {
            if let Ok(file) = std::fs::File::open(p.path()) {
                let mut bufreader = std::io::BufReader::new(&file);
                let exifreader = exif::Reader::new();
                
                if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
                    for f in exif.fields() {
                        match f.tag {
                            exif::Tag::DateTime => {
                                println!("{:?} - Found datetime as {}", p.file_name(), f.display_value())
                            },
                            _ => ()
                        }
                    }
                } else {
                    println!("{:?} - EXIF data not found", p.file_name());
                    // TODO: Some kinda fallback, e.g. filename rules?
                }
            } else {
                println!("Failed to open {:?}", p.file_name());
            }
        }
    }
}
