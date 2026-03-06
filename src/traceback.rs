use base_traits::AsStr;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Traceback {
    text: String,
}

impl From<String> for Traceback {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl Traceback {
    pub(crate) fn lines(&'_ self) -> impl Iterator<Item = TbLine<'_>> {
        self.text.lines().map(TbLine::from)
    }
}

#[derive(Debug)]
pub(crate) enum TbLine<'line> {
    TracebackHeader,
    FrameHeader(FrameHeader<'line>),
    FrameContents { text: &'line str },
    Exception(PyError),
}

impl<'line> From<&'line str> for TbLine<'line> {
    fn from(line: &'line str) -> Self {
        match line.split_whitespace().next() {
            Some("Traceback") => Self::TracebackHeader,
            Some("File") => Self::FrameHeader(line.into()),
            _ => {
                if line.starts_with("    ") {
                    Self::FrameContents { text: line }
                } else {
                    Self::Exception(PyError::from(line))
                }
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct FrameHeader<'line> {
    #[expect(unused)]
    file_name: &'line str,
    pub(crate) function_name: &'line str,
    pub(crate) line_number: &'line str,
}

impl<'line> From<&'line str> for FrameHeader<'line> {
    fn from(line: &'line str) -> Self {
        let mut words = line.split_whitespace();
        let file_name = words.nth(1).unwrap();
        let line_number = words.nth(1).unwrap().trim_end_matches(",");
        let function_name = words.nth(1).unwrap();
        Self {
            file_name,
            function_name,
            line_number,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PyError {
    AssertionError,
    Other,
}

impl AsStr for PyError {
    fn as_str(&self) -> &str {
        match self {
            Self::AssertionError => "AssertionError",
            Self::Other => todo!(),
        }
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
