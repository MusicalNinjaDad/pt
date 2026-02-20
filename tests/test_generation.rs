use std::{
    fs,
    path::{Path, PathBuf},
};

use pt::{TestSuite, Traceback};

#[test]
fn basic() {
    let id = "UID";
    let fixtures = PathBuf::from("./tests/fixtures/basic");
    let src = fs::read_to_string(fixtures.join("src.py")).unwrap();
    let mut suite = TestSuite::try_from(src.as_str()).unwrap();
    assert_eq!(2, suite.tests.len());
    let expected_runner = fs::read_to_string(fixtures.join("run.py")).unwrap();
    assert_eq!(expected_runner, suite.runner(id));
    let stdout = fs::read_to_string(fixtures.join("stdout.out")).unwrap();
    let stderr = fs::read_to_string(fixtures.join("stderr.out")).unwrap();
    suite.update_status(id, &stdout, &stderr);
    assert!(matches!(
        &suite.tests["test_passes"].status,
        pt::TestStatus::Pass
    ));
    let expect_tb: Traceback = stderr.as_str().into();
    assert!(
        matches!(&suite.tests["test_fails"].status, pt::TestStatus::Fail(tb) if tb == &expect_tb)
    );
}

#[test]
fn complex() {
    let id = "UID";
    let fixtures = PathBuf::from("./tests/fixtures/complex");
    let src = fs::read_to_string(fixtures.join("src.py")).unwrap();
    let mut suite = TestSuite::try_from(src.as_str()).unwrap();
    let expected_runner = fs::read_to_string(fixtures.join("run.py")).unwrap();
    assert_eq!(3, suite.tests.len());
    assert_eq!(expected_runner, suite.runner(id));
    let stdout = fs::read_to_string(fixtures.join("stdout.out")).unwrap();
    let stderr = fs::read_to_string(fixtures.join("stderr.out")).unwrap();
    suite.update_status(id, &stdout, &stderr);
    assert!(matches!(
        &suite.tests["test_passes"].status,
        pt::TestStatus::Pass
    ));
    let expect_tb: Traceback = stderr.as_str().into();
    assert!(
        matches!(&suite.tests["test_fails"].status, pt::TestStatus::Fail(tb) if tb == &expect_tb)
    );
}
