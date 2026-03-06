//!Handling individual tests & their status

use base_traits::AsStr;
use ruff_python_ast::StmtFunctionDef;

use crate::{PyError, Traceback};

/// A single test. Does not store the original source text, only the AST.
/// Construct with `Pytest::from(ruff_python_ast::StmtFunctionDef)`
#[derive(Debug, PartialEq)]
pub struct Pytest {
    pub ast: StmtFunctionDef,
    pub status: TestStatus,
}

impl From<StmtFunctionDef> for Pytest {
    //TODO: convert to TryFrom and include logic to validate whether this is a test function.
    fn from(fndef: StmtFunctionDef) -> Self {
        Self {
            ast: fndef,
            status: Default::default(),
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
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
