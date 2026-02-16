pub fn identify(_src: &str) -> &str{
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_test() {
        let src = "
        def test_passes():
            assert True
        ";
        assert_eq!("test_passes", identify(src));
    }
}
