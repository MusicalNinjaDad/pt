use std::{fs, path::PathBuf};

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
    suite.update_status(id, &stdout);
    assert!(matches!(
        &suite.tests["test_passes"].status,
        pt::TestStatus::Pass
    ));
    let expect_tb: Traceback = fs::read_to_string(fixtures.join("test_fails.tb"))
        .unwrap()
        .into();
    let tf_status = &suite.tests["test_fails"].status;
    assert!(
        matches!(tf_status, pt::TestStatus::Fail(tb) if tb == &expect_tb),
        "{failed_test}: {tf_status:?}"
    );
}

#[test]
fn complex() {
    let id = "UID";
    let fixtures = PathBuf::from("./tests/fixtures/complex");
    let src = fs::read_to_string(fixtures.join("src.py")).unwrap();
    let mut suite = TestSuite::try_from(src.as_str()).unwrap();
    assert_eq!(3, suite.tests.len());
    let expected_runner = fs::read_to_string(fixtures.join("run.py")).unwrap();
    assert_eq!(expected_runner, suite.runner(id));
    let stdout = fs::read_to_string(fixtures.join("stdout.out")).unwrap();
    suite.update_status(id, &stdout);
    assert!(matches!(
        &suite.tests["test_passes"].status,
        pt::TestStatus::Pass
    ));
    for failed_test in ["test_fails", "test_seven_is_six"] {
        let expect_tb: Traceback =
            fs::read_to_string(fixtures.join(failed_test).with_added_extension("tb"))
                .unwrap()
                .into();
        let tf_status = &suite.tests[failed_test].status;
        assert!(
            matches!(tf_status, pt::TestStatus::Fail(tb) if tb == &expect_tb),
            "{tf_status:?}"
        );
    }
}
