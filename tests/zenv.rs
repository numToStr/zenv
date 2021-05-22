use std::path::PathBuf;

use zenv::Zenv;

#[test]
fn zenv_basic() {
    let z = Zenv::new(PathBuf::from("tests/.env.basic"), false)
        .parse()
        .unwrap();

    assert_eq!(z.get("BASIC").unwrap(), "basic");
    assert_eq!(z.get("EMPTY").unwrap(), "");
    assert_eq!(z.get("SINGLE_QUOTES").unwrap(), "single_quotes");
    assert_eq!(z.get("DOUBLE_QUOTES").unwrap(), "double_quotes");
}

#[test]
fn zenv_expanded() {
    let z = Zenv::new(PathBuf::from("tests/.env.expanded"), true)
        .parse()
        .unwrap();

    assert_eq!(z.get("BASIC").unwrap(), "basic");
    assert_eq!(z.get("EXPANDED").unwrap(), "basic-expanded");
    assert_eq!(z.get("DOUBLE_EXPANDED").unwrap(), "basic-basic-expanded");

    assert_eq!(z.get("EXPANDED_NEW").unwrap(), "basic_expanded");
    assert_eq!(
        z.get("DOUBLE_EXPANDED_NEW").unwrap(),
        "basic_basic-basic-expanded"
    );
}
