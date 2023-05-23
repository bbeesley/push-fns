use std::{
  collections::HashSet,
  fs::{self},
  io::{Cursor, Write},
  path::{Path, PathBuf},
};

use zip::{write::FileOptions, ZipWriter};

/// Metadata for the symlink to direct resolvers to resources in a layer
pub struct SymLink {
  /// The path at which the symlink should be written
  pub path: String,
  /// The target that the symlink should point at
  pub target: String,
}

fn fill_zip(
  files: &HashSet<String>,
  archive: &mut Cursor<Vec<u8>>,
  base: PathBuf,
  symlink: Option<SymLink>,
) {
  let mut zip = ZipWriter::new(archive);
  files.iter().for_each(|f| {
    let full_path = Path::new(f);
    let mut file_path = Path::new(f);
    if full_path.is_absolute() {
      file_path = full_path.strip_prefix(&base).unwrap();
    }
    let contents = fs::read(full_path).unwrap();
    let options = FileOptions::default();
    zip
      .start_file(file_path.to_str().unwrap(), options)
      .unwrap();
    zip.write_all(&contents).unwrap();
  });
  if let Some(link) = symlink {
    let options = FileOptions::default();
    zip.add_symlink(link.path, link.target, options).unwrap();
  }
  zip.finish().unwrap();
}

/// Creates a zip file from a set of files.
///
/// # Examples
///
/// ```
/// use push_fn_lib::zip::create_zip;
/// use std::collections::HashSet;
///
/// let file_name: &str = "src/zip.rs";
/// let mut files: HashSet<String> = HashSet::new();
/// files.insert(file_name.to_string());
/// let result: Vec<u8> = create_zip(&".".to_string(), files, None);
/// ```
pub fn create_zip(path: &String, files: HashSet<String>, symlink: Option<SymLink>) -> Vec<u8> {
  let p = Path::new(path);
  let base = fs::canonicalize(p).unwrap();
  let buffer: Vec<u8> = Vec::new();
  let mut archive: Cursor<Vec<u8>> = Cursor::new(buffer);
  fill_zip(&files, &mut archive, base, symlink);
  archive.into_inner()
}

#[cfg(test)]
mod tests {
  use std::{
    fs::File,
    io::{copy, Read},
  };

  use zip::ZipArchive;

  use super::*;

  #[test]
  fn zip_contains_expected_files() {
    let file_name = "src/zip.rs";
    let mut files = HashSet::new();
    files.insert(file_name.to_string());
    let result = create_zip(&".".to_string(), files, None);

    // Create a buffer to hold the file from the zip
    let mut content_buf = Cursor::new(Vec::new());

    // Open the zip archive for reading
    let mut zip = ZipArchive::new(Cursor::new(result)).unwrap();

    // Extract the file from the archive
    let mut file_entry = zip.by_name(file_name).unwrap();

    // Copy it to the buffer
    copy(&mut file_entry, &mut content_buf).unwrap();

    // Read the same file in directly for an expected value
    let mut file = File::open(file_name).unwrap();

    // We'll store it in this buffer
    let mut file_buf = Vec::new();
    file.read_to_end(&mut file_buf).unwrap();

    // Compare the content we got from the archive to the content we got by reading from disk
    assert_eq!(file_buf, content_buf.into_inner());
  }
}
