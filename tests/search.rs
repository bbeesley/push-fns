
use push_fns::search::{get_files_for_glob, search};

#[test]
fn test_get_files_for_glob() {
    let files = get_files_for_glob("src/*");
    assert_eq!(files.len(), 8);
}

#[test]
fn search_works_with_different_paths() {
    let files = search(&"src".to_string(), &["*".to_string()], &[]);
    assert_eq!(files.len(), 8);
    let files = search(&".".to_string(), &["src/*".to_string()], &[]);
    assert_eq!(files.len(), 8);
}

#[test]
fn search_works_with_single_exclude() {
    let files = search(
        &"src".to_string(),
        &["*".to_string()],
        &["search*".to_string()],
    );
    assert_eq!(files.len(), 7);
}

#[test]
fn search_works_with_multiple_excludes() {
    let files = search(
        &"src".to_string(),
        &["*".to_string()],
        &["search*".to_string(), "zip*".to_string()],
    );
    assert_eq!(files.len(), 6);
}

#[test]
fn search_works_with_multiple_includes() {
    let files = search(
        &".".to_string(),
        &["src/*".to_string(), ".devcontainer/*".to_string()],
        &[],
    );
    println!("files: {:#?}", files);
    assert_eq!(files.len(), 9);
}
