use std::fs;

use pt::generate;

#[test]
fn test_generation() {
    let src = r"import pathlib

def test_fails():
    assert False

def test_passes():
    assert True
";
    let expected_runner = fs::read_to_string("./tests/fixtures/test.py").unwrap();
    let runner = generate(src.to_string()).unwrap();
    assert_eq!(expected_runner, runner);
}
