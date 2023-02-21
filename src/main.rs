fn main() {
    // TODO: Take in folder path as arg
    let paths = std::fs::read_dir("./ep/").unwrap();

    for path in paths {
        if let Ok(p) = path {
            let file = std::fs::File::open(p.path()).unwrap();
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
                    // println!("{} xxx {} xxx {}",
                    //          f.tag, f.ifd_num, f.display_value().with_unit(&exif));
                }
            } else {
                println!("{:?} - EXIF data not found", p.file_name());
                // TODO: Some kinda fallback, e.g. filename rules?
            }
        }
    }
}
