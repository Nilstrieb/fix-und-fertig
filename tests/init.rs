mod helpers;
use helpers::*;

#[test]
fn init() {
    let dir = tmpdir();

    run_in(&dir, ["init"]).unwrap();

    assert!(dir.path().join(".fuf").is_dir());
    assert!(dir.path().join(".fuf").join("db").is_dir());
    assert!(dir.path().join(".fuf").join("db").join("objects").is_dir());
}

#[test]
fn double_init() {
    let dir = tmpdir();

    run_in(&dir, ["init"]).unwrap();
    assert!(run_in(&dir, ["init"]).is_err());
}
