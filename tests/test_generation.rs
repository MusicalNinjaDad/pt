use std::{
    fs,
    path::{Path, PathBuf},
};

use pt::{PyError, TestStatus, TestSuite, Traceback};

fn load_src(directory: &Path) -> TestSuite {
    let src = fs::read_to_string(directory.join("src.py")).unwrap();
    TestSuite::try_from(src.as_str()).unwrap()
}

mod basic {
    use std::sync::LazyLock;

    use super::*;
    static ID: &str = "UID";
    static FIXTURES: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("./tests/fixtures/basic"));

    #[test]
    fn suite_from_src() {
        let suite = load_src(&FIXTURES);
        assert_eq!(2, suite.tests.len());
    }

    #[test]
    fn runner() {
        let suite = load_src(&FIXTURES);
        let expected_runner = fs::read_to_string(FIXTURES.join("run.py")).unwrap();
        assert_eq!(expected_runner, suite.runner(ID));
    }

    #[test]
    fn parse_status() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        assert!(matches!(
            &suite.tests["test_passes"].status,
            TestStatus::Pass
        ));
        let expect_tb: Traceback = fs::read_to_string(FIXTURES.join("test_fails.tb"))
            .unwrap()
            .into();
        let tf_status = &suite.tests["test_fails"].status;
        assert!(
            matches!(tf_status, TestStatus::Fail(err, tb)
                if matches!(err, PyError::AssertionError)
                && tb == &expect_tb
            ),
            "{tf_status:?}"
        );
    }

    #[test]
    fn assertion_rewrite_test_passes() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.tests["test_passes"].failure_report();
        assert!(report.is_none());
    }

    #[test]
    fn assertion_rewrite_test_fails() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.tests["test_fails"].failure_report().unwrap();
        assert_eq!("failed", report);
    }
}

mod complex {
    use std::sync::LazyLock;

    use super::*;
    static ID: &str = "UID";
    static FIXTURES: LazyLock<PathBuf> =
        LazyLock::new(|| PathBuf::from("./tests/fixtures/complex"));

    #[test]
    fn suite_from_src() {
        let suite = load_src(&FIXTURES);
        assert_eq!(3, suite.tests.len());
    }

    #[test]
    fn runner() {
        let suite = load_src(&FIXTURES);
        let expected_runner = fs::read_to_string(FIXTURES.join("run.py")).unwrap();
        assert_eq!(expected_runner, suite.runner(ID));
    }

    #[test]
    fn parse_status() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        assert!(matches!(
            &suite.tests["test_passes"].status,
            TestStatus::Pass
        ));
        for failed_test in ["test_fails", "test_seven_is_six"] {
            let expect_tb: Traceback =
                fs::read_to_string(FIXTURES.join(failed_test).with_added_extension("tb"))
                    .unwrap()
                    .into();
            let tf_status = &suite.tests[failed_test].status;
            assert!(
                matches!(tf_status, TestStatus::Fail(err, tb)
                    if matches!(err, PyError::AssertionError)
                    && tb == &expect_tb
                ),
                "{failed_test}: {tf_status:?}"
            );
        }
    }
}
