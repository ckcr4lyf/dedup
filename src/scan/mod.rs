use std::{io::{Read, Seek}, collections::HashMap};

use log::{debug,info,error, warn, trace};


pub fn scan_folder(map: &mut HashMap<u128, Vec<ImageMetadata>>, folder_name: &std::ffi::OsStr) {
    debug!("Going to scan folder {}", folder_name.to_str().unwrap());
    let dir_entries = std::fs::read_dir(folder_name).expect("Given path was not a folder!");

    for entry in dir_entries {
        if let Ok(p) = entry {
            // println!("Gonna do {:?}", p.file_name());
            let metadata = p.metadata().expect("Fail to read file metadata");
            if metadata.is_dir() {
                // println!("Found folder: {:?}", p.file_name());
                // recursive
                scan_folder(map, std::path::Path::new(folder_name).join(p.file_name()).as_os_str());
                continue;
            }

            // Otherwise just read file
            if let Some(v) = scan_date(&std::path::Path::new(folder_name).join(p.file_name())) {
                match map.get_mut(&v.hash) {
                    Some(existing) => {
                        warn!("{:?} is a dupe of {:?}!", v.path, existing[0].file_name);
                        existing.push(v);
                    },
                    None => {
                        info!("Got new image. Path: {:?}", v.path);
                        let new_vec = vec![v];
                        map.insert(new_vec[0].hash, new_vec);
                    }
                }
            }
        }
    }
}

const VALID_EXTENSIONS: [&str; 2] = ["jpg", "png"];

pub fn scan_date(file_path: &std::path::Path) -> Option<ImageMetadata> {
    if let Some(ext) = file_path.extension() {
        let lcase = ext.to_ascii_lowercase();
        let l2 = lcase.to_str().unwrap();

        // Skip invalid extension
        if VALID_EXTENSIONS.contains(&l2) == false {
            return None;
        }
    } else {
        error!("Could not get extension for file {:?}", file_path);
        return None;
    }

    let file_name = file_path.file_name().unwrap().to_str().unwrap().to_string();
    debug!("Going to parse file {}", file_name);


    let mut file = match std::fs::File::open(file_path) {
        Ok(file) => file,
        Err(e) => {
            error!("Failed to open file: {}", e);
            return None
        }
    };    

    // Read entire file into memory to hash it
    let metadata = file.metadata().expect("failed to read metadata");
    let mut buf = vec![0; metadata.len() as usize];
    file.read_exact(&mut buf).expect("failed to read");
    let hash = xxhash_rust::xxh3::xxh3_128(&buf);

    // seek back to beginning for EXIF
    if let Err(e) = file.seek(std::io::SeekFrom::Start(0)) {
        error!("failed to seek to beginning of file! {:?}", e);
        return None;
    }
    
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();

    debug!("Going to attempt to parse EXIF data");
    
    if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
        debug!("Found valid EXIF data");

        for f in exif.fields() {
            trace!("Found EXIF tag {:04X?}", f.tag.1);
            match f.tag {
                exif::Tag::DateTime | exif::Tag::DateTimeOriginal => {
                    // Try and match the EXIF datetime
                    let exif_tag_val = f.display_value();
                    debug!("Found relevant EXIF tag! Value: {}", exif_tag_val);

                    let exif_re = regex::Regex::new(r"^(\d{4})-(\d{2})-(\d{2})").unwrap();
                    if let Some(cap) = exif_re.captures(&exif_tag_val.to_string()) {
                        let dst = format!("{}-{}-{}", &cap[1], &cap[2], &cap[3]);
                        return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(dst), hash, file_name });
                    }
                    
                    error!("Found DateTime EXIF tag, but couldn't find date match, for: {:?}", file_path);
                    return None
                },
                _ => ()
            }
        }

        warn!("Could not find relevant EXIF tag for {:?}!", file_path);
    } else {
        warn!("No valid EXIF data for {:?}!", file_path)
    }

    debug!("Going to attempt to match regex filename...");
    
    // Handle names such as IMG-20180523-WA0013.jpg aka WhatsApp format
    let whatsapp_re = regex::Regex::new(r"^IMG-(\d{8})-").unwrap();
    if let Some(cap) = whatsapp_re.captures(&file_name) {
        let dst = format!("{}-{}-{}", &cap[1][0..4], &cap[1][4..6], &cap[1][6..8]);
        return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(dst), hash, file_name });
    }
    
    // Handles names such as signal-2020-11-17-104012.jpg aka Signal format
    let signal_re = regex::Regex::new(r"^signal-(\d{4})-(\d{2})-(\d{2})").unwrap();
    if let Some(cap) = signal_re.captures(&file_name) {
        let dst = format!("{}-{}-{}", &cap[1], &cap[2], &cap[3]);
        return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(dst), hash, file_name });
    }

    // Handles names such as YYYY-MM-DD 
    let custom_1 = regex::Regex::new(r"^(\d{4})-(\d{2})-(\d{2})").unwrap();
    if let Some(cap) = custom_1.captures(&file_name) {
        let dst = format!("{}-{}-{}", &cap[1], &cap[2], &cap[3]);
        return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(dst), hash, file_name });
    }

    // Handles names such as YYYY-MM-DD 
    let custom_2 = regex::Regex::new(r"^(\d{8})_").unwrap();
    if let Some(cap) = custom_2.captures(&file_name) {
        let dst = format!("{}-{}-{}", &cap[1][0..4], &cap[1][4..6], &cap[1][6..8]);
        return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(dst), hash, file_name });
    }

    // Handles names such as YYYYMMDD 
    let custom_3 = regex::Regex::new(r"^img(\d{8})_").unwrap();
    if let Some(cap) = custom_3.captures(&file_name) {
        let dst = format!("{}-{}-{}", &cap[1][0..4], &cap[1][4..6], &cap[1][6..8]);
        return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(dst), hash, file_name });
    }

    // Handles names such as Screenshot_YYYYMMDD 
    let custom_4 = regex::Regex::new(r"^Screenshot_(\d{8})").unwrap();
    if let Some(cap) = custom_4.captures(&file_name) {
        let dst = format!("{}-{}-{}", &cap[1][0..4], &cap[1][4..6], &cap[1][6..8]);
        return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(dst), hash, file_name });
    }
    
    warn!("Could not find a Regex match for {:?}! Will treat as NO_EXIF", file_path);
    return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: None, hash, file_name });

    // let date_modified = metadata.modified().expect("failed to read date modified");
    let date_modified: chrono::DateTime<chrono::Utc> = match metadata.modified() {
        Ok(dm) => dm.into(),
        Err(e) => {
            error!("Failed to read Date Modified! {}", e);
            return None;
        }
    };

    debug!("Obtained Date Modified as {}", date_modified);
    return Some(ImageMetadata { path: file_path.as_os_str().to_os_string(), date_str: Some(date_modified.format("%Y-%m").to_string()), hash, file_name });
}

#[derive(Debug)]
pub struct ImageMetadata {
    pub path: std::ffi::OsString,
    pub file_name: String,
    pub date_str: Option<String>,
    pub hash: u128,
}