fn main() {
    println!("Hello, world!");
    let path = "./ep/043.jpg";

    let file = std::fs::File::open(path).unwrap();
    let mut bufreader = std::io::BufReader::new(&file);
    let exifreader = exif::Reader::new();
    
    if let Ok(exif) = exifreader.read_from_container(&mut bufreader) {
        for f in exif.fields() {
            // match f.tag {
            //     exif::Tag::DateTime => {
            //         println!("Found datetime as {}", f.display_value())
            //     },
            //     _ => ()
            // }
            println!("{} xxx {} xxx {}", f.tag, f.ifd_num, f.display_value().with_unit(&exif));
        }
    } else {
        println!("EXIF data not found");
    }
}
