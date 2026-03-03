use std::str::Lines;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Traceback {
   pub (crate) text: String,
}

impl From<String> for Traceback {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl Traceback {
    pub (crate) fn lines(&self) -> Lines {
        self.text.lines()
    }
}

#[derive(Debug, Default)]
pub (crate) enum TbParseStatus {
    InFrame {
        indent: usize,
        first_line: bool,
    },
    #[default]
    NotInFrame,
}

#[derive(Debug)]
pub (crate) enum TbLine<'line> {
    TracebackHeader,
    FrameHeader(FrameHeader<'line>),
    FrameContents,
    Exception,
}

#[derive(Debug)]
pub(crate) struct FrameHeader<'line> {
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

impl<'line> From<&'line str> for TbLine<'line> {
    fn from(line: &'line str) -> Self {
        match line.split_whitespace().next() {
            Some("Traceback") => Self::TracebackHeader,
            Some("File") => Self::FrameHeader(line.into()),
            _ => {
                if line.starts_with("    ") {
                    Self::FrameContents
                } else {
                    Self::Exception
                }
            }
        }
    }
}