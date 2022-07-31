pub(crate) use std::{fs::File, io::Write, path::Path};

/// Write to local storage using File
pub(crate) fn write_storage_local(string_to_write: &str, path_file: String) {
    let path: &Path = Path::new(&path_file);
    let display: std::path::Display = path.display();
    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file: File = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    println!("file:create: {:?}", file);
    // Write the 'lorem_ipsum' strin to 'file', return `io::Result<()>`
    match file.write_all(string_to_write.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
