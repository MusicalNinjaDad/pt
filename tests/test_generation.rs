use std::fs;

use pt::TestSuite;

#[test]
fn test_generation() {
    let src = r"import pathlib


def test_fails():
    assert False


def test_passes():
    assert True
";
    let expected_runner = fs::read_to_string("./tests/fixtures/test.py").unwrap();
    let suite: TestSuite = src.try_into().unwrap();
    let runner = suite.runner("UID");
    assert_eq!(expected_runner, runner);
}
