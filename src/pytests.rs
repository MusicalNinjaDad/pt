//!Handling individual tests & their status

use std::str::FromStr;

use base_traits::AsStr;
use ruff_python_ast::StmtFunctionDef;

use crate::{Location, PyError, StringBuffer, TestSuite, Traceback, failures::TracebackLine};

/// A single test, with references to the related test suite and the test details.
#[derive(Debug, PartialEq)]
pub struct PythonTest<'suite, 'details> {
    pub full_src: &'suite str,
    pub test_ast: &'details StmtFunctionDef,
    pub status: &'details TestStatus,
}

/// Owned details of single test. Does not store the original source text, only the AST.
/// Construct with `Pytest::from(ruff_python_ast::StmtFunctionDef)`
#[derive(Debug, PartialEq)]
pub struct TestDetails {
    pub ast: StmtFunctionDef,
    pub status: TestStatus,
}

impl From<StmtFunctionDef> for TestDetails {
    //TODO: convert to TryFrom and include logic to validate whether this is a test function.
    fn from(fndef: StmtFunctionDef) -> Self {
        Self {
            ast: fndef,
            status: Default::default(),
        }
    }
}

impl PythonTest<'_, '_> {
    /// Returns an Iterator over the lines from `start` (inclusive) to `end` (exclusive)
    fn source(&self, start: &Location, end: &Location) -> impl Iterator<Item = &str> {
        let start_line = match start {
            Location::Position(pos) => self.full_src[0..*pos].lines().count(),
            Location::Line(_) => todo!(),
        };
        let end_line = match end {
            Location::Position(_) => todo!(),
            Location::Line(line) => *line - 1,
        };
        self.full_src
            .lines()
            .skip(start_line)
            .take(end_line - start_line)
    }

    /// Produce a test execution report.
    pub fn report(&self) -> Option<String> {
        enum Prefix<'str> {
            LineNumber(&'str str),
            Indent(usize),
        }

        let TestStatus::Fail(err, tb) = &self.status else {
            return None;
        };

        let mut frame_buf = String::new();
        match err {
            PyError::AssertionError => {
                let mut prefix = Prefix::Indent(0);
                for line in tb.lines() {
                    match line {
                        TracebackLine::TracebackHeader => (),
                        TracebackLine::FrameHeader(frameheader) => {
                            frame_buf.clear(); // We only want details from the last frame

                            let failure =
                                Location::Line(usize::from_str(frameheader.line_number).unwrap());
                            let testfn_def = Location::Position(self.test_ast.range.start().into());
                            let indent = frameheader.line_number.len() + 2;

                            frame_buf.push_line(0, ["==== ", frameheader.function_name, " ===="]);
                            for line in self.source(&testfn_def, &failure) {
                                frame_buf.push_line(indent, [line]);
                            }

                            prefix = Prefix::LineNumber(frameheader.line_number);
                        }
                        TracebackLine::FrameContents { text } => match prefix {
                            // TODO: compatibility python <3.13 ... need to manually recreate the
                            //       nice details that are in later version Tracebacks
                            Prefix::LineNumber(lineno) => {
                                frame_buf.push_line(0, [lineno, ": ", text]);
                                prefix = Prefix::Indent(lineno.len() + 2);
                            }
                            Prefix::Indent(indent) => {
                                frame_buf.push_line(indent, [text]);
                            }
                        },
                        TracebackLine::Exception(err) => {
                            frame_buf.push_line(0, [err.as_str()]);
                        }
                    }
                }
            }
            _ => todo!(),
        };
        Some(frame_buf)
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum TestStatus {
    #[default]
    NoRun,
    Running,
    Pass,
    Fail(PyError, Traceback),
}

impl AsStr for TestStatus {
    fn as_str(&self) -> &str {
        match self {
            TestStatus::NoRun => "NO RUN",
            TestStatus::Running => "RUNNING",
            TestStatus::Pass => "PASS",
            TestStatus::Fail(_, _) => "FAIL",
        }
    }
}

impl From<(&str, &str)> for TestStatus {
    fn from((status, traceback): (&str, &str)) -> Self {
        match status {
            "RUNNING" => Self::Running,
            "PASS" => Self::Pass,
            "FAIL" => Self::Fail(traceback.into(), traceback.into()),
            _ => todo!("Make fallible"),
        }
    }
}
