use std::{
    env, fs,
    path::PathBuf,
    process::{Command, ExitCode},
};

use pt::{TestStatus, TestSuite};

fn main() -> ExitCode {
    let id = "PT_CLI";
    let src_path = PathBuf::from(env::args().nth(1).unwrap());
    let src = fs::read_to_string(src_path).unwrap();
    let Ok(mut suite) = TestSuite::try_from(src) else {
        return ExitCode::from(3);
    };
    let mut runner = Command::new("python");
    runner.args(["-c", &suite.runner(id)]);
    let python_output = String::from_utf8(runner.output().unwrap().stdout).unwrap();
    dbg!(&python_output);
    suite.update_status(id, &python_output);
    print!("{}", suite.summary_report());
    if suite
        .tests()
        .any(|test| matches!(test.status, TestStatus::Fail(_, _)))
    {
        return ExitCode::from(1);
    };
    ExitCode::SUCCESS
}

// Pytest exit codes:
//  Exit code 0:
//   All tests were collected and passed successfully
//  Exit code 1:
//   Tests were collected and run but some of the tests failed
//  Exit code 2:
//   Test execution was interrupted by the user
//  Exit code 3:
//   Internal error happened while executing tests
//  Exit code 4:
//   pytest command line usage error
//  Exit code 5:
//   No tests were collected
