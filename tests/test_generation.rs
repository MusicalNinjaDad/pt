use std::fs;

use pt::{TestSuite, Traceback};

#[test]
fn test_generation() {
    let src = r"import pathlib


def test_fails():
    assert False


def test_passes():
    assert True
";
    let expected_runner = fs::read_to_string("./tests/fixtures/test.py").unwrap();
    let mut suite: TestSuite = src.try_into().unwrap();
    let runner = suite.runner("UID");
    assert_eq!(expected_runner, runner);
    let stdout = fs::read_to_string("./tests/fixtures/stdout.out").unwrap();
    let stderr = fs::read_to_string("./tests/fixtures/stderr.out").unwrap();
    suite.update_status("UID", &stdout, &stderr);
    assert!(matches!(&suite.tests["test_passes"].status, pt::TestStatus::Pass));
    let expect_tb: Traceback = stderr.as_str().into();
    assert!(matches!(&suite.tests["test_fails"].status, pt::TestStatus::Fail(tb) if tb == &expect_tb));
}
