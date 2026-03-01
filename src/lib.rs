use indexmap::IndexMap;
use ruff_python_ast::{Stmt, StmtFunctionDef};
use ruff_python_parser::{ParseError, parse_module};

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
pub struct Traceback {
    text: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PyError {
    AssertionError,
    Other,
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

impl From<String> for Traceback {
    fn from(text: String) -> Self {
        Self { text }
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
        let newline = "\n";
        let mut test_runner: String = "if __name__ == \"__main__\":".to_string() + newline;
        push_python_line(
            &mut test_runner,
            1,
            ["from traceback import TracebackException"],
        );
        push_python_line(&mut test_runner, 1, ["import sys"]);
        self.tests.keys().for_each(|testname| {
            test_runner.push_str(newline);
            push_python_line(
                &mut test_runner,
                1,
                ["print(\"", id.as_ref(), " ", testname, " RUNNING\")"],
            );
            push_python_line(&mut test_runner, 1, ["try:"]);
            push_python_line(&mut test_runner, 2, [testname, "()"]);
            push_python_line(&mut test_runner, 1, ["except Exception:"]);
            push_python_line(
                &mut test_runner,
                2,
                ["TracebackException.from_exception(sys.exception(), capture_locals=True).print(file=sys.stdout)"],
            );
            push_python_line(
                &mut test_runner,
                2,
                ["print(\"", id.as_ref(), " ", testname, " FAIL\")"],
            );
            push_python_line(&mut test_runner, 1, ["else:"]);
            push_python_line(
                &mut test_runner,
                2,
                ["print(\"", id.as_ref(), " ", testname, " PASS\")"],
            );
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

impl Pytest {
    pub fn failure_report(&self) -> Option<String> {
        match self.status {
            TestStatus::Fail(_, _) => Some("failed".to_string()),
            _ => None,
        }
    }
}

fn push_python_line<'strs, StrIter>(dst: &mut String, indents: usize, contents: StrIter)
where
    StrIter: IntoIterator<Item = &'strs str>,
{
    let indent = "    ";
    let newline = "\n";
    for _ in 0..indents {
        dst.push_str(indent);
    }
    for text in contents {
        dst.push_str(text);
    }
    dst.push_str(newline);
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
