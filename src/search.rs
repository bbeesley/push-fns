use std::{collections::HashSet, fs, path::Path};

use glob::glob;

/// Returns a vector of files that match a glob pattern.
///
/// # Examples
///
/// ```
/// use push_fns::search::get_files_for_glob;
///
/// let files = get_files_for_glob("src/*");
/// assert_eq!(files.len(), 8);
/// ```
pub fn get_files_for_glob(pattern: &str) -> Vec<String> {
  glob(pattern)
    .unwrap()
    .filter_map(Result::ok)
    .map(|f| f.into_os_string().into_string().unwrap_or_default())
    .collect::<Vec<String>>()
}

fn make_patterns_absolute(path: &String, patterns: &[String]) -> Vec<String> {
  if path == &".".to_string() {
    return patterns.to_vec();
  }
  let p = Path::new(path);

  let base = fs::canonicalize(p).unwrap();
  patterns
    .iter()
    .map(|pattern| {
      base
        .join(pattern)
        .into_os_string()
        .into_string()
        .unwrap_or_default()
    })
    .collect()
}

/// Returns a set of files that match the include patterns and do not match the exclude patterns.
///
/// # Examples
///
/// ```
/// use push_fns::search::search;
///
/// let files = search(
///     &".".to_string(),
///     &["src/*".to_string(), ".devcontainer/*".to_string()],
///     &[],
/// );
/// println!("files: {:#?}", files);
/// assert_eq!(files.len(), 9);
/// ```
pub fn search(path: &String, include: &[String], exclude: &[String]) -> HashSet<String> {
  let all_excluded_files: HashSet<String> = make_patterns_absolute(path, exclude)
    .iter()
    .flat_map(|p| get_files_for_glob(p))
    .collect();

  let all_incuded_files: HashSet<String> = make_patterns_absolute(path, include)
    .iter()
    .flat_map(|p| get_files_for_glob(p))
    .filter(|f| !all_excluded_files.contains(f))
    .collect();

  all_incuded_files
}

#[cfg(test)]
mod tests {
  use std::env::current_dir;

  use super::*;

  #[test]
  fn internal() {
    let patterns = make_patterns_absolute(&"src".to_string(), &["search*".to_string()]);
    let here = current_dir().unwrap();
    let expected = vec![here
      .join("src/search*")
      .into_os_string()
      .into_string()
      .unwrap()];
    println!("patterns: {:#?}", patterns);
    assert_eq!(expected, patterns);
  }
}
