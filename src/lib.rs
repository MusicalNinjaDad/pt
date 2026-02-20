use indexmap::IndexMap;
use ruff_python_ast::{Stmt, StmtFunctionDef};
use ruff_python_parser::{ParseError, parse_module};

#[derive(Debug, PartialEq)]
struct TestSuite {
    tests: IndexMap<String, Pytest>,
}

#[derive(Debug, PartialEq)]
struct Pytest {
    code: StmtFunctionDef,
    status: TestStatus,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum TestStatus {
    #[default]
    NoRun,
    Pass,
    Fail(Traceback),
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Traceback {
    text: String,
}

#[derive(Debug, PartialEq)]
struct TestOutput<'a> {
    id: &'a str,
    contents: &'a str,
}

impl From<TestOutput<'_>> for TestStatus {
    fn from(output: TestOutput) -> Self {
        if output.contents.starts_with(output.id) {
            return Self::Pass;
        }
        Self::Fail(Traceback {
            text: output.contents.to_string(),
        })
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

impl FromIterator<Stmt> for TestSuite {
    fn from_iter<T: IntoIterator<Item = Stmt>>(iter: T) -> Self {
        Self {
            tests: iter
                .into_iter()
                .filter_map(|stmt| -> Option<(String, Pytest)> {
                    match stmt {
                        Stmt::FunctionDef(function)
                            if function.name.as_str().starts_with("test_") =>
                        {
                            Some((function.name.to_string(), function.into()))
                        }
                        _ => None,
                    }
                })
                .collect(),
        }
    }
}

fn get_tests(src: &str) -> Result<TestSuite, ParseError> {
    let stmts = parse_module(src)?.into_suite();
    let pytests: TestSuite = stmts.into_iter().collect();
    Ok(pytests)
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

fn gen_runner<ID: AsRef<str>>(pytests: &TestSuite, id: ID) -> String {
    let indent = "    ";
    let newline = "\n";
    let mut test_runner: String = "if __name__ == \"__main__\":".to_string() + newline;
    test_runner += indent;
    test_runner += "import traceback";
    test_runner += newline;
    pytests.tests.keys().for_each(|testname| {
        test_runner.push_str(newline);
        push_python_line(
            &mut test_runner,
            1,
            ["print(\"", id.as_ref(), " running ", testname, "\")"],
        );
        push_python_line(&mut test_runner, 1, ["try:"]);
        push_python_line(&mut test_runner, 2, [testname, "()"]);
        push_python_line(&mut test_runner, 1, ["except Exception:"]);
        push_python_line(&mut test_runner, 2, ["traceback.print_exc()"]);
        push_python_line(&mut test_runner, 1, ["else:"]);
        push_python_line(&mut test_runner, 2, ["print(\"", id.as_ref(), " pass\")"]);
    });
    test_runner
}

pub fn generate<ID: AsRef<str>>(src: String, id: ID) -> Result<String, ParseError> {
    let pytests = get_tests(&src)?;
    let runner = gen_runner(&pytests, id);
    Ok(src + "\n\n" + &runner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        let src = r"def test_passes():
    assert True
";
        let pytests = get_tests(src).unwrap();
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
        let pytests = get_tests(src).unwrap();
        assert_eq!(2, pytests.tests.len());
        assert!(pytests.tests.contains_key("test_fails"));
        assert!(pytests.tests.contains_key("test_passes"));
    }

    #[test]
    fn parse_test_failure() {
        let stdout = r#"Traceback (most recent call last):
  File "/workspaces/pt/tests/fixtures/test.py", line 17, in <module>
    test_fails()
    ~~~~~~~~~~^^
  File "/workspaces/pt/tests/fixtures/test.py", line 5, in test_fails
    assert False
           ^^^^^
AssertionError"#;
        let output = TestOutput {
            id: "UID",
            contents: stdout,
        };
        let status: TestStatus = output.into();
        let expected_traceback = Traceback {
            text: stdout.to_string(),
        };
        assert_eq!(status, TestStatus::Fail(expected_traceback));
    }

    #[test]
    fn parse_test_success() {
        let output = TestOutput {
            id: "UID",
            contents: "UID pass",
        };
        let status: TestStatus = output.into();
        assert!(matches!(status, TestStatus::Pass))
    }
}
