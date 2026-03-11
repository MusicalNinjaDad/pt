/// Test cases making use of `tests/fixtures/**` - Single create, multiple modules to avoid
/// sequential execution. (cargo test executs _tests_ in parallel but for each _crate_ sequentially)
use std::{
    fs,
    path::{Path, PathBuf},
};

use assert_cmd::cargo::*;
use predicates::ord::eq;

use pt::{PyError, TestStatus, TestSuite, Traceback};

fn load_src(directory: &Path) -> TestSuite {
    let src = fs::read_to_string(directory.join("src.py")).unwrap();
    TestSuite::try_from(src).unwrap()
}

mod basic {
    use std::sync::LazyLock;

    use super::*;
    static ID: &str = "UID";
    static FIXTURES: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("./tests/fixtures/basic"));

    #[test]
    fn suite_from_src() {
        let suite = load_src(&FIXTURES);
        assert_eq!(2, suite.tests().collect::<Vec<_>>().len());
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
            &suite.test("test_passes").unwrap().status,
            TestStatus::Pass
        ));
        let expect_tb: Traceback = fs::read_to_string(FIXTURES.join("test_fails.tb"))
            .unwrap()
            .into();
        let tf_status = &suite.test("test_fails").unwrap().status;
        assert!(
            matches!(tf_status, TestStatus::Fail(err, tb)
                if matches!(err, PyError::AssertionError)
                && tb == &expect_tb
            ),
            "{tf_status:?}"
        );
    }

    #[test]
    fn assertion_rewrite_test_fails() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.test("test_fails").unwrap().report().unwrap();
        let expect_rpt = fs::read_to_string(FIXTURES.join("test_fails.rpt")).unwrap();
        assert_eq!(expect_rpt, report);
    }

    #[test]
    fn summary_report() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.summary_report();
        let expect_rpt = fs::read_to_string(FIXTURES.join("summary.rpt")).unwrap();
        assert_eq!(expect_rpt, report);
    }

    #[test]
    fn cli() {
        let mut pt_cmd = cargo_bin_cmd!("pt");
        pt_cmd.arg(FIXTURES.join("src.py").as_os_str());
        let expected_stdout = fs::read_to_string(FIXTURES.join("summary.rpt")).unwrap();
        pt_cmd.assert().stdout(eq(expected_stdout));
        pt_cmd.assert().code(1);
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
        assert_eq!(3, suite.tests().collect::<Vec<_>>().len());
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
            &suite.test("test_passes").unwrap().status,
            TestStatus::Pass
        ));
        for failed_test in ["test_fails", "test_seven_is_six"] {
            let expect_tb: Traceback =
                fs::read_to_string(FIXTURES.join(failed_test).with_added_extension("tb"))
                    .unwrap()
                    .into();
            let tf_status = &suite.test(failed_test).unwrap().status;
            assert!(
                matches!(tf_status, TestStatus::Fail(err, tb)
                    if matches!(err, PyError::AssertionError)
                    && tb == &expect_tb
                ),
                "{failed_test}: {tf_status:?}"
            );
        }
    }

    #[test]
    fn assertion_rewrite_test_seven_is_six() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.test("test_seven_is_six").unwrap().report().unwrap();
        let expect_rpt = fs::read_to_string(FIXTURES.join("test_seven_is_six.rpt")).unwrap();
        assert_eq!(expect_rpt, report);
    }

    #[test]
    fn summary_report() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.summary_report();
        let expect_rpt = fs::read_to_string(FIXTURES.join("summary.rpt")).unwrap();
        assert_eq!(expect_rpt, report);
    }

    #[test]
    fn cli() {
        let mut pt_cmd = cargo_bin_cmd!("pt");
        pt_cmd.arg(FIXTURES.join("src.py").as_os_str());
        let expected_stdout = fs::read_to_string(FIXTURES.join("summary.rpt")).unwrap();
        pt_cmd.assert().stdout(eq(expected_stdout));
        pt_cmd.assert().code(1);
    }
}

mod pass {
    use std::sync::LazyLock;

    use super::*;
    static ID: &str = "UID";
    static FIXTURES: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("./tests/fixtures/pass"));

    #[test]
    fn suite_from_src() {
        let suite = load_src(&FIXTURES);
        assert_eq!(1, suite.tests().collect::<Vec<_>>().len());
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
            &suite.test("test_passes").unwrap().status,
            TestStatus::Pass
        ));
    }

    #[test]
    fn assertion_rewrite_test_passes() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.test("test_passes").unwrap().report();
        assert!(report.is_none());
    }

    #[test]
    fn summary_report() {
        let mut suite = load_src(&FIXTURES);
        let stdout = fs::read_to_string(FIXTURES.join("stdout.out")).unwrap();
        suite.update_status(ID, &stdout);
        let report = suite.summary_report();
        let expect_rpt = fs::read_to_string(FIXTURES.join("summary.rpt")).unwrap();
        assert_eq!(expect_rpt, report);
    }

    #[test]
    fn cli() {
        let mut pt_cmd = cargo_bin_cmd!("pt");
        pt_cmd.arg(FIXTURES.join("src.py").as_os_str());
        let expected_stdout = fs::read_to_string(FIXTURES.join("summary.rpt")).unwrap();
        pt_cmd.assert().stdout(eq(expected_stdout));
        pt_cmd.assert().code(0);
    }
}

mod exitcodes {
    use super::*;

    #[test]
    fn invalid_src() {
        let mut pt_cmd = cargo_bin_cmd!("pt");
        pt_cmd.arg(PathBuf::from("./tests/fixtures/basic/stdout.out"));
        pt_cmd.assert().code(3);
    }
}