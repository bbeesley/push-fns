use std::{
    collections::HashSet,
    fs::{self},
    io::{Cursor, Write},
    path::{Path, PathBuf},
};

use zip::{write::FileOptions, ZipWriter};

fn fill_zip(files: &HashSet<String>, archive: &mut Cursor<Vec<u8>>, base: PathBuf) {
    let mut zip = ZipWriter::new(archive);
    files.iter().for_each(|f| {
        let full_path = Path::new(f);
        let mut file_path = Path::new(f);
        if full_path.is_absolute() {
            file_path = full_path.strip_prefix(&base).unwrap();
        }
        let contents = fs::read(full_path).unwrap();
        let options = FileOptions::default();
        zip.start_file(file_path.to_str().unwrap(), options)
            .unwrap();
        zip.write_all(&contents).unwrap();
    });
    zip.finish().unwrap();
}

/// Creates a zip file from a set of files.
///
/// # Examples
///
/// ```
/// use push_fns::zip::create_zip;
/// use std::collections::HashSet;
///
/// let file_name: &str = "src/zip.rs";
/// let mut files: HashSet<String> = HashSet::new();
/// files.insert(file_name.to_string());
/// let result: Vec<u8> = create_zip(&".".to_string(), files);
/// ```
pub fn create_zip(path: &String, files: HashSet<String>) -> Vec<u8> {
    let p = Path::new(path);
    let base = fs::canonicalize(p).unwrap();
    let buffer: Vec<u8> = Vec::new();
    let mut archive: Cursor<Vec<u8>> = Cursor::new(buffer);
    fill_zip(&files, &mut archive, base);
    archive.into_inner()
}
