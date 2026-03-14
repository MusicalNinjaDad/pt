//! Parsing and storing the output from failed tests

use base_traits::AsStr;
use std::str::FromStr;

use crate::Error;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Traceback {
    text: String,
}

impl From<String> for Traceback {
    fn from(text: String) -> Self {
        Self { text }
    }
}

impl From<&str> for Traceback {
    fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
        }
    }
}

impl Traceback {
    pub(crate) fn lines(&'_ self) -> impl Iterator<Item = Result<TracebackLine<'_>, Error>> {
        self.text.lines().map(TracebackLine::try_from)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum TracebackLine<'line> {
    TracebackHeader,
    FrameHeader(FrameHeader<'line>),
    FrameContents { text: &'line str },
    Exception(Exception),
}

impl<'line> TryFrom<&'line str> for TracebackLine<'line> {
    type Error = Error;

    fn try_from(line: &'line str) -> Result<Self, Self::Error> {
        match line.split_whitespace().next() {
            Some("Traceback") => Ok(Self::TracebackHeader),
            Some("File") => Ok(Self::FrameHeader(line.try_into()?)),
            _ => {
                if line.starts_with("    ") {
                    Ok(Self::FrameContents { text: line })
                } else {
                    Ok(Self::Exception(Exception::try_from(line)?))
                }
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct FrameHeader<'line> {
    file_name: &'line str,
    pub(crate) function_name: &'line str,
    pub(crate) line_number: usize,
}

impl<'line> TryFrom<&'line str> for FrameHeader<'line> {
    type Error = Error;

    fn try_from(line: &'line str) -> Result<FrameHeader<'line>, Error> {
        let header: Option<_> = try {
            let mut words = line.split_whitespace();
            let file_name = words.nth(1)?;
            let line_number = usize::from_str(words.nth(1)?.trim_end_matches(",")).ok()?;
            let function_name = words.nth(1)?;
            FrameHeader {
                file_name,
                function_name,
                line_number,
            }
        };
        header.ok_or(Error::InvalidTraceback(line.to_string()))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Exception {
    AssertionError,
    Other,
}

impl AsStr for Exception {
    fn as_str(&self) -> &str {
        match self {
            Self::AssertionError => "AssertionError",
            Self::Other => todo!(),
        }
    }
}

impl TryFrom<&str> for Exception {
    type Error = Error;

    fn try_from(traceback: &str) -> Result<Exception, Error> {
        let exception: Option<_> = try {
            let lastline = traceback.lines().last()?;
            match lastline {
                "AssertionError" => Self::AssertionError,
                _ => Self::Other,
            }
        };
        exception.ok_or(Error::InvalidTraceback(traceback.to_string()))
    }
}
