use std::collections::HashMap;

use clap::Parser;
use log::info;
use scan::ImageMetadata;

mod scan;

#[derive(Parser, Debug)]
struct Args {
   /// Directory to look through
   target_directory: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    let mut map: HashMap<u128, ImageMetadata> = HashMap::new();
    info!("Starting dedupe on {}...", args.target_directory);
    scan::scan_folder(&mut map, &std::ffi::OsString::from(args.target_directory));

    info!("Finished! Total {} unique images.", map.len());

    // let paths = std::fs::read_dir(args.target_directory).unwrap();

    // for path in paths {
    //     if let Ok(p) = path {
    //         if let Ok(file) = std::fs::File::open(p.path()) {
    //             let mut bufreader = std::io::BufReader::new(&file);
    //             let exifreader = exif::Reader::new();
                
    //             if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
    //                 for f in exif.fields() {
    //                     match f.tag {
    //                         exif::Tag::DateTime => {
    //                             println!("{:?} - Found datetime as {}", p.file_name(), f.display_value())
    //                         },
    //                         _ => ()
    //                     }
    //                 }
    //             } else {
    //                 println!("{:?} - EXIF data not found", p.file_name());
    //                 // TODO: Some kinda fallback, e.g. filename rules?
    //             }
    //         } else {
    //             println!("Failed to open {:?}", p.file_name());
    //         }
    //     }
    // }
}
