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
            if let Some(v) = scan_date(&std::path::Path::new(folder_name).join(p.file_name())) {
                println!("We got {:?}", v);
            }
        }
    }
}

const VALID_EXTENSIONS: [&str; 2] = ["jpg", "png"];

pub fn scan_date(file_path: &std::path::Path) -> Option<ImageMetadata> {
    if let Some(ext) = file_path.extension() {
        let lcase = ext.to_ascii_lowercase();
        let l2 = lcase.to_str().unwrap();
        if VALID_EXTENSIONS.contains(&l2) == false {
            // Skip invalid extension
            return None;
        }
    } else {
        println!("Could not get extension for file {:?}", file_path);
        return None;
    }

    // Try and open the file
    if let Ok(file) = std::fs::File::open(file_path) {
        let mut bufreader = std::io::BufReader::new(&file);
        let exifreader = exif::Reader::new();
        
        if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
            // println!("Well we got exif");
            for f in exif.fields() {
                match f.tag {
                    exif::Tag::DateTime => {
                        // Try and match the EXIF datetime
                        let exif_tag_val = f.display_value();
                        // println!("And we got {}", exif_tag_val);
                        let re = regex::Regex::new(r"^(\d{4})-(\d{2})-(\d{2})").unwrap();

                        if re.is_match(&exif_tag_val.to_string()) {
                            return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: exif_tag_val.to_string() })
                        }
                        
                        println!("[EXIF] Could not find date match, for: {:?}", file_path);
                        return None
                    },
                    _ => ()
                }
            }

            return None;
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

                // Try and get modified
                if let Ok(modified_time) = file.metadata().unwrap().modified() {
                    // We are ok
                    return None;
                } else {
                    println!("[NO EXIF, NO MODIFIED] Could not find a date match for: {:?}", file_path);
                    return None;
                }
                // file.metadata().unwrap().created();

                // let datetime: chrono::DateTime<chrono::Utc> = file.metadata().unwrap().modified().unwrap().into();
                // println!("[NO EXIF] Metadata is {:?}", datetime.to_rfc3339());
            }

            return None;
        }
    }

    return None;
}

#[derive(Debug)]
pub struct ImageMetadata {
    pub path: std::ffi::OsString,
    pub date_str: String,
}