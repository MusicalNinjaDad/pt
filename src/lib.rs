use ruff_python_ast::Stmt;
use ruff_python_parser::{ParseError, parse_module};

fn get_tests(src: &str) -> Result<Vec<Stmt>, ParseError> {
    let stmts = parse_module(src)?.into_suite();
    let pytests = stmts
        .into_iter()
        .filter(|stmt| {
            stmt.is_function_def_stmt()
                && stmt
                    .as_function_def_stmt()
                    .unwrap()
                    .name
                    .as_str()
                    .starts_with("test_")
        })
        .collect();
    Ok(pytests)
}

fn gen_runner(pytests: &[Stmt]) -> String {
    let indent = "    ";
    let newline = "\n";
    let mut test_runner: String = "if __name__ == '__main__':".to_string() + newline;
    pytests.iter().for_each(|pytest| {
        test_runner += indent;
        test_runner += pytest.as_function_def_stmt().unwrap().name.as_str();
        test_runner += "()";
        test_runner += newline;
    });
    test_runner
}

pub fn generate(src: String) -> Result<String, ParseError> {
    let pytests = get_tests(&src)?;
    let runner = gen_runner(&pytests);
    Ok(src + "\n" + &runner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_tests_and_generate_runner() {
        let src = r"import pathlib

def test_fails():
    assert False

def test_passes():
    assert True
";
        let pytests = get_tests(src).unwrap();
        assert_eq!(2, pytests.len());
    }
}
