use ruff_python_parser::{ParseError, parse_module};

pub fn identify(src: &str) -> Result<String, ParseError> {
    let stmts = parse_module(src)?.into_suite();
    let testfn = stmts
        .iter()
        .find(|stmt| stmt.is_function_def_stmt())
        .unwrap();
    let testname: String = testfn.as_function_def_stmt().unwrap().name.to_string();
    Ok(testname)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        let src = r"
def test_passes():
    assert True
";
        assert_eq!("test_passes", identify(src).unwrap());
    }

    #[test]
    fn test_not_first() {
        let src = r"
import foo

def test_passes():
    assert True
";
        assert_eq!("test_passes", identify(src).unwrap());
    }
}
