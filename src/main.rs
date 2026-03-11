#![feature(try_trait_v2)]
use std::{
    convert::Infallible,
    env, fs, io,
    path::PathBuf,
    process::{Command, ExitCode},
    string::FromUtf8Error,
};

use core::ops::{ControlFlow, Try};
use std::{
    io::{Write, stderr},
    ops::FromResidual,
    process::Termination,
};

use pt::{TestStatus, TestSuite};

fn main() -> Exit<()> {
    let id = "PT_CLI";
    // TODO use clap or similar, leaving this unwrap until we do
    let src_path = PathBuf::from(env::args().nth(1).unwrap());
    let src = match fs::read_to_string(&src_path) {
        Ok(src) => src,
        Err(err) => return Exit::InternalError(format!("Error opening {src_path:?}: {err}")),
    };
    let mut suite = match TestSuite::try_from(src) {
        Ok(suite) => suite,
        Err(err) => return Exit::InternalError(format!("Error parsing {src_path:?}: {err}")),
    };
    let mut runner = Command::new("python");
    runner.args(["-c", &suite.runner(id)]);
    let python_output = String::from_utf8(runner.output()?.stdout)?;
    suite.update_status(id, &python_output);
    print!("{}", suite.summary_report());
    if suite
        .tests()
        .any(|test| matches!(test.status, TestStatus::Fail(_, _)))
    {
        return Exit::TestsFailed;
    };
    Exit::Ok(())
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

/// Custom ExitCode handler. Using this rather than just calling `exit()` to allow for proper
/// unwinding and Drops to occur.
enum Exit<T: Termination> {
    Ok(T),
    TestsFailed,
    InternalError(String),
    Err(u8, String),
}

/// Boilerplate
impl<T: Termination> Try for Exit<T> {
    type Output = T;

    type Residual = Exit<Infallible>;

    fn from_output(output: Self::Output) -> Self {
        Self::Ok(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            Self::Ok(v) => ControlFlow::Continue(v),
            Self::TestsFailed => ControlFlow::Break(Exit::TestsFailed),
            Self::InternalError(msg) => ControlFlow::Break(Exit::InternalError(msg)),
            Self::Err(code, msg) => ControlFlow::Break(Exit::Err(code, msg)),
        }
    }
}

/// Boilerplate
impl<T: Termination> FromResidual<Exit<Infallible>> for Exit<T> {
    fn from_residual(residual: Exit<Infallible>) -> Self {
        match residual {
            Exit::TestsFailed => Exit::TestsFailed,
            Exit::InternalError(msg) => Exit::InternalError(msg),
            Exit::Err(code, msg) => Exit::Err(code, msg),
        }
    }
}

/// Boilerplate Conversion
impl<T: Termination, E: Into<Exit<T>>> FromResidual<std::result::Result<Infallible, E>>
    for Exit<T>
{
    fn from_residual(residual: std::result::Result<Infallible, E>) -> Self {
        match residual {
            Result::Err(e) => e.into(),
        }
    }
}

/// Write msg to stderr then identify correct ExitCode
impl<T: Termination> Termination for Exit<T> {
    fn report(self) -> ExitCode {
        match self {
            Exit::Ok(ok) => ok.report(),
            Exit::TestsFailed => ExitCode::from(1),
            Exit::InternalError(msg) => {
                _ = stderr().write(msg.as_bytes());
                ExitCode::from(3)
            }
            Exit::Err(code, msg) => {
                _ = stderr().write(msg.as_bytes());
                code.into()
            }
        }
    }
}

/// UTF8 Errors return InternalError
impl<T: Termination> From<FromUtf8Error> for Exit<T> {
    fn from(err: FromUtf8Error) -> Self {
        Exit::InternalError(err.to_string())
    }
}

/// IO Errors return InternalError
impl<T: Termination> From<io::Error> for Exit<T> {
    fn from(err: io::Error) -> Self {
        Exit::InternalError(err.to_string())
    }
}
