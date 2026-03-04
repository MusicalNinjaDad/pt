#![feature(if_let_guard)]

use indexmap::IndexMap;
use ruff_python_ast::{Stmt, StmtFunctionDef};
use ruff_python_parser::{ParseError, parse_module};
use std::{fmt::Display, str::FromStr};

mod traceback;
pub use traceback::Traceback;

use crate::traceback::TbLine;

#[derive(Debug, PartialEq)]
pub struct TestSuite {
    src: String,
    pub tests: IndexMap<String, Pytest>,
}

#[derive(Debug, PartialEq)]
pub struct Pytest {
    code: StmtFunctionDef,
    pub status: TestStatus,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum TestStatus {
    #[default]
    NoRun,
    Pass,
    Fail(PyError, Traceback),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PyError {
    AssertionError,
    Other,
}

impl Display for PyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AssertionError => write!(f, "AssertionError"),
            Self::Other => todo!(),
        }
    }
}

impl From<StmtFunctionDef> for Pytest {
    //TODO: convert to TryFrom and handle not a valid function_def
    fn from(fndef: StmtFunctionDef) -> Self {
        Self {
            code: fndef,
            status: Default::default(),
        }
    }
}

impl TryFrom<&str> for TestSuite {
    type Error = ParseError;
    fn try_from(src: &str) -> Result<Self, Self::Error> {
        let tests: IndexMap<String, Pytest> = parse_module(src)?
            .into_suite()
            .into_iter()
            .filter_map(|stmt| -> Option<(String, Pytest)> {
                match stmt {
                    Stmt::FunctionDef(function) if function.name.as_str().starts_with("test_") => {
                        Some((function.name.to_string(), function.into()))
                    }
                    _ => None,
                }
            })
            .collect();
        Ok(Self {
            src: src.to_string(),
            tests,
        })
    }
}

impl From<&str> for PyError {
    fn from(traceback: &str) -> Self {
        let lastline = traceback.lines().last().unwrap();
        match lastline {
            "AssertionError" => Self::AssertionError,
            _ => Self::Other,
        }
    }
}

impl TestSuite {
    pub fn runner<ID: AsRef<str>>(&self, id: ID) -> String {
        let mut test_runner = String::new();
        test_runner.push_python_line(0, ["if __name__ == \"__main__\":"]);
        test_runner.push_python_line(1, ["from traceback import TracebackException"]);
        test_runner.push_python_line(1, ["import sys"]);
        self.tests.keys().for_each(|testname| {
            test_runner.push_newline();
            test_runner.push_python_line(1, ["print(\"", id.as_ref(), " ", testname, " RUNNING\")"]);
            test_runner.push_python_line( 1, ["try:"]);
            test_runner.push_python_line( 2, [testname, "()"]);
            test_runner.push_python_line( 1, ["except Exception:"]);
            test_runner.push_python_line(2,["TracebackException.from_exception(sys.exception(), capture_locals=True).print(file=sys.stdout)"]);
            test_runner.push_python_line(2,["print(\"", id.as_ref(), " ", testname, " FAIL\")"]);
            test_runner.push_python_line(1, ["else:"]);
            test_runner.push_python_line(2,["print(\"", id.as_ref(), " ", testname, " PASS\")"]);
        });
        self.src.clone() + "\n\n" + &test_runner
    }

    pub fn update_status(&mut self, id: &str, stdout: &str) {
        let mut tb_buf = String::new();
        for line in stdout.lines() {
            let mut words = line.split_ascii_whitespace();
            match words.next() {
                Some("Traceback") => {
                    tb_buf = line.to_string();
                }
                Some(id_) if id_ == id => {
                    if let Some(testname) = words.next()
                        && let Some(test) = self.tests.get_mut(testname)
                    {
                        match words.next() {
                            Some("PASS") => test.status = TestStatus::Pass,
                            Some("FAIL") => {
                                test.status =
                                    TestStatus::Fail(tb_buf.as_str().into(), tb_buf.clone().into())
                            }
                            Some("RUNNING") => (),
                            _ => todo!(),
                        }
                    }
                }
                Some(_) => {
                    tb_buf.push('\n');
                    tb_buf.push_str(line);
                }
                None => {
                    tb_buf.push('\n');
                    tb_buf.push_str(line);
                }
            }
        }
    }
}

trait StringBuffer {
    fn push_line<'strs>(&mut self, indent: usize, contents: impl IntoIterator<Item = &'strs str>);
    fn push_python_line<'strs>(
        &mut self,
        indent: usize,
        contents: impl IntoIterator<Item = &'strs str>,
    );
    fn push_newline(&mut self);
}

impl StringBuffer for String {
    fn push_newline(&mut self) {
        self.push('\n');
    }

    /// `indent` with spaces, concatenate `contents`, end with `\n`
    fn push_line<'strs>(&mut self, indent: usize, contents: impl IntoIterator<Item = &'strs str>) {
        for _ in 0..indent {
            self.push(' ');
        }
        for text in contents {
            self.push_str(text);
        }
        self.push_newline();
    }

    /// `indent` with 4 x n spaces, concatenate `contents`, end with `\n`
    fn push_python_line<'strs>(
        &mut self,
        indent: usize,
        contents: impl IntoIterator<Item = &'strs str>,
    ) {
        self.push_line(4 * indent, contents);
    }
}

enum Location {
    Position(usize),
    Line(usize),
}

impl TestSuite {
    /// Returns an Iterator over the lines from `start` (inclusive) to `end` (exclusive)
    fn source(&self, start: &Location, end: &Location) -> impl Iterator<Item = &str> {
        let start_line = match start {
            Location::Position(pos) => self.src[0..*pos].lines().count(),
            Location::Line(_) => todo!(),
        };
        let end_line = match end {
            Location::Position(_) => todo!(),
            Location::Line(line) => *line - 1,
        };
        self.src
            .lines()
            .skip(start_line)
            .take(end_line - start_line)
    }
}

impl TestSuite {
    pub fn failure_report(&self, testname: &str) -> Option<String> {
        enum Prefix<'str> {
            LineNumber(&'str str),
            Indent(usize),
        }

        match &self.tests[testname].status {
            TestStatus::Fail(err, tb) => match err {
                PyError::AssertionError => {
                    let mut frame_buf = String::new();
                    let mut prefix = Prefix::Indent(0);
                    for line in tb.lines() {
                        match line {
                            TbLine::TracebackHeader => (),
                            TbLine::FrameHeader(frameheader) => {
                                let failure = Location::Line(
                                    usize::from_str(frameheader.line_number).unwrap(),
                                );
                                let testfn_def = Location::Position(
                                    self.tests[testname].code.range.start().into(),
                                );
                                let indent = frameheader.line_number.len() + 2;
                                frame_buf.clear();
                                frame_buf
                                    .push_line(0, ["==== ", frameheader.function_name, " ===="]);
                                for line in self.source(&testfn_def, &failure) {
                                    frame_buf.push_line(indent, [line]);
                                }
                                prefix = Prefix::LineNumber(frameheader.line_number);
                            }
                            TbLine::FrameContents { text } => match prefix {
                                Prefix::LineNumber(lineno) => {
                                    frame_buf.push_line(0, [lineno, ": ", text]);
                                    prefix = Prefix::Indent(lineno.len() + 2);
                                }
                                Prefix::Indent(indent) => {
                                    frame_buf.push_line(indent, [text]);
                                }
                            },
                            TbLine::Exception(err) => {
                                frame_buf.push_line(0, [err.to_string().as_str()]);
                            }
                        }
                    }
                    Some(frame_buf)
                }
                _ => todo!(),
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        let src = r"def test_passes():
    assert True
";
        let pytests: TestSuite = src.try_into().unwrap();
        assert_eq!(1, pytests.tests.len());
        assert!(pytests.tests.contains_key("test_passes"));
    }

    #[test]
    fn import_and_two_tests() {
        let src = r"import pathlib


def test_fails():
    assert False


def test_passes():
    assert True
";
        let pytests: TestSuite = src.try_into().unwrap();
        assert_eq!(2, pytests.tests.len());
        assert!(pytests.tests.contains_key("test_fails"));
        assert!(pytests.tests.contains_key("test_passes"));
    }
}
