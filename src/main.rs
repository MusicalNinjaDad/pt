#![feature(never_type)]
#![feature(try_trait_v2)]
use std::{env, fs, io, path::PathBuf, process::Command, string::FromUtf8Error};

use std::process::Termination as _T;

use exit_safely::Termination;
use try_v2::{Try, Try_ConvertResult};

use pt::{TestStatus, TestSuite};

fn main() -> Exit<()> {
    let id = "PT_CLI";

    let src_path: PathBuf = env::args()
        .nth(1)
        .ok_or_else(|| Exit::InvalidInvocation("Please provide a file to test".to_string()))?
        .into();
    let src = fs::read_to_string(&src_path)
        .map_err(|err| Exit::InternalError(format!("Error opening {src_path:?}: {err}")))?;
    let mut suite = TestSuite::try_from(src)
        .map_err(|err| Exit::InternalError(format!("Error parsing {src_path:?}: {err}")))?;

    let mut runner = Command::new("python");
    runner.args(["-c", &suite.runner(id)]);

    let python_output = String::from_utf8(runner.output()?.stdout)?;

    suite.update_status(id, &python_output)?;
    print!("{}", suite.summary_report());
    Exit::from(suite)
}

/// Custom ExitCode handler. Using this rather than just calling `exit()` to allow for proper
/// unwinding and Drops to occur.
/// based upon Pytest exit codes:
///  Exit code 0:
///   All tests were collected and passed successfully
///  Exit code 1:
///   Tests were collected and run but some of the tests failed
///  Exit code 2:
///   Test execution was interrupted by the user
///  Exit code 3:
///   Internal error happened while executing tests
///  Exit code 4:
///   pytest command line usage error
///  Exit code 5:
///   No tests were collected
#[derive(Debug, Termination, Try, Try_ConvertResult)]
#[repr(u8)]
enum Exit<T: _T> {
    Ok(T) = 0,
    TestsFailed = 1,
    InternalError(String) = 3,
    InvalidInvocation(String) = 4,
}

impl From<TestSuite> for Exit<()> {
    fn from(suite: TestSuite) -> Self {
        if suite
            .tests()
            .any(|test| matches!(test.status, TestStatus::Fail(_, _)))
        {
            return Exit::TestsFailed;
        };
        Exit::Ok(())
    }
}

/// UTF8 Errors return InternalError
impl<T: _T> From<FromUtf8Error> for Exit<T> {
    fn from(err: FromUtf8Error) -> Self {
        Exit::InternalError(err.to_string())
    }
}

/// IO Errors return InternalError
impl<T: _T> From<io::Error> for Exit<T> {
    fn from(err: io::Error) -> Self {
        Exit::InternalError(err.to_string())
    }
}

impl<T: _T> From<pt::Error> for Exit<T> {
    fn from(err: pt::Error) -> Self {
        Exit::InternalError(err.to_string())
    }
}
