use std::{collections::HashSet, env::current_dir};

use glob::glob;

/// Returns a vector of files that match a glob pattern.
///
/// # Examples
///
/// ```
/// use push_fn_lib::search::get_files_for_glob;
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
  let base = current_dir().unwrap().join(path);
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
/// use push_fn_lib::search::search;
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
  use std::env;

  use super::*;

  const NUM_FILES: usize = 7;

  #[test]
  fn test_absolute_patterns() {
    let patterns = make_patterns_absolute(&"src".to_string(), &["search*".to_string()]);
    let here = current_dir().unwrap();
    let expected = vec![here
      .join("src")
      .join("search*")
      .into_os_string()
      .into_string()
      .unwrap()];
    println!("patterns: {:#?}", patterns);
    assert_eq!(expected, patterns);
  }

  #[test]
  fn test_get_files_for_glob() {
    let files = get_files_for_glob("src/*");
    assert_eq!(files.len(), NUM_FILES + 1);
  }

  #[test]
  fn search_works_with_different_paths() {
    let files = search(&"src".to_string(), &["*".to_string()], &[]);
    assert_eq!(files.len(), NUM_FILES + 1);
    let include = match env::consts::OS {
      "windows" => "src\\*",
      _ => "src/*",
    };
    let files = search(&".".to_string(), &[include.to_string()], &[]);
    assert_eq!(files.len(), NUM_FILES + 1);
  }

  #[test]
  fn search_works_with_single_exclude() {
    let files = search(
      &"src".to_string(),
      &["*".to_string()],
      &["search*".to_string()],
    );
    assert_eq!(files.len(), NUM_FILES);
  }

  #[test]
  fn search_works_with_multiple_excludes() {
    let files = search(
      &"src".to_string(),
      &["*".to_string()],
      &["search*".to_string(), "zip*".to_string()],
    );
    assert_eq!(files.len(), NUM_FILES - 1);
  }

  #[test]
  fn search_works_with_multiple_includes() {
    let files = search(
      &".".to_string(),
      &["src/*".to_string(), ".devcontainer/*".to_string()],
      &[],
    );
    println!("files: {:#?}", files);
    assert_eq!(files.len(), NUM_FILES + 2);
  }
}
