pub fn scan_folder(folder_name: &std::ffi::OsStr) {
    let dir_entries = std::fs::read_dir(folder_name).expect("Given path was not a folder!");

    for entry in dir_entries {
        if let Ok(p) = entry {
            // println!("Gonna do {:?}", p.file_name());
            let metadata = p.metadata().expect("Fail to read file metadata");
            if metadata.is_dir() {
                // println!("Found folder: {:?}", p.file_name());
                // recursive
                scan_folder(std::path::Path::new(folder_name).join(p.file_name()).as_os_str());
                continue;
            }

            // Otherwise just read file
            scan_date(&std::path::Path::new(folder_name).join(p.file_name()));
        }
    }
}

const VALID_EXTENSIONS: [&str; 2] = ["jpg", "png"];

pub fn scan_date(file_path: &std::path::Path) {
    if let Some(ext) = file_path.extension() {
        let lcase = ext.to_ascii_lowercase();
        let l2 = lcase.to_str().unwrap();
        if VALID_EXTENSIONS.contains(&l2) == false {
            // Skip invalid extension
            return;
        }
    } else {
        println!("Could not get extension for file {:?}", file_path);
        return;
    }

    // Try and open the file
    if let Ok(file) = std::fs::File::open(file_path) {
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        
        if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
            for f in exif.fields() {
                match f.tag {
                    exif::Tag::DateTime => {

                        // Try and match the EXIF datetime
                        let exif_tag_val = f.display_value();
                        let set = regex::RegexSet::new(&[
                            r"^(\d{4})-(\d{2})-(\d{2})",
                        ]).unwrap();
                        let matches = set.matches(&exif_tag_val.to_string());
                        if matches.matched_any() == false {
                            println!("[EXIF] Could not find date match, for: {:?}", file_path);
                        }
                        // println!("{:?} - Found datetime as {}", file_path.file_name().unwrap(), f.display_value())
                    },
                    _ => ()
                }
            }
        } else {
            // println!("{:?} - EXIF data not found", file_path.file_name().unwrap());

            let set = regex::RegexSet::new(&[
                r"^IMG-(\d{8})-",
                r"^signal-(\d{4})-(\d{2})-(\d{2})",
                r"^(\d{4})-(\d{2})-(\d{2})",
                r"^(\d{8})_",
                r"^img(\d{8})_",
                r"^Screenshot_(\d{8})",
            ]).unwrap();
            let matches = set.matches(file_path.file_name().unwrap().to_str().unwrap());
            if matches.matched_any() == false {
                println!("[NO EXIF] Could not find a date match for: {:?}", file_path);
            }
        }
    }
}