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

fn gen_runner<ID: AsRef<str>>(pytests: &[Stmt], id: ID) -> String {
    let indent = "    ";
    let newline = "\n";
    let mut test_runner: String = "if __name__ == \"__main__\":".to_string() + newline;
    test_runner += indent;
    test_runner += "import traceback";
    test_runner += newline;
    pytests.iter().for_each(|pytest| {
        let testname = pytest.as_function_def_stmt().unwrap().name.as_str();
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
        assert_eq!(1, pytests.len());
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
        assert_eq!(2, pytests.len());
    }
}
