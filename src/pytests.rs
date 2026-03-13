//!Handling individual tests & their status

use base_traits::AsStr;
use ruff_python_ast::StmtFunctionDef;

use crate::{
    Error, Exception, Traceback,
    failures::TracebackLine,
    multiline::{Location, Multiline, MultilineMut},
};

/// Owned details of single test. Does not store the original source text, only the AST.
/// Construct with `Pytest::from(ruff_python_ast::StmtFunctionDef)`
#[derive(Debug, PartialEq)]
pub(crate) struct TestDetails {
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

/// A single test, with references to the full module source and the test details.
#[derive(Debug, PartialEq)]
pub struct PythonTest<'name, 'suite, 'details> {
    pub testname: &'name str,
    pub full_src: &'suite str,
    pub test_ast: &'details StmtFunctionDef,
    pub status: &'details TestStatus,
}

impl PythonTest<'_, '_, '_> {
    /// Produce a test execution report.
    pub fn report(&self) -> Option<String> {
        enum Prefix {
            Text(String),
            Indent(usize),
        }

        let TestStatus::Fail(err, tb) = &self.status else {
            return None;
        };

        let mut frame_buf = String::new();
        match err {
            Exception::AssertionError => {
                let mut prefix = Prefix::Indent(0);
                for line in tb.lines() {
                    match line {
                        Ok(TracebackLine::TracebackHeader) => (),
                        Ok(TracebackLine::FrameHeader(frameheader)) => {
                            frame_buf.clear(); // We only want details from the last frame

                            let failure = Location::Line(frameheader.line_number);
                            let testfn_def = Location::Offset(self.test_ast.range.start().into());
                            let line_no = frameheader.line_number.to_string();
                            let indent = line_no.len() + 2;

                            frame_buf.push_line(0, ["==== ", frameheader.function_name, " ===="]);
                            self.full_src
                                .lines_from(&testfn_def)
                                .lines_to(&failure)
                                .for_each(|line| frame_buf.push_line(indent, [line]));

                            prefix = Prefix::Text(line_no);
                        }
                        Ok(TracebackLine::FrameContents { text }) => match prefix {
                            // TODO: compatibility python <3.13 ... need to manually recreate the
                            //       nice details that are in later version Tracebacks
                            Prefix::Text(lineno) => {
                                frame_buf.push_line(0, [&lineno, ": ", text]);
                                prefix = Prefix::Indent(lineno.len() + 2);
                            }
                            Prefix::Indent(indent) => {
                                frame_buf.push_line(indent, [text]);
                            }
                        },
                        Ok(TracebackLine::Exception(err)) => {
                            frame_buf.push_line(0, [err.as_str()]);
                        }
                        Err(err) => frame_buf.push_line(0, [err.to_string().as_str()]),
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
    Fail(Exception, Traceback),
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

impl TryFrom<(&str, &str)> for TestStatus {
    type Error = Error;

    fn try_from((status, traceback): (&str, &str)) -> Result<Self, Self::Error> {
        match status {
            "RUNNING" => Ok(Self::Running),
            "PASS" => Ok(Self::Pass),
            "FAIL" => Ok(Self::Fail(traceback.try_into()?, traceback.into())),
            _ => Err(Error::InvalidStatus(status.to_string())),
        }
    }
}
