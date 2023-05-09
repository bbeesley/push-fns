use std::{
  collections::HashSet,
  fs::File,
  io::{self, Cursor, Read},
};

use zip::ZipArchive;

use push_fns::zip::create_zip;

#[test]
fn zip_contains_expected_files() {
  let file_name = "src/zip.rs";
  let mut files = HashSet::new();
  files.insert(file_name.to_string());
  let result = create_zip(&".".to_string(), files);

  // Create a buffer to hold the file from the zip
  let mut content_buf = Cursor::new(Vec::new());

  // Open the zip archive for reading
  let mut zip = ZipArchive::new(Cursor::new(result)).unwrap();

  // Extract the file from the archive
  let mut file_entry = zip.by_name(file_name).unwrap();

  // Copy it to the buffer
  io::copy(&mut file_entry, &mut content_buf).unwrap();

  // Read the same file in directly for an expected value
  let mut file = File::open(file_name).unwrap();

  // We'll store it in this buffer
  let mut file_buf = Vec::new();
  file.read_to_end(&mut file_buf).unwrap();

  // Compare the content we got from the archive to the content we got by reading from disk
  assert_eq!(file_buf, content_buf.into_inner());
}
